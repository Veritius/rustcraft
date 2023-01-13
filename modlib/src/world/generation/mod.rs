use bevy::{prelude::*, tasks::{Task, AsyncComputeTaskPool}};
use futures_lite::future;
use noise::{Perlin, NoiseFn};
use super::{
    chunk::{
        CHUNK_SIZE,
        CHUNK_SIZE_F32,
        CHUNK_SIZE_I32,
        loader::LoadChunkMessage,
        registry::{ChunkRegistry, ChunkState},
        meshing::RemeshChunkMarker,
        Chunk,
    },
    block::{
        Block,
        BlockId
    }
};

#[derive(SystemLabel)]
pub enum SystemLabels {
    ChunkGenerationDispatchSystem,
    ChunkGenerationPollingSystem,
}

#[derive(Component)]
pub struct BeingGenerated(Task<Chunk>);

// TODO: Make this able to be changed
const WORLD_GEN_SEED: u32 = 53252345;
const PERLIN_MODIFIER: f32 = 0.026639428;

pub struct WorldGenPlugin;
impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generation_dispatch_system
            .label(SystemLabels::ChunkGenerationDispatchSystem));
        app.add_system(generation_polling_system
            .label(SystemLabels::ChunkGenerationPollingSystem)
            .after(SystemLabels::ChunkGenerationDispatchSystem));
    }
}

fn generation_dispatch_system(
    mut commands: Commands,
    mut gen_events: EventReader<LoadChunkMessage>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let task_pool = AsyncComputeTaskPool::get();
    for event in gen_events.iter() {
        let chunk_position = event.0.clone();
        let task: Task<Chunk> = task_pool.spawn(async move {
            let mut chunk = Chunk::new(chunk_position.into());
            let perlin = Perlin::new(WORLD_GEN_SEED);

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let mut pick_coords = Vec3 {
                            x: x as f32 + (chunk_position.x * CHUNK_SIZE_I32) as f32,
                            y: y as f32 + (chunk_position.y * CHUNK_SIZE_I32) as f32,
                            z: z as f32 + (chunk_position.z * CHUNK_SIZE_I32) as f32,
                        };
                        pick_coords *= Vec3::splat(PERLIN_MODIFIER);

                        let value = perlin.get([pick_coords.x as f64, pick_coords.y as f64, pick_coords.z as f64]);
                        let block = if value >= 0.5 { Block::Generic(BlockId(1)) } else { Block::empty() };
                        chunk.set_block(x, y, z, block);
                    }
                }
            }

            chunk
        });

        let mut pbr = PbrBundle::default();
        pbr.transform.translation = Vec3 {
            x: CHUNK_SIZE_F32 * event.0.x as f32,
            y: CHUNK_SIZE_F32 * event.0.y as f32,
            z: CHUNK_SIZE_F32 * event.0.z as f32,
        };

        commands.spawn((pbr, BeingGenerated(task)));
        chunk_registry.set(event.0.into(), ChunkState::BeingGenerated);
    }
}

fn generation_polling_system(
    mut commands: Commands,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut query: Query<(Entity, &mut BeingGenerated)>
) {
    for (entity, mut chunk) in query.iter_mut() {
        if let Some(chunk) = future::block_on(future::poll_once(&mut chunk.0)) {
            chunk_registry.set(chunk.get_position(), ChunkState::Present(entity));
            commands.entity(entity).remove::<BeingGenerated>().insert(chunk).insert(RemeshChunkMarker);
        }
    }
}