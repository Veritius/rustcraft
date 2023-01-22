use bevy::{prelude::IVec3, render::once_cell::sync::Lazy};
use noise::{Perlin, NoiseFn};
use crate::world::chunk::{Chunk, CHUNK_SIZE};

pub trait WorldGeneratorPass {
    fn chunk_pass(pos: IVec3, chunk: &mut Chunk);
}

struct BasicLayers;
impl WorldGeneratorPass for BasicLayers {
    fn chunk_pass(pos: IVec3, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {

                }
            }
        }
    }
}