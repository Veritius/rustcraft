use bevy::{render::once_cell::sync::Lazy, prelude::{Vec2, Vec3}};
use noise::{Perlin, NoiseFn};

// TODO: Changing seeds

/// Adds up a vec of tuples, with a reference to the noise generator, modifier, and position.
pub fn add_up_2d(c: Vec<(&Perlin, f64, Vec2)>) -> f64 {
    let mut v = 0.0;
    for i in c {
        v += i.1 * i.0.get([i.2.x as f64, i.2.y as f64]);
    }
    v
}

/// Adds up a vec of tuples, with a reference to the noise generator, modifier, and position.
pub fn add_up_3d(c: Vec<(&Perlin, f64, Vec3)>) -> f64 {
    let mut v = 0.0;
    for i in c {
        v += i.1 * i.0.get([i.2.x as f64, i.2.y as f64, i.2.z as f64]);
    }
    v
}

pub static WGEN_SURFACE_NOISE_1: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01011110010111011001000110101011)});
pub const WGEN_SURFACE_NOISE_1_MODIFIER: f64 = 0.016639428;
pub static WGEN_SURFACE_NOISE_2: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b10011001111101100101111000110001)});
pub const WGEN_SURFACE_NOISE_2_MODIFIER: f64 = 0.0093313213;
pub static WGEN_SURFACE_NOISE_3: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01110110000000001100101100111101)});
pub const WGEN_SURFACE_NOISE_3_MODIFIER: f64 = 0.001219412;

pub static WGEN_HEIGHT_NOISE_1: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01011010100101101010111100001101)});
pub const WGEN_HEIGHT_NOISE_1_MODIFIER: f64 = 0.01232553;
pub static WGEN_HEIGHT_NOISE_2: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b10101001010100011000011111101111)});
pub const WGEN_HEIGHT_NOISE_2_MODIFIER: f64 = 0.00721256;
pub static WGEN_HEIGHT_NOISE_3: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01000100101000110001010010000110)});
pub const WGEN_HEIGHT_NOISE_3_MODIFIER: f64 = 0.001542412;

pub static WGEN_TEMPERATURE_NOISE_1: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b01000010011000001010111001011101)});
pub const WGEN_TEMPERATURE_NOISE_1_MODIFIER: f64 = 0.019638214;
pub static WGEN_TEMPERATURE_NOISE_2: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b10100011111101001000011111101010)});
pub const WGEN_TEMPERATURE_NOISE_2_MODIFIER: f64 = 0.008532412;
pub static WGEN_TEMPERATURE_NOISE_3: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b00101111001111101101101100110011)});
pub const WGEN_TEMPERATURE_NOISE_3_MODIFIER: f64 = 0.001864563;

pub static WGEN_HUMIDITY_NOISE_1: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b11010100000101001011001001010010)});
pub const WGEN_HUMIDITY_NOISE_1_MODIFIER: f64 = 0.01121253;
pub static WGEN_HUMIDITY_NOISE_2: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b00110101011111100100000111111001)});
pub const WGEN_HUMIDITY_NOISE_2_MODIFIER: f64 = 0.00953263;
pub static WGEN_HUMIDITY_NOISE_3: Lazy<Perlin> = Lazy::new(||{Perlin::new(0b00001011101011010010111101011010)});
pub const WGEN_HUMIDITY_NOISE_3_MODIFIER: f64 = 0.001313215;