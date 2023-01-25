pub mod meshing;
pub mod bundle;
pub mod registry;
pub mod loader;
pub mod events;

use bevy::{prelude::{Component, SystemLabel, Entity, Plugin, IntoSystemDescriptor}, utils::HashMap};
use ndarray::Array3;
use self::{registry::{ChunkCoordinate, Chunks}, events::*, meshing::*};

use super::block::{BlockId, Block};

pub struct ChunkedWorldPlugin;
impl Plugin for ChunkedWorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Chunks::new());

        app.add_event::<UnloadChunkMessage>();
        app.add_event::<LoadChunkMessage>();
        app.add_event::<ChunkModifiedEvent>();

        app.add_system(chunk_change_system
            .label(SystemLabels::ChunkChangeEventSystem));
        app.add_system(chunk_remesh_dispatch_system
            .label(SystemLabels::ChunkMeshingDispatchSystem));
        app.add_system(chunk_remesh_polling_system
            .label(SystemLabels::ChunkMeshingPollingSystem)
            .after(SystemLabels::ChunkMeshingDispatchSystem));
        app.add_system(remesh_changed_chunks_system
            .after(SystemLabels::ChunkChangeEventSystem));
    }
}

#[derive(SystemLabel)]
pub enum SystemLabels {
    ChunkMeshingDispatchSystem,
    ChunkMeshingPollingSystem,
    ChunkChangeEventSystem,
}

// The size of each chunk in all axes, so a value of 16 would be 16x16x16.
// It'll probably work, but no guarantees - it's only tested with 16.
// The chunk size can only go up to 244 for technical reasons.
pub const CHUNK_SIZE: usize = 16;

// Derivative consts to make repeating `as f32` and such unnecessary.
pub const CHUNK_SIZE_U8: u8 = CHUNK_SIZE as u8;
pub const CHUNK_SIZE_U16: u16 = CHUNK_SIZE as u16;
pub const CHUNK_SIZE_U32: u32 = CHUNK_SIZE as u32;
pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

#[derive(Component)]
pub struct Chunk {
    position: ChunkCoordinate,
    array: Array3<ChunkBlockInternal>,
    entities: HashMap<u16, Entity>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ChunkBlockInternal {
    Generic(BlockId),
    Entity(u16),
}

impl ChunkBlockInternal {
    pub const EMPTY: ChunkBlockInternal = ChunkBlockInternal::Generic(BlockId::EMPTY);
}

impl Default for ChunkBlockInternal {
    fn default() -> Self {
        Self::Generic(BlockId(0))
    }
}

impl From<BlockId> for ChunkBlockInternal {
    fn from(value: BlockId) -> Self {
        Self::Generic(value)
    }
}

impl Chunk {
    pub fn new(at_coordinates: ChunkCoordinate) -> Self {
        Self {
            position: at_coordinates,
            array: Array3::from_elem([CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE], ChunkBlockInternal::EMPTY),
            entities: HashMap::new(),
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        match self.array[[x, y, z]] {
            ChunkBlockInternal::Generic(blockid) => Block::Generic(blockid),
            ChunkBlockInternal::Entity(idx) => Block::Entity(*self.entities.get(&idx).expect("Entity index should have been in the the map!")),
        }
    }

    pub fn get_generic_or_empty(&self, x: usize, y: usize, z: usize) -> BlockId {
        match self.array[[x, y, z]] {
            ChunkBlockInternal::Generic(blockid) => blockid,
            ChunkBlockInternal::Entity(_) => BlockId::EMPTY,
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, to: Block) {
        match to {
            Block::Generic(blockid) => {
                if let ChunkBlockInternal::Entity(idx) = self.array[[x, y, z]] {
                    self.entities.remove(&idx);
                }
                self.array[[x, y, z]] = ChunkBlockInternal::Generic(blockid)
            },
            Block::Entity(entity) => {
                for idx in 0..u16::MAX {
                    if self.entities.contains_key(&idx) { continue; }
                    self.entities.insert(idx, entity);
                    self.array[[x, y, z]] = ChunkBlockInternal::Entity(idx);
                }
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_internal_array(&self) -> &Array3<ChunkBlockInternal> {
        &self.array
    }

    pub fn get_position(&self) -> ChunkCoordinate {
        self.position
    }
}

trait GetBlockOrEmpty {
    fn get_block_or_empty(&self, x: usize, y: usize, z: usize) -> Block;
    fn get_generic_or_empty(&self, x: usize, y: usize, z: usize) -> BlockId;
}

impl GetBlockOrEmpty for Option<&Chunk> {
    fn get_block_or_empty(&self, x: usize, y: usize, z: usize) -> Block {
        match self {
            Some(chunk) => {
                chunk.get_block(x, y, z)
            },
            None => Block::EMPTY,
        }
    }

    fn get_generic_or_empty(&self, x: usize, y: usize, z: usize) -> BlockId {
        match self.get_block_or_empty(x, y, z) {
            Block::Generic(blockid) => blockid,
            Block::Entity(_) => BlockId::EMPTY,
        }
    }
}