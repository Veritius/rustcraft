use rustcraft_modlib::engine::bevy::prelude::Color;
use rustcraft_modlib::engine::{world::{block::data::BlockData, chunk::meshing::MeshingVisibility}, attributes::AttributeValue};

pub(crate) fn water() -> BlockData {
    BlockData::new_with_attributes("rustcraft_water", MeshingVisibility::Translucent, vec![
        (BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Water")),
        (BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::rgba(0.18, 0.55, 0.34, 0.6))),
        (BlockData::ATTRIBUTE_USE_LIQUID_MESHER, AttributeValue::None),
    ])
}

pub(crate) fn dirt() -> BlockData {
    BlockData::new_with_attributes("rustcraft_dirt", MeshingVisibility::Opaque, vec![
        (BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Dirt")),
        (BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::hex("724A11").unwrap())),
        (BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None),
        (BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["dirt"; 6])),
    ])
}

pub(crate) fn stone() -> BlockData {
    BlockData::new_with_attributes("rustcraft_stone", MeshingVisibility::Opaque, vec![
        (BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Stone")),
        (BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::GRAY)),
        (BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None),
        (BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["stone"; 6])),
    ])
}

pub(crate) fn sand() -> BlockData {
    BlockData::new_with_attributes("rustcraft_sand", MeshingVisibility::Opaque, vec![
        (BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Sand")),
        (BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::GOLD)),
        (BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None),
        (BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["sand"; 6])),
    ])
}

pub(crate) fn grass() -> BlockData {
    BlockData::new_with_attributes("rustcraft_grass", MeshingVisibility::Opaque, vec![
        (BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Grass")),
        (BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::GREEN)),
        (BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None),
        (BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["grass_side", "grass_side", "grass_top", "dirt", "grass_side", "grass_side"])),
    ])
}

pub(crate) fn glass() -> BlockData {
    BlockData::new_with_attributes("rustcraft_glass", MeshingVisibility::Translucent, vec![
        (BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Grass")),
        (BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::ANTIQUE_WHITE)),
        (BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["glass"; 6])),
        (BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None),
    ])
}