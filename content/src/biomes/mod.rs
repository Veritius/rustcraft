use rustcraft_modlib::{world::generation::biome::table::BiomeData, attributes::AttributeValue};

pub(crate) fn ocean() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Ocean"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, AttributeValue::RangeI32(0..70));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, AttributeValue::RangeI32(0..60));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, AttributeValue::RangeU16(0..100));

    biome
}

pub(crate) fn plains() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Plains"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, AttributeValue::RangeI32(70..120));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, AttributeValue::RangeI32(15..30));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, AttributeValue::RangeU16(25..35));

    biome
}

pub(crate) fn forest() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Forest"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, AttributeValue::RangeI32(80..140));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, AttributeValue::RangeI32(20..35));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, AttributeValue::RangeU16(35..65));

    biome
}

pub(crate) fn jungle() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Jungle"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, AttributeValue::RangeI32(80..130));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, AttributeValue::RangeI32(35..45));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, AttributeValue::RangeU16(55..100));

    biome
}

pub(crate) fn desert() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Desert"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, AttributeValue::RangeI32(70..150));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, AttributeValue::RangeI32(35..65));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, AttributeValue::RangeU16(10..70));

    biome
}

pub(crate) fn tundra() -> BiomeData {
    let mut biome = BiomeData::new();
    biome.insert_attribute(BiomeData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Tundra"));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HEIGHT, AttributeValue::RangeI32(70..150));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_TEMPERATURE, AttributeValue::RangeI32(-20..0));
    biome.insert_attribute(BiomeData::ATTRIBUTE_GENVAR_HUMIDITY, AttributeValue::RangeU16(15..100));

    biome
}