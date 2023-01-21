use bevy::prelude::Color;
use rustcraft_modlib::world::{
    block::data::{
        BlockData,
        BlockAttributeValue,
    },
    chunk::meshing::MeshingVisibility,
};

// TODO: There's definitely a better way to do this

pub(crate) fn dirt() -> BlockData {
    let mut block = BlockData::new("rustcraft_dirt", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, BlockAttributeValue::StaticStr("Dirt"));
    block.insert_attribute(BlockData::ATTRIBUTE_COLOR, BlockAttributeValue::Color(Color::BEIGE));

    block
}

pub(crate) fn stone() -> BlockData {
    let mut block = BlockData::new("rustcraft_stone", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, BlockAttributeValue::StaticStr("Stone"));
    block.insert_attribute(BlockData::ATTRIBUTE_COLOR, BlockAttributeValue::Color(Color::GRAY));

    block
}

pub(crate) fn sand() -> BlockData {
    let mut block = BlockData::new("rustcraft_sand", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, BlockAttributeValue::StaticStr("Sand"));
    block.insert_attribute(BlockData::ATTRIBUTE_COLOR, BlockAttributeValue::Color(Color::GOLD));

    block
}

pub(crate) fn grass() -> BlockData {
    let mut block = BlockData::new("rustcraft_grass", MeshingVisibility::Opaque);
    block.insert_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME, BlockAttributeValue::StaticStr("Grass"));
    block.insert_attribute(BlockData::ATTRIBUTE_COLOR, BlockAttributeValue::Color(Color::GREEN));

    block
}