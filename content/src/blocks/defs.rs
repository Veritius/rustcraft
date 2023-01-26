use bevy::prelude::Color;
use rustcraft_modlib::{world::{block::data::BlockData, chunk::meshing::MeshingVisibility}, attributes::AttributeValue};

pub(crate) fn water() -> BlockData {
    let mut block = BlockData::new("rustcraft_water", MeshingVisibility::Translucent);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Water"));
    block.insert_attribute(BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::rgba(0.18, 0.55, 0.34, 0.6)));
    block.insert_attribute(BlockData::ATTRIBUTE_USE_LIQUID_MESHER, AttributeValue::None);

    block
}

pub(crate) fn dirt() -> BlockData {
    let mut block = BlockData::new("rustcraft_dirt", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Dirt"));
    block.insert_attribute(BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::hex("724A11").unwrap()));
    block.insert_attribute(BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None);
    block.insert_attribute(BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["dirt"; 6]));

    block
}

pub(crate) fn stone() -> BlockData {
    let mut block = BlockData::new("rustcraft_stone", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Stone"));
    block.insert_attribute(BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::GRAY));
    block.insert_attribute(BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None);
    block.insert_attribute(BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["stone"; 6]));

    block
}

pub(crate) fn sand() -> BlockData {
    let mut block = BlockData::new("rustcraft_sand", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Sand"));
    block.insert_attribute(BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::GOLD));
    block.insert_attribute(BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None);
    block.insert_attribute(BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["sand"; 6]));

    block
}

pub(crate) fn grass() -> BlockData {
    let mut block = BlockData::new("rustcraft_grass", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Grass"));
    block.insert_attribute(BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::GREEN));
    block.insert_attribute(BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None);
    block.insert_attribute(BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["grass_side", "grass_side", "grass_top", "dirt", "grass_side", "grass_side"]));

    block
}

pub(crate) fn glass() -> BlockData {
    let mut block = BlockData::new("rustcraft_glass", MeshingVisibility::Translucent);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, AttributeValue::StaticStr("Grass"));
    block.insert_attribute(BlockData::ATTRIBUTE_BASE_COLOR, AttributeValue::Color(Color::ANTIQUE_WHITE));
    block.insert_attribute(BlockData::ATTRIBUTE_SOLID_TEXTURE_SIDES, AttributeValue::StaticStrX6(["glass"; 6]));
    block.insert_attribute(BlockData::ATTRIBUTE_USE_SOLID_MESHER, AttributeValue::None);

    block
}