use std::sync::{Arc, RwLock};
use bevy::{prelude::Resource, math::DVec3, utils::HashMap};

#[derive(Resource)]
pub struct NoiseTable(Arc<RwLock<NoiseTableInternal>>);
impl NoiseTable {
    pub(crate) fn new() -> Self {
        Self(Arc::new(RwLock::new(NoiseTableInternal::new())))
    }

    pub(crate) fn set_seed(&self, seed: u32) {
        self.0.write().unwrap().set_seed(seed);
    }
}

pub struct NoiseTableInternal(HashMap<String, Box<dyn NoiseLayer>>);
impl NoiseTableInternal {
    fn new() -> Self {
        Self(HashMap::new())
    }

    #[allow(dead_code)] // rust analyser isn't detecting usages
    pub(crate) fn set_seed(&mut self, seed: u32) {
        for layer in self.0.values_mut() {
            layer.set_seed(seed);
        }
    }
}

pub trait NoiseLayer: 'static + Send + Sync {
    fn get_value(&self, pos: DVec3) -> f64;
    fn set_seed(&mut self, seed: u32);
}