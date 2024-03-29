use self::{
    biome::{
        scorer::BiomeSelectionScorer,
        registry::{BiomeData, BiomesInternal, Biomes}, BiomeId,
    },
    generator::{WorldGeneratorPass, WorldGeneration, WORLD_GENERATION}, noise::NoiseLayer,
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
        registry::{Chunks, ChunkState},
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
        app.init_resource::<Biomes>();
        app.init_resource::<WorldGeneration>();

        app.add_startup_system(worldgen_setup_system);
        app.add_system(generation_dispatch_system
            .label(SystemLabels::ChunkGenerationDispatchSystem)
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
    mut chunk_registry: ResMut<Chunks>,
    chunk_mat: Res<ChunkMaterialHandle>,
) {
    let task_pool = AsyncComputeTaskPool::get();
    for event in gen_events.iter() {
        let chunk_position = event.0.clone();

        // Async task definition
        let task: Task<Chunk> = task_pool.spawn(async move {
            let mut chunk = Chunk::new(chunk_position.into());
            WORLD_GENERATION.read().unwrap().do_passes_on_chunk(chunk_position, &mut chunk);

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
    mut chunk_registry: ResMut<Chunks>,
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
    fn add_biome(&mut self, name: BiomeId, biome: BiomeData) -> &mut Self;
    fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) -> &mut Self;
    fn add_world_generator_pass(&mut self, pass: impl WorldGeneratorPass) -> &mut Self;
    fn add_noise_layer(&mut self, key: String, layer: impl NoiseLayer) -> &mut Self;
}

impl WorldGenExtensionFns for App {
    /// Adds a new biome type. Shorthand for
    /// ```rs
    /// BIOME_REGISTRY.write().unwrap().add_biome()
    /// ```
    fn add_biome(&mut self, name: BiomeId, biome: BiomeData) -> &mut Self {
        self.add_startup_system(move |biomes: Res<Biomes>| {
            biomes.add_biome(name, biome.clone());
        });

        self
    }

    /// Adds a new `BiomeSelectionScorer` for biome selection. Shorthand for
    /// ```rs
    /// BIOME_REGISTRY.write().unwrap().add_biome_scorer()
    /// ```
    fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) -> &mut Self {
        self.add_startup_system(move |biomes: Res<Biomes>| {
            biomes.add_biome_scorer(dyn_clone::clone(&scorer));
        });

        self
    }

    /// Adds a new `WorldGeneratorPass` for chunk generation. Shorthand for
    /// ```rs
    /// WORLD_GENERATION.write().unwrap().add_world_generator_pass()
    /// ```
    fn add_world_generator_pass(&mut self, pass: impl WorldGeneratorPass) -> &mut Self {
        self.add_startup_system(move |world_generation: Res<WorldGeneration>| {
            world_generation.add_world_generator_pass(dyn_clone::clone(&pass));
        });

        self
    }

    /// Adds a new `NoiseLayer` to the chunk generation system. Shorthand for
    /// ```rs
    /// WORLD_GENERATION.write().unwrap().add_noise_layer()
    fn add_noise_layer(&mut self, key: String, layer: impl NoiseLayer) -> &mut Self {
        self.add_startup_system(move |world_generation: Res<WorldGeneration>| {
            world_generation.add_noise_layer(key.clone(), dyn_clone::clone(&layer));
        });

        self
    }
}
