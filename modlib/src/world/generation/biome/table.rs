use std::{collections::BTreeMap, ops::Range};
use bevy::{prelude::Resource, utils::HashMap};
use crate::attributes::{AttributeKind, AttributeValue};

use super::{BiomeId, considerations::BiomeSelectionScorer};

#[derive(Resource)]
pub struct BiomeTable {
    id_index: u32,
    map: HashMap<u32, BiomeData>,
    considerations: Vec<Box<dyn BiomeSelectionScorer>>,
}

impl BiomeTable {
    pub(crate) fn new() -> Self {
        Self {
            id_index: 0,
            map: HashMap::new(),
            considerations: vec![],
        }
    }

    pub fn add_biome_type(&mut self, biome: BiomeData) -> BiomeId {
        let id = self.id_index;
        self.map.insert(id, biome);
        self.id_index += 1;

        id
    }
}

#[derive(Clone)]
pub struct BiomeData {
    attributes: BTreeMap<u32, AttributeValue>,  
}

impl BiomeData {
    pub const ATTRIBUTE_DISPLAY_NAME: BiomeAttribute =
        BiomeAttribute::new("biome_display_name", 0, AttributeKind::StaticStr);
    /// The altitude range this biome should spawn in.
    pub const ATTRIBUTE_GENVAR_HEIGHT: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_height", 1, AttributeKind::RangeI32);
    /// The temperature range this biome should spawn in.
    pub const ATTRIBUTE_GENVAR_TEMPERATURE: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_temperature", 2, AttributeKind::RangeI32);
    /// The humidity range this biome should spawn in.
    pub const ATTRIBUTE_GENVAR_HUMIDITY: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_humidity", 3, AttributeKind::RangeU16);

    pub fn new() -> Self {
        Self {
            attributes: BTreeMap::new()
        }
    }

    pub fn insert_attribute(&mut self, attribute: BiomeAttribute, value: AttributeValue) {
        let value_kind = AttributeKind::from(&value);
        if attribute.kind != value_kind {
            panic!("Failed to insert attribute. Invalid attribute kind for {}. Given kind is {value_kind:?} but expected {:?}",
            attribute.name, attribute.kind);
        }

        self.attributes.insert(attribute.id, value);
    }

    pub(crate) fn get_attribute(&self, attribute: BiomeAttribute) -> Option<&AttributeValue> {
        self.attributes.get(&attribute.id)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct BiomeAttribute {
    name: &'static str,
    /// _Unique_ id for this attribute. If in doubt, make a very large or random number.
    /// Built in attributes follow a close-to-zero pattern.
    id: u32,
    kind: AttributeKind,
}

impl BiomeAttribute {
    pub const fn new(name: &'static str, id: u32, value: AttributeKind) -> Self {
        BiomeAttribute { name, id, kind: value }
    }
}