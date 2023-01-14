use std::{collections::BTreeMap, ops::Deref, task::Poll};

use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, tasks::{AsyncComputeTaskPool, Task}};
use futures_lite::{FutureExt, future};
use crate::world::{block::{registry::BlockRegistry, entity::BlockEntity, BlockId, traits::BlockDefinition, Block}, WorldMapHelpers, chunk::{CHUNK_SIZE, CHUNK_SIZE_U8, GetBlockOrEmpty, CHUNK_SIZE_U16, CHUNK_SIZE_U32, meshing::meshers::greedy_mesh}};
use super::{registry::ChunkRegistry, Chunk, CHUNK_SIZE_I32};

mod meshers;

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

const SHAPE_SIZE_USIZE: usize = CHUNK_SIZE + 2;
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
            
                greedy_mesh(&intermediate_array, &registry);

                let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
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