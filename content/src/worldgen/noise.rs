use bevy::math::DVec3;
use rustcraft_modlib::{world::generation::noise::NoiseLayer, noise_rs::{Perlin, NoiseFn, Seedable}};

pub const NOISE_LAYER_HEIGHT: &'static str = "rustcraft_noise_layer_height";
pub const NOISE_LAYER_TEMPERATURE: &'static str = "rustcraft_noise_layer_temperature";
pub const NOISE_LAYER_HUMIDITY: &'static str = "rustcraft_noise_layer_humidity";