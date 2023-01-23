use bevy::prelude::{Vec2, IVec3};
use crate::world::generation::noise_layers::*;
use super::table::BiomeData;

pub trait BiomeWeightingConsideration: Send + Sync {
    fn get_weight_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> u32;
}

/// Considerations for height, temperature, and humidity.
pub(crate) struct BaseWeightingConsiderations;
impl BiomeWeightingConsideration for BaseWeightingConsiderations {
    fn get_weight_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> u32 {
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

        let mut total = 0;

        for (noise_level, attribute) in [
            (height, biome_data.get_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT)),
            (temperature, biome_data.get_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE)),
            (humidity, biome_data.get_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY)),
        ] {
            if attribute.is_none() { continue; }
            let attribute = attribute.unwrap();
        }

        total
    }
}