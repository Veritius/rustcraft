use bevy::render::once_cell::sync::Lazy;
use noise::Perlin;

pub static WGEN_SURFACE_NOISE_1: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01011110010111011001000110101011)});
pub const WGEN_SURFACE_NOISE_1_MODIFIER: f64 = 0.016639428;
pub static WGEN_SURFACE_NOISE_2: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b10011001111101100101111000110001)});
pub const WGEN_SURFACE_NOISE_2_MODIFIER: f64 = 0.0093313213;
pub static WGEN_SURFACE_NOISE_3: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01110110000000001100101100111101)});
pub const WGEN_SURFACE_NOISE_3_MODIFIER: f64 = 0.001219412;