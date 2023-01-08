pub mod meshing;
pub mod bundle;
pub mod registry;

use bevy::prelude::Component;
use crate::world::block::Block;
use self::registry::ChunkCoordinate;

// It's not recommended you change this value. It'll probably work, but no guarantees - it's only tested with 16.
pub const CHUNK_SIZE: usize = 16;

#[derive(Component)]
pub struct Chunk {
    position: ChunkCoordinate,
    array: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new(at_coordinates: ChunkCoordinate) -> Self {
        Self { position: at_coordinates, array: [[[Block::Empty; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] }
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