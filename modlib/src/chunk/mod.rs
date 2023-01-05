pub mod meshing;
pub mod bundle;
pub mod registry;

use bevy::prelude::Component;
use ndarray::{Array3, ArrayView3};
use crate::block::Block;
use self::registry::ChunkCoordinate;

pub const CHUNK_SIZE: usize = 16;

#[derive(Component)]
pub struct Chunk {
    position: ChunkCoordinate,
    array: Array3<Block>,
}

impl Chunk {
    pub fn new(at_coordinates: ChunkCoordinate) -> Self {
        Self { position: at_coordinates, array: Array3::<Block>::default((CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE)) }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &Block {
        self.array.get((x, y, z)).expect("Tried to access out of bounds index in chunk")
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, to: Block) {
        // there's probably a set method or something but I can't find it
        *self.array.get_mut((x, y, z)).expect("Tried to access out of bounds index in chunk") = to;
    }

    pub fn array_view(&self) -> ArrayView3<Block> {
        self.array.view()
    }

    pub fn get_position(&self) -> ChunkCoordinate {
        self.position
    }
}