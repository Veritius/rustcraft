use std::sync::{Arc, RwLock};
use bevy::{prelude::*, render::once_cell::sync::Lazy, utils::HashMap};
use dyn_clone::DynClone;
use crate::world::chunk::Chunk;
use super::noise::NoiseLayer;

pub static WORLD_GENERATION: Lazy<Arc<RwLock<WorldGenerationInternal>>> = Lazy::new(||{Arc::new(RwLock::new(WorldGenerationInternal::new()))});

#[derive(Resource)]
pub struct WorldGeneration(Arc<RwLock<WorldGenerationInternal>>);

impl WorldGeneration {
    pub fn add_world_generator_pass(&self, pass: impl WorldGeneratorPass) {
        self.0.write().unwrap().add_world_generator_pass(pass);
    }

    pub fn add_noise_layer(&self, name: String, layer: impl NoiseLayer) {
        self.0.write().unwrap().add_noise_layer(name, layer);
    }

    pub fn do_passes_on_chunk(&self, pos: IVec3, chunk: &mut Chunk) {
        self.0.read().unwrap().do_passes_on_chunk(pos, chunk);
    }
}

impl Default for WorldGeneration {
    fn default() -> Self {
        Self(WORLD_GENERATION.clone())
    }
}

pub struct WorldGenerationInternal {
    pub seed: u32,
    pub gen_mode: WorldGenerationMode,
    passes: Vec<Box<dyn WorldGeneratorPass>>,
    noise_layers: HashMap<String, Box<dyn NoiseLayer>>,
}

impl WorldGenerationInternal {
    fn new() -> Self {
        Self {
            seed: 0,
            gen_mode: WorldGenerationMode::NONE,
            passes: vec![],
            noise_layers: HashMap::new(),
        }
    }

    pub fn add_world_generator_pass(&mut self, pass: impl WorldGeneratorPass) {
        self.passes.push(Box::new(pass));
    }

    pub fn add_noise_layer(&mut self, name: String, layer: impl NoiseLayer) {
        self.noise_layers.insert(name, Box::new(layer));
    }

    pub fn do_passes_on_chunk(&self, pos: IVec3, chunk: &mut Chunk) {
        for pass in &self.passes {
            pass.chunk_pass(pos, chunk);
        }
    }

    pub fn get_noise_layer(&self, name: &str) -> Option<&Box<dyn NoiseLayer>> {
        self.noise_layers.get(name)
    }
}

/// A _unique_ id for a world generation mode. Has an internal opaque value.
/// Use a random number generator to generate a unique unsigned 32-bit integer for your generation mode.
/// Default modes use a non-zero pattern, such as `NONE` being 0.
/// 
/// **TODO:** Replace this with an attribute registry.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct WorldGenerationMode(pub u32);
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
    fn chunk_pass(&self, pos: IVec3, chunk: &mut Chunk);
}