use rustcraft_modlib::world::generation::biome::{BiomeData, BiomeAttributeValue};

pub(crate) fn ocean() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Ocean"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, BiomeAttributeValue::RangeI32(0..70));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, BiomeAttributeValue::RangeI32(0..60));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, BiomeAttributeValue::RangeU16(0..100));

    biome
}

pub(crate) fn plains() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Plains"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, BiomeAttributeValue::RangeI32(70..120));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, BiomeAttributeValue::RangeI32(15..30));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, BiomeAttributeValue::RangeU16(25..35));

    biome
}

pub(crate) fn forest() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Forest"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, BiomeAttributeValue::RangeI32(80..140));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, BiomeAttributeValue::RangeI32(20..35));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, BiomeAttributeValue::RangeU16(35..65));

    biome
}

pub(crate) fn jungle() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Jungle"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, BiomeAttributeValue::RangeI32(80..130));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, BiomeAttributeValue::RangeI32(35..45));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, BiomeAttributeValue::RangeU16(55..100));

    biome
}

pub(crate) fn desert() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Desert"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, BiomeAttributeValue::RangeI32(70..150));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, BiomeAttributeValue::RangeI32(35..65));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, BiomeAttributeValue::RangeU16(10..70));

    biome
}

pub(crate) fn tundra() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, BiomeAttributeValue::StaticStr("Tundra"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, BiomeAttributeValue::RangeI32(70..150));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, BiomeAttributeValue::RangeI32(-20..0));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, BiomeAttributeValue::RangeU16(15..100));

    biome
}