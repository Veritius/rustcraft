use std::{sync::{Arc, RwLock}, collections::BTreeSet, cmp::Ordering};
use bevy::{prelude::*, render::once_cell::sync::Lazy, utils::HashMap};
use dyn_clone::DynClone;
use crate::world::{chunk::Chunk, block::registry::{BlockRegistryInternal, BLOCK_REGISTRY}};
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
    passes: BTreeSet<WorldGenPassWrapper>,
    noise_layers: HashMap<String, Box<dyn NoiseLayer>>,
}

impl WorldGenerationInternal {
    fn new() -> Self {
        Self {
            seed: 0,
            gen_mode: WorldGenerationMode::NONE,
            passes: BTreeSet::new(),
            noise_layers: HashMap::new(),
        }
    }

    pub fn add_world_generator_pass(&mut self, pass: impl WorldGeneratorPass) {
        info!("Added world generator pass {} at order {}", pass.name(), pass.ordering_value());
        self.passes.insert(WorldGenPassWrapper(Box::new(pass)));
    }

    pub fn add_noise_layer(&mut self, name: String, layer: impl NoiseLayer) {
        self.noise_layers.insert(name, Box::new(layer));
    }

    pub fn do_passes_on_chunk(&self, pos: IVec3, chunk: &mut Chunk) {
        let blocks = BLOCK_REGISTRY.read().unwrap();
        for pass in &self.passes {
            pass.0.chunk_pass(pos, &blocks, &self, chunk);
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

/// World generator passes are used to procedurally generate the world. Passes are not aware of eachother.
/// 
/// Passes must be ordered correctly or issues may occur. The `f64` returned by `ordering_value` is used for ordering, based on the `Ord` implementation of [WorldGenPassWrapper].
/// Ordering will panic if the returned float is a NaN or other value that cannot be either greater, equal, or smaller than another.
pub trait WorldGeneratorPass: 'static + Send + Sync + DynClone {
    /// Used for ordering chunk passes.
    fn ordering_value(&self) -> f64;
    /// Developer-friendly display name for this object. Used for debugging.
    fn name(&self) -> &'static str;
    /// Checks if this generator pass supports a specific generation mode.
    fn supports_mode(&self, mode: WorldGenerationMode) -> bool;
    /// Does a pass over a given chunk.
    fn chunk_pass(&self, pos: IVec3, blocks: &BlockRegistryInternal, gen: &WorldGenerationInternal, chunk: &mut Chunk);
}
dyn_clone::clone_trait_object!(WorldGeneratorPass);

/// `Ord`-implementing wrapper for `WorldGeneratorPass` objects.
struct WorldGenPassWrapper(Box<dyn WorldGeneratorPass>);

impl PartialEq for WorldGenPassWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.ordering_value() == other.0.ordering_value()
    }
}

impl Eq for WorldGenPassWrapper {}

impl PartialOrd for WorldGenPassWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.ordering_value().partial_cmp(&other.0.ordering_value())
    }
}

// Total ordering for f64s. What could go wrong?
impl Ord for WorldGenPassWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        let this = self.0.ordering_value();
        let other = self.0.ordering_value();
        if this < other { return Ordering::Less }
        if this == other { return Ordering::Equal }
        if this > other { return Ordering::Greater }
        panic!("Invalid comparison while ordering world generation passes. Possible NaN passed? Self: {}, Other: {}", this, other);
    }
}