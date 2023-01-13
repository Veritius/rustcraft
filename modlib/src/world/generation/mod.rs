use bevy::prelude::*;
use noise::{Perlin, NoiseFn};
use super::{
    chunk::{
        CHUNK_SIZE,
        CHUNK_SIZE_F32,
        CHUNK_SIZE_I32,
        loader::LoadChunkMessage,
        registry::ChunkRegistry,
        bundle::ChunkBundle,
        meshing::RemeshChunkMarker,
        Chunk,
    },
    block::{
        Block,
        BlockId
    }
};

const WORLD_GEN_SEED: u32 = 241231;
const PERLIN_MODIFIER: f32 = 0.026639428;

pub struct WorldGenPlugin;
impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(chunk_gen_system);
    }
}

fn chunk_gen_system(
    mut commands: Commands,
    mut gen_events: EventReader<LoadChunkMessage>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    for event in gen_events.iter() {
        let mut chunk = Chunk::new(event.0.into());
        let perlin = Perlin::new(WORLD_GEN_SEED);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut pick_coords = Vec3 {
                        x: x as f32 + (event.0.x * CHUNK_SIZE_I32) as f32,
                        y: y as f32 + (event.0.y * CHUNK_SIZE_I32) as f32,
                        z: z as f32 + (event.0.z * CHUNK_SIZE_I32) as f32,
                    };
                    pick_coords *= Vec3::splat(PERLIN_MODIFIER);

                    let value = perlin.get([pick_coords.x as f64, pick_coords.y as f64, pick_coords.z as f64]);
                    let block = if value >= 0.5 { Block::Generic(BlockId(1)) } else { Block::Empty };
                    chunk.set_block(x, y, z, block);
                }
            }
        }

        let mut pbr = PbrBundle::default();
        pbr.transform.translation = Vec3 {
            x: CHUNK_SIZE_F32 * event.0.x as f32,
            y: CHUNK_SIZE_F32 * event.0.y as f32,
            z: CHUNK_SIZE_F32 * event.0.z as f32,
        };

        let ent = commands.spawn(ChunkBundle { chunk, pbr }).insert(RemeshChunkMarker).id();
        chunk_registry.set_uncaring(event.0.into(), Some(ent));
    }
}