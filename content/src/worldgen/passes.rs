use bevy::{prelude::IVec3, math::DVec3};
use rustcraft_modlib::world::{
    generation::{
        generator::{
            WorldGeneratorPass,
            WorldGenerationMode, WORLD_GENERATION, WorldGenerationInternal,
        },
    },
    chunk::{
        Chunk,
        CHUNK_SIZE, CHUNK_SIZE_I32,
    },
    block::{
        BlockId,
        Block, registry::{BLOCK_REGISTRY, BlockRegistryInternal},
    },
};
use super::noise::NOISE_LAYER_HEIGHT;

pub const WGEN_MODE_NORMAL: u32 = 2584328536;
pub const V_DIRT_DEPTH: f64 = 5.0;
pub const V_WATER_DEPTH: f64 = -6.0;

#[derive(Clone)]
pub struct BaseTerrainPass;
impl WorldGeneratorPass for BaseTerrainPass {
    fn ordering_value(&self) -> f64 { 1.0 }

    fn name(&self) -> &'static str { "rustcraft_terrain_base" }

    fn supports_mode(&self, mode: WorldGenerationMode) -> bool {
        match mode.0 {
            WGEN_MODE_NORMAL => true,
            _ => false,
        }
    }

    fn chunk_pass(&self, pos: IVec3, blocks: &BlockRegistryInternal, worldgen_data: &WorldGenerationInternal, chunk: &mut Chunk) {
        // Block types
        let water = Block::Generic(blocks.get_by_string_id("rustcraft_water").unwrap().0);
        let grass = Block::Generic(blocks.get_by_string_id("rustcraft_grass").unwrap().0);
        let dirt = Block::Generic(blocks.get_by_string_id("rustcraft_dirt").unwrap().0);
        let stone = Block::Generic(blocks.get_by_string_id("rustcraft_stone").unwrap().0);

        for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let dvec = DVec3 {
                x: ((pos.x * CHUNK_SIZE_I32) + x as i32) as f64,
                z: ((pos.y * CHUNK_SIZE_I32) + y as i32) as f64,
                y: ((pos.z * CHUNK_SIZE_I32) + z as i32) as f64,
            };
            
            let height = worldgen_data.get_noise_layer(NOISE_LAYER_HEIGHT).unwrap().get_value(dvec).round();
            let v_block_pos = dvec.z.round();

            // Water
            if v_block_pos < V_WATER_DEPTH && height < V_WATER_DEPTH {
                chunk.set_block(x, y, z, water);
            }

            // Grass
            if height == v_block_pos {
                if height > V_WATER_DEPTH - 2.0 {
                    chunk.set_block(x, y, z, grass);
                } else {
                    chunk.set_block(x, y, z, dirt);
                }
            }

            // Dirt
            if height > v_block_pos && v_block_pos >= height - V_DIRT_DEPTH {
                chunk.set_block(x, y, z, dirt);
            }

            // Stone
            if height - V_DIRT_DEPTH > v_block_pos {
                chunk.set_block(x, y, z, stone);
            }
        }}}
    }
}