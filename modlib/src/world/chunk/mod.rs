pub mod meshing;
pub mod bundle;
pub mod registry;
pub mod loader;

use bevy::prelude::{Component, SystemLabel};
use crate::world::block::Block;
use self::registry::ChunkCoordinate;

#[derive(SystemLabel)]
pub enum SystemLabels {
    ChunkMeshingDispatchSystem,
    ChunkMeshingPollingSystem,
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
    array: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new(at_coordinates: ChunkCoordinate) -> Self {
        Self { position: at_coordinates, array: [[[Block::empty(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &Block {
        &self.array[x][y][z]
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, to: Block) {
        // there's probably a set method or something but I can't find it
        self.array[x][y][z] = to;
    }

    pub fn get_array(&self) -> &[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] {
        &self.array
    }

    pub fn get_array_mut(&mut self) -> &mut [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] {
        &mut self.array
    }

    pub fn get_position(&self) -> ChunkCoordinate {
        self.position
    }
}

trait GetBlockOrEmpty {
    fn get_block_or_empty(&self, x: usize, y: usize, z: usize) -> Block;
}

impl GetBlockOrEmpty for Option<&Chunk> {
    fn get_block_or_empty(&self, x: usize, y: usize, z: usize) -> Block {
        match self {
            Some(chunk) => {
                *chunk.get_block(x, y, z)
            },
            None => Block::empty(),
        }
    }
}