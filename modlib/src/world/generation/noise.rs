use std::sync::{Arc, RwLock};
use bevy::{prelude::Resource, math::DVec3, utils::HashMap};
use dyn_clone::DynClone;
use noise::{Perlin, Seedable, NoiseFn};

#[derive(Resource)]
pub struct NoiseTable(Arc<RwLock<NoiseTableInternal>>);
impl NoiseTable {
    pub(crate) fn new() -> Self {
        Self(Arc::new(RwLock::new(NoiseTableInternal::new())))
    }

    pub(crate) fn set_seed(&self, seed: u32) {
        self.0.write().unwrap().set_seed(seed);
    }

    pub fn add_layer(&mut self, key: String, layer: Box<dyn NoiseLayer>) {
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

    pub(crate) fn add_layer(&mut self, key: String, layer: Box<dyn NoiseLayer>) {
        self.0.insert(key, layer);
    }
}

pub trait NoiseLayer: 'static + Send + Sync + DynClone {
    fn get_value(&self, pos: DVec3) -> f64;
    fn set_seed(&mut self, seed: u32);
}

#[derive(Clone)]
pub struct SimpleNoiseLayer2D {
    soff: u32,
    vec: Vec<(f64, Perlin, f64)>
}

impl SimpleNoiseLayer2D {
    pub fn new(soff: u32, vec: Vec<(f64, Perlin, f64)>) -> Self {
        Self {
            soff,
            vec,
        }
    }
}

impl NoiseLayer for SimpleNoiseLayer2D {
    fn get_value(&self, pos: DVec3) -> f64 {
        let mut total = 0.0;
        for (multiplier, perlin, modifier) in &self.vec {
            let pos = pos * DVec3::splat(*modifier);
            total += multiplier * perlin.get([pos.x, pos.y])
        }

        total
    }

    fn set_seed(&mut self, seed: u32) {
        let seed = seed.overflowing_add(self.soff).0;
        for (_, perlin, _) in &self.vec {
            perlin.set_seed(seed);
        }
    }
}

#[derive(Clone)]
pub struct SimpleNoiseLayer3D {
    soff: u32,
    vec: Vec<(f64, Perlin, f64)>
}

impl SimpleNoiseLayer3D {
    pub fn new(soff: u32, vec: Vec<(f64, Perlin, f64)>) -> Self {
        Self {
            soff,
            vec,
        }
    }
}

impl NoiseLayer for SimpleNoiseLayer3D {
    fn get_value(&self, pos: DVec3) -> f64 {
        let mut total = 0.0;
        for (multiplier, perlin, modifier) in &self.vec {
            let pos = pos * DVec3::splat(*modifier);
            total += multiplier * perlin.get([pos.x, pos.y, pos.z])
        }

        total
    }

    fn set_seed(&mut self, seed: u32) {
        let seed = seed.overflowing_add(self.soff).0;
        for (_, perlin, _) in &self.vec {
            perlin.set_seed(seed);
        }
    }
}