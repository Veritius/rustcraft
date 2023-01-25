use bevy::{prelude::IVec3, math::DVec3};
use rustcraft_modlib::world::{
    generation::{
        generator::{
            WorldGeneratorPass,
            WorldGenerationMode, WORLD_GENERATION,
        },
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

pub const WGEN_MODE_NORMAL: u32 = 2584328536;
pub const V_DIRT_DEPTH: f64 = 5.0;

#[derive(Clone)]
pub struct BaseTerrainPass;
impl WorldGeneratorPass for BaseTerrainPass {
    fn supports_mode(&self, mode: WorldGenerationMode) -> bool {
        match mode.0 {
            WGEN_MODE_NORMAL => true,
            _ => false,
        }
    }

    fn chunk_pass(&self, pos: IVec3, chunk: &mut Chunk) {
        let worldgen_data = WORLD_GENERATION.read().unwrap();
        for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let dvec = DVec3 {
                x: ((pos.x * CHUNK_SIZE_I32) + x as i32) as f64,
                z: ((pos.y * CHUNK_SIZE_I32) + y as i32) as f64,
                y: ((pos.z * CHUNK_SIZE_I32) + z as i32) as f64,
            };
            
            let level = worldgen_data.get_noise_layer(NOISE_LAYER_HEIGHT).unwrap().get_value(dvec);
            let height = level.round();
            let v_block_pos = dvec.z.round();
            if height == v_block_pos {
                chunk.set_block(x, y, z, Block::Generic(BlockId(2)));
            }
            if height > v_block_pos && v_block_pos >= height - V_DIRT_DEPTH {
                chunk.set_block(x, y, z, Block::Generic(BlockId(1)));
            }
            if height - V_DIRT_DEPTH > v_block_pos {
                chunk.set_block(x, y, z, Block::Generic(BlockId(3)));
            }
        }}}
    }
}