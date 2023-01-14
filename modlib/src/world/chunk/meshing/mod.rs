use std::{collections::BTreeMap, ops::Deref, task::Poll};

use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, tasks::{AsyncComputeTaskPool, Task}};
use block_mesh::{ndshape::{ConstShape3u8, ConstShape, ConstShape3u16, ConstShape3u32}, greedy_quads, RIGHT_HANDED_Y_UP_CONFIG, GreedyQuadsBuffer, Voxel, MergeVoxel, VoxelVisibility};
use futures_lite::{FutureExt, future};
use crate::world::{block::{registry::BlockRegistry, entity::BlockEntity, BlockId, traits::BlockDefinition, Block}, WorldMapHelpers, chunk::{CHUNK_SIZE, CHUNK_SIZE_U8, GetBlockOrEmpty, CHUNK_SIZE_U16, CHUNK_SIZE_U32}};
use super::{registry::ChunkRegistry, Chunk, CHUNK_SIZE_I32};

/// Used for generating a mesh for a chunk.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MeshingVisibility {
    /// Produces faces on all sides, and prevents faces being produced for other blocks.
    Opaque,
    /// Produces faces on all sides, but does not prevent faces being produced for other blocks.
    Translucent,
    /// Does not produce faces at all, and allows faces to be produced for other blocks.
    /// This may also be used for blocks that have their own meshes and should not be included in the chunk mesh generation, i.e. entities.
    Invisible,
}

impl From<MeshingVisibility> for VoxelVisibility {
    fn from(value: MeshingVisibility) -> Self {
        match value {
            MeshingVisibility::Opaque => VoxelVisibility::Opaque,
            MeshingVisibility::Translucent => VoxelVisibility::Translucent,
            MeshingVisibility::Invisible => VoxelVisibility::Empty,
        }
    }
}

impl MeshingVisibility {
    pub fn is_visible_against(&self, other: &MeshingVisibility) -> bool {
        match (self, other) {
            (MeshingVisibility::Invisible, _) => false,
            (MeshingVisibility::Opaque, MeshingVisibility::Opaque) => false,
            (MeshingVisibility::Opaque, MeshingVisibility::Translucent) => true,
            (MeshingVisibility::Opaque, MeshingVisibility::Invisible) => true,
            (MeshingVisibility::Translucent, MeshingVisibility::Opaque) => false,
            (MeshingVisibility::Translucent, MeshingVisibility::Translucent) => false,
            (MeshingVisibility::Translucent, MeshingVisibility::Invisible) => false,
        }
    }
}

/// Added to chunks to indicate the need to regenerate their mesh.
#[derive(Component)]
pub struct RemeshChunkMarker;

/// This chunk has an ongoing asynchronous task to generate its mesh.
#[derive(Component)]
pub struct BeingRemeshed(Task<Mesh>);

const UV_SCALE: f32 = 1.0 / CHUNK_SIZE as f32;

pub fn chunk_remesh_dispatch_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    block_registry: Res<BlockRegistry>,
    chunk_registry: Res<ChunkRegistry>,
    world_map: WorldMapHelpers,
    blocks: Query<(Entity, &BlockEntity)>,
    chunks: Query<(Entity, &Chunk, Option<&RemeshChunkMarker>), Without<BeingRemeshed>>,
) {
    let task_pool = AsyncComputeTaskPool::get();

    for (chunk_entityid, this_chunk, chunk_remesh_marker) in chunks.iter() {
        if let Some(_) = chunk_remesh_marker {
            let this_chunk_position = this_chunk.get_position();

            let left_chunk = world_map.get_chunk_or_none((this_chunk_position.0 + 1, this_chunk_position.1, this_chunk_position.2)); // left
            let right_chunk = world_map.get_chunk_or_none((this_chunk_position.0 - 1, this_chunk_position.1, this_chunk_position.2)); // right
            let up_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1 + 1, this_chunk_position.2)); // up
            let down_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1 - 1, this_chunk_position.2)); // down
            let forward_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1, this_chunk_position.2 + 1)); // forward
            let back_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1, this_chunk_position.2 - 1)); // back

            const SHAPE_SIZE_USIZE: usize = CHUNK_SIZE + 2;
                
            let mut intermediate_array = [[[BlockId(0); SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];

            // Main chunk
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        intermediate_array[x+1][y+1][z+1] = this_chunk.get_generic_or_empty(x, y, z);
                    }
                }
            }

            // Left and right chunks
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    intermediate_array[0][y][z] = left_chunk.get_generic_or_empty(0, y, z);
                    intermediate_array[17][y][z] = right_chunk.get_generic_or_empty(15, y, z);
                }
            }

            // Above and below chunks
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    intermediate_array[x][0][z] = up_chunk.get_generic_or_empty(x, 0, z);
                    intermediate_array[x][17][z] = down_chunk.get_generic_or_empty(x, 15, z);
                }
            }

            // Forward and back chunks
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    intermediate_array[x][y][0] = forward_chunk.get_generic_or_empty(x, y, 15);
                    intermediate_array[x][y][17] = back_chunk.get_generic_or_empty(x, y, 0);
                }
            }

            let registry = block_registry.clone();
            
            // Spawn task
            commands.entity(chunk_entityid).remove::<RemeshChunkMarker>().insert(BeingRemeshed(task_pool.spawn(async move {
                // TODO: Figure out a solution that doesn't involve cloning the entire block registry
                const SHAPE_SIZE_U32: u32 = CHUNK_SIZE_U32 + 2;
                type ChunkShape = ConstShape3u32<SHAPE_SIZE_U32, SHAPE_SIZE_U32, SHAPE_SIZE_U32>;

                #[derive(Clone, Copy)]
                struct VoxelCapsule<'r> {
                    id: BlockId,
                    reg: &'r BlockRegistry,
                }

                impl Voxel for VoxelCapsule<'_> {
                    fn get_visibility(&self) -> block_mesh::VoxelVisibility {
                        match self.reg.get_by_id(self.id) {
                            Some(res) => { res.visibility().into() },
                            None => { VoxelVisibility::Empty },
                        }
                    }
                }

                impl MergeVoxel for VoxelCapsule<'_> {
                    type MergeValue = BlockId;

                    fn merge_value(&self) -> Self::MergeValue {
                        self.id
                    }
                }
            
                let mut voxels = [VoxelCapsule { id: BlockId(65535), reg: &registry }; ChunkShape::SIZE as usize];
                for x in 0..SHAPE_SIZE_U32 {
                    for y in 0..SHAPE_SIZE_U32 {
                        for z in 0..SHAPE_SIZE_U32 {
                            let i = ChunkShape::linearize([x, y, z]);
                            let block_id = intermediate_array[x as usize][y as usize][z as usize];
                            voxels[i as usize] = VoxelCapsule { id: block_id, reg: &registry };
                        }
                    }
                }

                let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

                let mut buffer = GreedyQuadsBuffer::new(voxels.len());
                greedy_quads(
                    &voxels,
                    &ChunkShape {},
                    [0;3],
                    [17;3],
                    &faces,
                    &mut buffer,
                );
                
                let num_indices = buffer.quads.num_quads() * 6;
                let num_vertices = buffer.quads.num_quads() * 4;
                let mut indices = Vec::with_capacity(num_indices);
                let mut positions = Vec::with_capacity(num_vertices);
                let mut normals = Vec::with_capacity(num_vertices);
                let mut tex_coords = Vec::with_capacity(num_vertices);

                for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
                    for quad in group.into_iter() {
                        indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                        positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                        normals.extend_from_slice(&face.quad_mesh_normals());
                        tex_coords.extend_from_slice(&face.tex_coords(
                            RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                            true,
                            &quad,
                        ));
                    }
                }

                let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);

                for uv in tex_coords.iter_mut() {
                    for c in uv.iter_mut() {
                        *c *= UV_SCALE;
                    }
                }

                render_mesh.set_indices(Some(Indices::U32(indices)));
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);

                render_mesh
            })));
        }     
    }
}

pub fn chunk_remesh_polling_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &mut Handle<Mesh>, &mut BeingRemeshed)>
) {
    for (entity, mut handle, mut remesh) in query.iter_mut() {
        if let Some(mesh) = future::block_on(future::poll_once(&mut remesh.0)) {
            *handle = meshes.add(mesh);
            commands.entity(entity).remove::<BeingRemeshed>();
        }
    }
}