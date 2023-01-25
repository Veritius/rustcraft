use std::sync::{Arc, RwLock};
use bevy::{prelude::Resource, math::DVec3, utils::HashMap};
use dyn_clone::DynClone;

#[derive(Resource)]
pub struct NoiseTable(Arc<RwLock<NoiseTableInternal>>);
impl NoiseTable {
    pub(crate) fn new() -> Self {
        Self(Arc::new(RwLock::new(NoiseTableInternal::new())))
    }

    pub(crate) fn set_seed(&self, seed: u32) {
        self.0.write().unwrap().set_seed(seed);
    }

    pub fn add_layer(&mut self, key: String, layer: impl NoiseLayer) {
        self.0.write().unwrap().add_layer(key, layer);
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

    pub(crate) fn add_layer(&mut self, key: String, layer: impl NoiseLayer) {
        self.0.insert(key, Box::new(layer));
    }
}

pub trait NoiseLayer: 'static + Send + Sync + DynClone {
    fn get_value(&self, pos: DVec3) -> f64;
    fn set_seed(&mut self, seed: u32);
}