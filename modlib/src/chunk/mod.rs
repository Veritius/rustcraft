use bevy::prelude::Component;
use ndarray::Array3;
use crate::block::Block;

pub const CHUNK_SIZE: (usize, usize, usize) = (16, 16, 16);

#[derive(Component)]
pub struct Chunk {
    array: Array3<Block>,
}

impl Chunk {
    fn new() -> Self {
        Self { array: Array3::<Block>::default(CHUNK_SIZE) }
    }

    fn get(&self, x: usize, y: usize, z: usize) -> &Block {
        self.array.get((x, y, z)).expect("Tried to access out of bounds index in chunk")
    }

    fn set(&mut self, x: usize, y: usize, z: usize, to: Block) {
        // there's probably a set method or something but I can't find it
        *self.array.get_mut((x, y, z)).expect("Tried to access out of bounds index in chunk") = to;
    }
}