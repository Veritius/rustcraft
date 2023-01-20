use bevy::{prelude::*, tasks::{Task, AsyncComputeTaskPool}, render::once_cell::sync::Lazy};
use futures_lite::future;
use noise::{Perlin, NoiseFn, Seedable};
use super::{
    chunk::{
        CHUNK_SIZE,
        CHUNK_SIZE_F32,
        CHUNK_SIZE_I32,
        events::LoadChunkMessage,
        registry::{ChunkRegistry, ChunkState},
        meshing::RemeshChunkMarker,
        Chunk,
    },
    block::{BlockId, Block, registry::BlockRegistry},
};

#[derive(SystemLabel)]
pub enum SystemLabels {
    ChunkGenerationDispatchSystem,
    ChunkGenerationPollingSystem,
}

#[derive(Component)]
pub struct BeingGenerated(Task<Chunk>);

pub struct WorldGenPlugin;
impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(worldgen_setup_system);
        app.add_system(generation_dispatch_system
            .label(SystemLabels::ChunkGenerationDispatchSystem));
        app.add_system(generation_polling_system
            .label(SystemLabels::ChunkGenerationPollingSystem)
            .after(SystemLabels::ChunkGenerationDispatchSystem));
    }
}

#[derive(Resource)]
struct ChunkMaterialHandle(Handle<StandardMaterial>);

fn worldgen_setup_system(
    mut commands: Commands,
    mut assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ChunkMaterialHandle(
        assets.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })
    ));
}

fn generation_dispatch_system(
    mut commands: Commands,
    mut gen_events: EventReader<LoadChunkMessage>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    chunk_mat: Res<ChunkMaterialHandle>,
) {
    let task_pool = AsyncComputeTaskPool::get();
    for event in gen_events.iter() {
        let chunk_position = event.0.clone();
        let task: Task<Chunk> = task_pool.spawn(async move {
            let mut chunk = Chunk::new(chunk_position.into());

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        chunk.set_block(x, y, z, generate_per_block(chunk_position, x, y, z));
                    }
                }
            }

            chunk
        });

        let mut pbr = PbrBundle::default();
        pbr.material = chunk_mat.0.clone();
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

static WGEN_SURFACE_NOISE_1: Lazy<Perlin> = Lazy::new(||{Perlin::new(52842)});
const WGEN_SURFACE_NOISE_1_MODIFIER: f64 = 0.016639428;
static WGEN_SURFACE_NOISE_2: Lazy<Perlin> = Lazy::new(||{Perlin::new(15834)});
const WGEN_SURFACE_NOISE_2_MODIFIER: f64 = 0.0093313213;
static WGEN_SURFACE_NOISE_3: Lazy<Perlin> = Lazy::new(||{Perlin::new(21722)});
const WGEN_SURFACE_NOISE_3_MODIFIER: f64 = 0.001219412;

// TODO: Make this not a monolith function and instead modular using Bevy
fn generate_per_block(chunk_position: IVec3, x: usize, y: usize, z: usize) -> Block {
    let block_coordinates = Vec3 {
        x: x as f32 + (chunk_position.x * CHUNK_SIZE_I32) as f32,
        y: y as f32 + (chunk_position.y * CHUNK_SIZE_I32) as f32,
        z: z as f32 + (chunk_position.z * CHUNK_SIZE_I32) as f32,
    };

    // Major
    let mut surface_level = 5.0 * WGEN_SURFACE_NOISE_1.get([
        block_coordinates.x as f64 * WGEN_SURFACE_NOISE_1_MODIFIER,
        block_coordinates.z as f64 * WGEN_SURFACE_NOISE_1_MODIFIER,
    ]);

    // Middle
    surface_level += 10.0 * WGEN_SURFACE_NOISE_2.get([
        block_coordinates.x as f64 * WGEN_SURFACE_NOISE_2_MODIFIER,
        block_coordinates.z as f64 * WGEN_SURFACE_NOISE_2_MODIFIER,
    ]);

    // Minor
    surface_level += 15.0 * WGEN_SURFACE_NOISE_3.get([
        block_coordinates.x as f64 * WGEN_SURFACE_NOISE_3_MODIFIER,
        block_coordinates.z as f64 * WGEN_SURFACE_NOISE_3_MODIFIER,
    ]);
    
    let block = if surface_level >= block_coordinates.y as f64 { Block::Generic(BlockId(1)) } else { Block::EMPTY };

    block
}