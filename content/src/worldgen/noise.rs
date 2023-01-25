use bevy::math::DVec3;
use rustcraft_modlib::{world::generation::noise::NoiseLayer, noise_rs::{Perlin, NoiseFn, Seedable}};

pub const NOISE_LAYER_SURFACE: &'static str = "rustcraft_noise_layer_surface";

#[derive(Clone)]
pub(crate) struct SurfaceNoise(Vec<(f64, Perlin, f64)>);

impl NoiseLayer for SurfaceNoise {
    fn new() -> Self where Self: Sized {
        Self(vec![
            (5.0, Perlin::new(0), 0.013412525),
            (10.0, Perlin::new(0), 0.00713241),
            (15.0, Perlin::new(0), 0.00215235),
        ])
    }

    fn get_value(&self, pos: DVec3) -> f64 {
        let mut total = 0.0;
        for (multiplier, perlin, modifier) in &self.0 {
            let pos = pos * DVec3::splat(*modifier);
            total += multiplier * perlin.get([pos.x, pos.y])
        }

        total
    }

    fn set_seed(&mut self, seed: u32) {
        for (_, perlin, _) in &self.0 {
            perlin.set_seed(seed);
        }
    }
}