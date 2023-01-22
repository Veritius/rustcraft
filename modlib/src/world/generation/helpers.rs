use super::noise_layers::*;
use noise::NoiseFn;

pub fn get_surface_level(x: i32, z: i32) -> f64 {
    let x = x as f64;
    let z = z as f64;

    let mut surface_level = 0.0;
    surface_level += 5.0 * WGEN_SURFACE_NOISE_1.get([x * WGEN_SURFACE_NOISE_1_MODIFIER, z * WGEN_SURFACE_NOISE_1_MODIFIER]);
    surface_level += 10.0 * WGEN_SURFACE_NOISE_2.get([x * WGEN_SURFACE_NOISE_2_MODIFIER, z * WGEN_SURFACE_NOISE_2_MODIFIER]);
    surface_level += 15.0 * WGEN_SURFACE_NOISE_3.get([x * WGEN_SURFACE_NOISE_3_MODIFIER, z * WGEN_SURFACE_NOISE_3_MODIFIER]);

    surface_level
}