use std::{ops::Range, collections::BTreeMap};
use bevy::prelude::{App, ResMut, Color};
use crate::world::chunk::meshing::MeshingVisibility;

use super::registry::BlockRegistry;

#[derive(Clone)]
pub struct BlockData {
    /// Unique, human-readable string identifier for this block, like `engine_air`. 
    /// 
    /// A good way to lay out your string identifiers is the following:
    /// - `rustcraft` - the name of your mod
    /// - `_` - an underscore
    /// - `dirt` - the name of your block
    /// Which gives `rustcraft_dirt`.
    pub string_identifier: &'static str,
    pub block_visibility: MeshingVisibility,
    attributes: BTreeMap<u32, BlockAttributeValue>,
}

impl BlockData {
    pub const ATTRIBUTE_DISPLAY_NAME: BlockAttribute =
        BlockAttribute::new("block_display_name", 0, BlockAttributeKind::StaticStr);
    pub const ATTRIBUTE_COLOR: BlockAttribute =
        BlockAttribute::new("block_base_color", 2, BlockAttributeKind::Color);

    pub fn new(string_identifier: &'static str, block_visibility: MeshingVisibility) -> Self {
        Self {
            string_identifier,
            block_visibility,
            attributes: BTreeMap::new(),
        }
    }

    pub fn insert_attribute(&mut self, attribute: BlockAttribute, value: BlockAttributeValue) {
        let value_kind = BlockAttributeKind::from(&value);
        if attribute.kind != value_kind {
            panic!("Failed to insert attribute. Invalid attribute kind for {}. Given kind is {value_kind:?} but expected {:?}",
            attribute.string_identifier, attribute.kind);
        }

        self.attributes.insert(attribute.id, value);
    }

    pub(crate) fn get_attribute(&self, attribute: BlockAttribute) -> Option<&BlockAttributeValue> {
        self.attributes.get(&attribute.id)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct BlockAttribute {
    string_identifier: &'static str,
    /// _Unique_ id for this attribute. If in doubt, make a very large or random number.
    /// Built in attributes follow a close-to-zero pattern.
    id: u32,
    kind: BlockAttributeKind,
}

impl BlockAttribute {
    pub const fn new(name: &'static str, id: u32, value: BlockAttributeKind) -> Self {
        BlockAttribute { string_identifier: name, id, kind: value }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BlockAttributeKind {
    Color,
    StaticStr,
    String,
    Uint16,
    Uint32,
    Uint64,
    Sint16,
    Sint32,
    Sint64,
    Float32,
    Float64,
    RangeU16,
    RangeU32,
    RangeI16,
    RangeI32,
    RangeF32,
}

impl From<&BlockAttributeValue> for BlockAttributeKind {
    fn from(value: &BlockAttributeValue) -> Self {
        match value {
            BlockAttributeValue::Color(_) => BlockAttributeKind::Color,
            BlockAttributeValue::StaticStr(_) => BlockAttributeKind::StaticStr,
            BlockAttributeValue::String(_) => BlockAttributeKind::String,
            BlockAttributeValue::Uint16(_) => BlockAttributeKind::Uint16,
            BlockAttributeValue::Uint32(_) => BlockAttributeKind::Uint32,
            BlockAttributeValue::Uint64(_) => BlockAttributeKind::Uint64,
            BlockAttributeValue::Sint16(_) => BlockAttributeKind::Sint16,
            BlockAttributeValue::Sint32(_) => BlockAttributeKind::Sint32,
            BlockAttributeValue::Sint64(_) => BlockAttributeKind::Sint64,
            BlockAttributeValue::Float32(_) => BlockAttributeKind::Float32,
            BlockAttributeValue::Float64(_) => BlockAttributeKind::Float64,
            BlockAttributeValue::RangeU16(_) => BlockAttributeKind::RangeU16,
            BlockAttributeValue::RangeU32(_) => BlockAttributeKind::RangeU32,
            BlockAttributeValue::RangeI16(_) => BlockAttributeKind::RangeI16,
            BlockAttributeValue::RangeI32(_) => BlockAttributeKind::RangeU32,
            BlockAttributeValue::RangeF32(_) => BlockAttributeKind::RangeF32,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockAttributeValue {
    Color(Color),
    StaticStr(&'static str),
    String(String),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Sint16(i16),
    Sint32(i32),
    Sint64(i64),
    Float32(f32),
    Float64(f64),
    RangeU16(Range<u16>),
    RangeU32(Range<u32>),
    RangeI16(Range<i16>),
    RangeI32(Range<i32>),
    RangeF32(Range<f32>),
}

pub trait AddBlock {
    fn add_block(&mut self, block: BlockData) -> &mut Self;
}

impl AddBlock for App {
    fn add_block(&mut self, block: BlockData) -> &mut Self {
        self.add_startup_system(move |mut registry: ResMut<BlockRegistry>| {
            registry.add_block_type(block.clone());
        });

        self
    }
}