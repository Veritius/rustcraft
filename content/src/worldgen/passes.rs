use bevy::{prelude::IVec3, math::DVec3};
use rustcraft_modlib::world::{
    generation::{
        generator::{
            WorldGeneratorPass,
            WorldGenerationMode,
        },
        noise::NoiseTableInternal,
    },
    chunk::{
        Chunk,
        CHUNK_SIZE, CHUNK_SIZE_I32,
    },
    block::{
        BlockId,
        Block,
    },
};
use super::noise::NOISE_LAYER_HEIGHT;

pub const WGEN_NORMAL_MODE: u32 = 2584328536;

#[derive(Clone)]
pub struct BaseTerrainPass;
impl WorldGeneratorPass for BaseTerrainPass {
    fn supports_mode(&self, mode: WorldGenerationMode) -> bool {
        match mode.0 {
            WGEN_NORMAL_MODE => true,
            _ => false,
        }
    }

    fn chunk_pass(&self, pos: IVec3, seed: u32, mode: WorldGenerationMode, noise: &NoiseTableInternal, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let dvec = DVec3 {
                x: ((pos.x * CHUNK_SIZE_I32) + x as i32) as f64,
                z: ((pos.y * CHUNK_SIZE_I32) + y as i32) as f64,
                y: ((pos.z * CHUNK_SIZE_I32) + z as i32) as f64,
            };
            
            let level = noise.get_value(NOISE_LAYER_HEIGHT, dvec).unwrap();
            if level > dvec.z {
                chunk.set_block(x, y, z, Block::Generic(BlockId(1)));
            }
        }}}
    }
}