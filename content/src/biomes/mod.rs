use rustcraft_modlib::world::generation::biome::{BiomeData, BiomeAttributeValue};

pub(crate) fn plains() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Plains"));

    biome
}