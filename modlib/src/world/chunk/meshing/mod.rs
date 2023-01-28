use std::{collections::BTreeMap, ops::Deref, task::Poll, sync::{Arc, RwLock}};

use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::{Indices, MeshVertexAttribute, VertexAttributeValues, MeshVertexAttributeId}, once_cell::sync::Lazy}, tasks::{AsyncComputeTaskPool, Task}};
use dyn_clone::DynClone;
use futures_lite::{FutureExt, future};
use ndarray::Array3;
use crate::world::{block::{entity::BlockComponent, BlockId, Block, registry::Blocks}, WorldMapHelpers, chunk::{CHUNK_SIZE, CHUNK_SIZE_U8, GetBlockOrEmpty, CHUNK_SIZE_U16, CHUNK_SIZE_U32}};
use super::{registry::Chunks, Chunk, CHUNK_SIZE_I32, events::ChunkModifiedEvent};

pub mod greedy;
pub mod solid;
pub mod liquid;

pub static MESHING_PASSES: Lazy<Arc<RwLock<MeshingPassesInternal>>> = Lazy::new(||{Arc::new(RwLock::new(MeshingPassesInternal::new()))});

pub struct MeshingPassesInternal {
    passes: BTreeMap<MeshingPassIdentifier, Box<dyn MeshingPass>>,
}

impl MeshingPassesInternal {
    fn new() -> Self {
        Self {
            passes: BTreeMap::new(),
        }
    }

    pub fn add_pass(&mut self, name: MeshingPassIdentifier, pass: impl MeshingPass) {
        info!("Added meshing pass {}", name.name);
        self.passes.insert(name, Box::new(pass));
    }

    pub fn remove_pass(&mut self, name: MeshingPassIdentifier) {
        self.passes.remove(&name);
    }

    fn do_passes(&self, positions: &mut Vec<[f32;3]>, normals: &mut Vec<[f32;3]>, uvs: &mut Vec<[f32;2]>, colors: &mut Vec<[f32;4]>, data: &Array3<BlockId>) {
        for pass in self.passes.values() {
            pass.do_pass(positions, normals, uvs, colors, data);
        }
    }
}

pub struct MeshingPassIdentifier {
    name: &'static str,
    id: u32,
}

impl MeshingPassIdentifier {
    pub const fn new(name: &'static str, id: u32) -> Self {
        Self { name, id }
    }
}

impl PartialEq for MeshingPassIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MeshingPassIdentifier {}

impl PartialOrd for MeshingPassIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for MeshingPassIdentifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// A single 'pass' of the meshing system. Passes allow new cases for blocks to be specified, allowing the generation of new geometry.
/// Passes are not ordered and will be executed in the order they were inserted, which can be unpredictable.
pub trait MeshingPass: 'static + Send + Sync {
    // TODO: Add support for arbitrary attributes
    /// Does a pass over the chunk.
    /// 
    /// **Warning for implementors:** All vectors must be the same length!
    fn do_pass(&self, positions: &mut Vec<[f32;3]>, normals: &mut Vec<[f32;3]>, uvs: &mut Vec<[f32;2]>, colors: &mut Vec<[f32;4]>, data: &Array3<BlockId>);
}

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
            (MeshingVisibility::Translucent, MeshingVisibility::Invisible) => true,
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
    block_registry: Res<Blocks>,
    chunk_registry: Res<Chunks>,
    world_map: WorldMapHelpers,
    blocks: Query<(Entity, &BlockComponent)>,
    chunks: Query<(Entity, &Chunk, Option<&RemeshChunkMarker>), Without<BeingRemeshed>>,
) {
    let task_pool = AsyncComputeTaskPool::get();

    for (chunk_entityid, this_chunk, chunk_remesh_marker) in chunks.iter() {
        if let Some(_) = chunk_remesh_marker {
            let this_chunk_position = this_chunk.get_position();

            // let left_chunk = world_map.get_chunk_or_none((this_chunk_position.0 + 1, this_chunk_position.1, this_chunk_position.2)); // left
            // let right_chunk = world_map.get_chunk_or_none((this_chunk_position.0 - 1, this_chunk_position.1, this_chunk_position.2)); // right
            // let up_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1 + 1, this_chunk_position.2)); // up
            // let down_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1 - 1, this_chunk_position.2)); // down
            // let forward_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1, this_chunk_position.2 + 1)); // forward
            // let back_chunk = world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1, this_chunk_position.2 - 1)); // back
                
            let mut intermediate_array: Array3<BlockId> = Array3::from_elem((SHAPE_SIZE_USIZE, SHAPE_SIZE_USIZE, SHAPE_SIZE_USIZE), BlockId::EMPTY);

            // Main chunk
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        intermediate_array[[x+1, y+1, z+1]] = this_chunk.get_blockid_or_empty(&blocks, x, y, z);
                    }
                } 
            }

            // TODO: Fix random holes in geometry
            // Adjacent chunks are disabled to prevent the holes, but this creates a lot of redundant polygons.

            // Left and right chunks
            // for y in 0..CHUNK_SIZE {
            //     for z in 0..CHUNK_SIZE {
            //         intermediate_array[[0, y, z]] = left_chunk.get_generic_or_empty(15, y, z);
            //         intermediate_array[[17, y, z]] = right_chunk.get_generic_or_empty(0, y, z);
            //     }
            // }

            // // Above and below chunks
            // for x in 0..CHUNK_SIZE {
            //     for z in 0..CHUNK_SIZE {
            //         intermediate_array[[x, 0, z]] = up_chunk.get_generic_or_empty(x, 15, z);
            //         intermediate_array[[x, 17, z]] = down_chunk.get_generic_or_empty(x, 0, z);
            //     }
            // }

            // // Forward and back chunks
            // for x in 0..CHUNK_SIZE {
            //     for y in 0..CHUNK_SIZE {
            //         intermediate_array[[x, y, 0]] = forward_chunk.get_generic_or_empty(x, y, 15);
            //         intermediate_array[[x, y, 17]] = back_chunk.get_generic_or_empty(x, y, 0);
            //     }
            // }

            // Spawn task
            commands.entity(chunk_entityid).remove::<RemeshChunkMarker>().insert(BeingRemeshed(task_pool.spawn(async move {
                let mut positions: Vec<[f32; 3]> = vec![];
                let mut normals: Vec<[f32; 3]> = vec![];
                let mut uvs: Vec<[f32; 2]> = vec![];
                let mut colors: Vec<[f32; 4]> = vec![];

                MESHING_PASSES.read().unwrap().do_passes(&mut positions, &mut normals, &mut uvs, &mut colors, &intermediate_array);

                let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                render_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

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

pub(crate) fn remesh_changed_chunks_system(
    registry: Res<Chunks>,
    mut events: EventReader<ChunkModifiedEvent>,
    mut commands: Commands,
) {
    for event in events.iter() {
        for offset in [
            [1,0,0], [-1,0,0],
            [0,1,0], [0,-1,0],
            [0,0,1], [0,0,-1],
        ] {
            match registry.get((
                event.0.x + offset[0],
                event.0.y + offset[1],
                event.0.z + offset[2]
            )) {
                super::registry::ChunkState::Present(entity) => {
                    commands.entity(entity).insert(RemeshChunkMarker);
                },
                _ => {},
            }
        }
    }
}