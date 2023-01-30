use std::{ops::Range, collections::BTreeMap, hash::Hash};
use bevy::prelude::{App, ResMut, Color};
use crate::{world::chunk::meshing::MeshingVisibility, attributes::{AttributeKind, AttributeValue}};

use super::registry::Blocks;

/// Storage for BlockAttributes.
#[derive(Clone)]
pub struct BlockData {
    /// Unique, human-readable string identifier for this block, like `engine_air`. 
    /// 
    /// A good way to lay out your string identifiers is the following:
    /// - `rustcraft` - the name of your mod
    /// - `_` - an underscore
    /// - `dirt` - the name of your block
    /// 
    /// Which gives `rustcraft_dirt`.
    pub string_identifier: &'static str,
    pub block_visibility: MeshingVisibility,
    attributes: BTreeMap<u32, AttributeValue>,
}

impl BlockData {
    pub const ATTRIBUTE_DISPLAY_NAME: BlockAttribute =
        BlockAttribute::new("engine_display_name", 0, AttributeKind::StaticStr);
    /// A base color for the block. Usually used either for debugging or massive world views.
    pub const ATTRIBUTE_BASE_COLOR: BlockAttribute =
        BlockAttribute::new("engine_base_color", 1, AttributeKind::Color);
    /// Marker for the `SolidBlockMesher` to know to draw this block.
    pub const ATTRIBUTE_USE_SOLID_MESHER: BlockAttribute =
        BlockAttribute::new("engine_use_solid_mesher", 2, AttributeKind::None);
    /// Image ids for each side of a solid block, in this order:
    /// Left, right, up, down, forward, back.
    pub const ATTRIBUTE_SOLID_TEXTURE_SIDES: BlockAttribute =
        BlockAttribute::new("engine_texture_sides", 3, AttributeKind::StaticStrX6);
    /// Marker for the `LiquidMesher` to know to draw this block.
    pub const ATTRIBUTE_USE_LIQUID_MESHER: BlockAttribute =
        BlockAttribute::new("engine_use_liquid_mesher", 4, AttributeKind::None);

    pub fn new(string_identifier: &'static str, block_visibility: MeshingVisibility) -> Self {
        Self {
            string_identifier,
            block_visibility,
            attributes: BTreeMap::new(),
        }
    }

    pub fn new_with_attributes(string_identifier: &'static str, block_visibility: MeshingVisibility, attributes: Vec<(BlockAttribute, AttributeValue)>) -> Self {
        let mut block = Self::new(string_identifier, block_visibility);
        for (attribute, value) in attributes {
            block.insert_attribute(attribute, value);
        }

        block
    }

    pub fn insert_attribute(&mut self, attribute: BlockAttribute, value: AttributeValue) {
        let value_kind = AttributeKind::from(&value);
        if attribute.kind != value_kind {
            panic!("Failed to insert attribute. Invalid attribute kind for {}. Given kind is {value_kind:?} but expected {:?}",
            attribute.string_identifier, attribute.kind);
        }

        self.attributes.insert(attribute.id, value);
    }

    #[must_use]
    pub(crate) fn get_attribute(&self, attribute: BlockAttribute) -> Option<&AttributeValue> {
        self.attributes.get(&attribute.id)
    }
}

// TODO: Hash only the `id` field.
#[derive(Hash)]
pub struct BlockAttribute {
    string_identifier: &'static str,
    /// _Unique_ id for this attribute. If in doubt, make a very large or random number.
    /// Built in attributes follow a close-to-zero pattern.
    id: u32,
    kind: AttributeKind,
}

impl PartialEq for BlockAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BlockAttribute {
    pub const fn new(name: &'static str, id: u32, value: AttributeKind) -> Self {
        BlockAttribute { string_identifier: name, id, kind: value }
    }
}

pub trait AddBlock {
    fn add_block(&mut self, block: BlockData) -> &mut Self;
}

impl AddBlock for App {
    /// Adds a new block type. Shorthand for
    /// 
    /// ```rs
    /// BLOCK_REGISTRY.write().unwrap().add_block_type()
    /// ```
    fn add_block(&mut self, block: BlockData) -> &mut Self {
        self.add_startup_system(move |mut registry: ResMut<Blocks>| {
            registry.add_block_type(block.clone());
        });

        self
    }
}