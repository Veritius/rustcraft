use self::{
    biome::{
        scorer::BiomeSelectionScorer,
        registry::{BiomeData, BiomeRegistry, BiomeRegistryStartupBuffer, biome_buffer_transfer_system},
    },
    generator::{WorldGenerationConfig, WorldGeneratorPass, WorldGenerationConfigStartupBuffer, generation_config_buffer_transfer_system}, noise::{NoiseTable, NoiseLayer},
};
use bevy::{
    prelude::*,
    render::once_cell::sync::Lazy,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use std::{
    collections::BTreeMap,
    ops::{Deref, Range},
};

use super::{
    block::{Block, BlockId},
    chunk::{
        events::LoadChunkMessage,
        meshing::RemeshChunkMarker,
        registry::{ChunkRegistry, ChunkState},
        Chunk, CHUNK_SIZE, CHUNK_SIZE_F32, CHUNK_SIZE_I32,
    },
};

pub mod biome;
pub mod generator;
pub mod noise;

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
        app.insert_resource(BiomeRegistryStartupBuffer::new());
        app.add_startup_system_to_stage(StartupStage::PostStartup,
            biome_buffer_transfer_system);
        app.insert_resource(WorldGenerationConfigStartupBuffer::new());
        app.add_startup_system_to_stage(StartupStage::PostStartup,
            generation_config_buffer_transfer_system);
        app.insert_resource(NoiseTable::new());

        app.add_startup_system(worldgen_setup_system);
        app.add_system(
            generation_dispatch_system.label(SystemLabels::ChunkGenerationDispatchSystem),
        );
        app.add_system(
            generation_polling_system
                .label(SystemLabels::ChunkGenerationPollingSystem)
                .after(SystemLabels::ChunkGenerationDispatchSystem),
        );
    }
}

#[derive(Resource)]
struct ChunkMaterialHandle(Handle<StandardMaterial>);

fn worldgen_setup_system(mut commands: Commands, mut assets: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(ChunkMaterialHandle(assets.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    })));
}

fn generation_dispatch_system(
    mut commands: Commands,
    mut gen_events: EventReader<LoadChunkMessage>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    biome_registry: Res<BiomeRegistry>,
    world_gen_config: Res<WorldGenerationConfig>,
    chunk_mat: Res<ChunkMaterialHandle>,
) {
    let task_pool = AsyncComputeTaskPool::get();
    for event in gen_events.iter() {
        let chunk_position = event.0.clone();
        // let biome_registry_arc = biome_registry.get_internal_registry();
        let generation_seed = world_gen_config.seed;
        let generation_mode = world_gen_config.mode;
        let world_gen_config_arc = world_gen_config.get_passes_arc();

        // Async task definition
        let task: Task<Chunk> = task_pool.spawn(async move {
            let mut chunk = Chunk::new(chunk_position.into());
            // let biome = biome_registry_arc.calculate_biome_for_chunk(chunk_position);
            world_gen_config_arc.do_passes_on_chunk(chunk_position, generation_seed, generation_mode, &mut chunk);

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
    mut query: Query<(Entity, &mut BeingGenerated)>,
) {
    for (entity, mut chunk) in query.iter_mut() {
        if let Some(chunk) = future::block_on(future::poll_once(&mut chunk.0)) {
            chunk_registry.set(chunk.get_position(), ChunkState::Present(entity));
            commands
                .entity(entity)
                .remove::<BeingGenerated>()
                .insert(chunk)
                .insert(RemeshChunkMarker);
        }
    }
}

pub trait WorldGenExtensionFns {
    fn add_biome(&mut self, biome: BiomeData) -> &mut Self;
    fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) -> &mut Self;
    fn add_world_generator_pass(&mut self, scorer: impl WorldGeneratorPass) -> &mut Self;
    fn add_noise_layer<T: NoiseLayer>(&mut self, key: String) -> &mut Self;
}

impl WorldGenExtensionFns for App {
    /// Adds a new biome type
    fn add_biome(&mut self, biome: BiomeData) -> &mut Self {
        self.add_startup_system(move |mut biome_table: ResMut<BiomeRegistryStartupBuffer>| {
            biome_table.add_biome_type(biome.clone());
        });

        self
    }

    /// Adds a new `BiomeSelectionScorer` to the game
    fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) -> &mut Self {
        self.add_startup_system(move |mut biome_table: ResMut<BiomeRegistryStartupBuffer>| {
            biome_table.add_biome_scorer(dyn_clone::clone(&scorer));
        });

        self
    }

    /// Adds a new `WorldGeneratorPass` to the chunk generation system
    fn add_world_generator_pass(&mut self, gen_pass: impl WorldGeneratorPass) -> &mut Self {
        self.add_startup_system(move |mut generation_config: ResMut<WorldGenerationConfigStartupBuffer>| {
            generation_config.add_worldgen_pass(dyn_clone::clone(&gen_pass));
        });

        self
    }

    /// Adds a new `NoiseLayer` to the chunk generation system
    fn add_noise_layer<T: NoiseLayer>(&mut self, key: String) -> &mut Self {
        self.add_startup_system(move |mut layer_table: ResMut<NoiseTable>| {
            layer_table.add_layer::<T>(key.clone());
        });

        self
    }
}
