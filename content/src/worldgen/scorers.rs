use std::ops::Range;
use rustcraft_modlib::engine::bevy::{prelude::IVec3, math::DVec3};
use rustcraft_modlib::engine::world::generation::{biome::{scorer::BiomeSelectionScorer, registry::BiomeData}, generator::WORLD_GENERATION};
use super::noise::{NOISE_LAYER_HEIGHT, NOISE_LAYER_TEMPERATURE, NOISE_LAYER_HUMIDITY};
use crate::biomes::attributes::{ATTRIBUTE_GENVAR_HEIGHT, ATTRIBUTE_GENVAR_TEMPERATURE, ATTRIBUTE_GENVAR_HUMIDITY};

/// Biome scoring for height, temperature, and humidity.
/// 
/// Currently does scores for height, temperature, and humidity.
/// Biomes get better scores if they're closer to the middle point of their range.
// TODO: Move this to content
#[derive(Clone)]
pub(crate) struct BaseSelectionScorer;
impl BiomeSelectionScorer for BaseSelectionScorer {
    fn get_point_score_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> f64 {
        let worldgen_data = WORLD_GENERATION.read().unwrap();
        let d_vec = DVec3 { x: coordinates.x as f64, y: coordinates.y as f64, z: coordinates.z as f64 };
        let height = worldgen_data.get_noise_layer(NOISE_LAYER_HEIGHT).unwrap().get_value(d_vec);
        let temperature = worldgen_data.get_noise_layer(NOISE_LAYER_TEMPERATURE).unwrap().get_value(d_vec);
        let humidity = worldgen_data.get_noise_layer(NOISE_LAYER_HUMIDITY).unwrap().get_value(d_vec);

        let mut total = 0.0;

        // Calculate score
        for (level, attribute) in [
            (height, biome_data.get_attribute(ATTRIBUTE_GENVAR_HEIGHT)),
            (temperature, biome_data.get_attribute(ATTRIBUTE_GENVAR_TEMPERATURE)),
            (humidity, biome_data.get_attribute(ATTRIBUTE_GENVAR_HUMIDITY)),
        ] {
            if attribute.is_none() { continue; }
            let attribute: Result<Range<f32>, ()> = attribute.unwrap().clone().try_into();
            if attribute.is_err() { continue; }
            let attribute = attribute.unwrap();

            // Find midpoint between minimum and maximum
            let midpoint = (attribute.start + attribute.end) as f64 / 2.0;
            
            // Calculate point score
            let distance = (level - midpoint).abs();
            let value = (10.0 - distance).max(0.0);

            // Apply to total
            total += value;
        }

        total
    }
}