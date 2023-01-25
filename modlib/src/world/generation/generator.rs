use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use dyn_clone::DynClone;
use crate::world::chunk::Chunk;

use super::noise::NoiseTableInternal;

#[derive(Clone)]
pub struct WorldGenPasses {
    passes: Vec<Box<dyn WorldGeneratorPass>>,
}

impl WorldGenPasses {
    pub(crate) fn new() -> Self {
        Self {
            passes: vec![],
        }
    }
    
    pub(crate) fn add_worldgen_pass(&mut self, pass: impl WorldGeneratorPass) {
        self.passes.push(Box::new(pass));
    }

    pub(crate) fn do_passes_on_chunk(&self, pos: IVec3, seed: u32, mode: WorldGenerationMode, noise: Arc<RwLock<NoiseTableInternal>>, chunk: &mut Chunk) {
        let noise = noise.read().unwrap();
        for pass in &self.passes {
            pass.chunk_pass(pos, seed, mode, &noise, chunk);
        }
    }
}

#[derive(Resource)]
pub(crate) struct WorldGenerationConfigStartupBuffer {
    passes: WorldGenPasses
}

impl WorldGenerationConfigStartupBuffer {
    pub(crate) fn new() -> Self {
        Self {
            passes: WorldGenPasses::new(),
        }
    }

    pub fn add_worldgen_pass(&mut self, pass: impl WorldGeneratorPass) {
        self.passes.add_worldgen_pass(pass);
    }
}

/// World generation options
#[derive(Resource)]
pub struct WorldGenerationConfig {
    pub seed: u32,
    pub mode: WorldGenerationMode,
    passes: Arc<WorldGenPasses>,
}

impl WorldGenerationConfig {
    pub(crate) fn new(seed: u32, vec: WorldGenPasses) -> Self {
        Self {
            seed,
            mode: WorldGenerationMode::NONE,
            passes: Arc::new(vec),
        }
    }

    pub fn get_passes_arc(&self) -> Arc<WorldGenPasses> {
        self.passes.clone()
    }

    pub(crate) fn do_passes_on_chunk(&self, pos: IVec3, noise: Arc<RwLock<NoiseTableInternal>>, chunk: &mut Chunk) {
        self.passes.do_passes_on_chunk(pos, self.seed, self.mode, noise, chunk);
    }
}

/// A _unique_ id for a world generation mode. Has an internal opaque value.
/// Use a random number generator to generate a unique unsigned 32-bit integer for your generation mode.
/// Default modes use a non-zero pattern, such as `NONE` being 0.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct WorldGenerationMode(u32);
impl WorldGenerationMode {
    pub const NONE: Self = Self(0);

    pub fn new(unique_id: u32) -> Self {
        Self(unique_id)
    }
}

dyn_clone::clone_trait_object!(WorldGeneratorPass);
pub trait WorldGeneratorPass: 'static + Send + Sync + DynClone {
    /// Checks if this generator pass supports a specific generation mode.
    fn supports_mode(&self, mode: WorldGenerationMode) -> bool;
    /// Does a pass over a given chunk.
    fn chunk_pass(&self, pos: IVec3, seed: u32, mode: WorldGenerationMode, noise: &NoiseTableInternal, chunk: &mut Chunk);
}

pub(crate) fn generation_config_buffer_transfer_system(
    mut commands: Commands,
    buffer: Res<WorldGenerationConfigStartupBuffer>,
) {
    commands.insert_resource(WorldGenerationConfig::new(0, buffer.passes.clone()));
    commands.remove_resource::<WorldGenerationConfigStartupBuffer>();
}