use std::ops::Range;
use bevy::prelude::{Vec2, IVec3};
use dyn_clone::DynClone;
use crate::world::generation::noise_layers::*;
use super::table::BiomeData;

dyn_clone::clone_trait_object!(BiomeSelectionScorer);
pub trait BiomeSelectionScorer: 'static + Send + Sync + DynClone {
    fn get_weight_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> f64;
}

/// Biome scoring for height, temperature, and humidity.
/// 
/// Currently does scores for height, temperature, and humidity.
/// Biomes get better scores if they're closer to the middle point of their range.
// TODO: Move this to content
#[derive(Clone)]
pub(crate) struct BaseSelectionScorer;
impl BiomeSelectionScorer for BaseSelectionScorer {
    fn get_weight_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> f64 {
        let coordinates_as_vec2 = Vec2 { x: coordinates.x as f32, y: coordinates.z as f32 };
        let height = add_up_2d(vec![
            (&WGEN_HEIGHT_NOISE_1, WGEN_HEIGHT_NOISE_1_MODIFIER, coordinates_as_vec2),
            (&WGEN_HEIGHT_NOISE_2, WGEN_HEIGHT_NOISE_2_MODIFIER, coordinates_as_vec2),
            (&WGEN_HEIGHT_NOISE_3, WGEN_HEIGHT_NOISE_3_MODIFIER, coordinates_as_vec2),
        ]);
        let temperature = add_up_2d(vec![
            (&WGEN_TEMPERATURE_NOISE_1, WGEN_TEMPERATURE_NOISE_1_MODIFIER, coordinates_as_vec2),
            (&WGEN_TEMPERATURE_NOISE_2, WGEN_TEMPERATURE_NOISE_2_MODIFIER, coordinates_as_vec2),
            (&WGEN_TEMPERATURE_NOISE_3, WGEN_TEMPERATURE_NOISE_3_MODIFIER, coordinates_as_vec2),
        ]);
        let humidity = add_up_2d(vec![
            (&WGEN_HUMIDITY_NOISE_1, WGEN_HUMIDITY_NOISE_1_MODIFIER, coordinates_as_vec2),
            (&WGEN_HUMIDITY_NOISE_2, WGEN_HUMIDITY_NOISE_2_MODIFIER, coordinates_as_vec2),
            (&WGEN_HUMIDITY_NOISE_3, WGEN_HUMIDITY_NOISE_3_MODIFIER, coordinates_as_vec2),
        ]);

        let mut total = 0.0;

        // Calculate score
        for (level, attribute) in [
            (height, biome_data.get_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT)),
            (temperature, biome_data.get_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE)),
            (humidity, biome_data.get_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY)),
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