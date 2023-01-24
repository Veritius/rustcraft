use bevy::prelude::{IVec3, Resource};
use dyn_clone::DynClone;
use crate::world::chunk::Chunk;

#[derive(Resource)]
pub struct WorldGenerationConfig {
    seed: u32,
    mode: WorldGenerationMode,
    passes: Vec<Box<dyn WorldGeneratorPass>>,
}

impl WorldGenerationConfig {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            mode: WorldGenerationMode::NONE,
            passes: vec![],
        }
    }

    // TODO: Make it so order can be specified (not doing stone before ore, etc)
    pub fn add_worldgen_pass(&mut self, pass: impl WorldGeneratorPass) {
        self.passes.push(Box::new(pass))
    }

    pub fn set_worldgen_mode(&mut self, mode: WorldGenerationMode) {
        self.mode = mode
    }

    pub(crate) fn do_passes_on_chunk(&self, pos: IVec3, chunk: &mut Chunk) {
        for pass in &self.passes {
            pass.chunk_pass(pos, self.mode, chunk)
        }
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
    fn chunk_pass(&self, pos: IVec3, mode: WorldGenerationMode, chunk: &mut Chunk);
}