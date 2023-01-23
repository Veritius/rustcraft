use std::collections::BTreeMap;
use bevy::{prelude::{Resource, IVec3}, utils::HashMap};
use crate::attributes::{AttributeKind, AttributeValue};

use super::{BiomeId, scorer::BiomeSelectionScorer};

#[derive(Resource)]
pub struct BiomeRegistry {
    id_index: u32,
    biomes: HashMap<u32, BiomeData>,
    scorers: Vec<Box<dyn BiomeSelectionScorer>>,
}

impl BiomeRegistry {
    pub(crate) fn new() -> Self {
        Self {
            id_index: 0,
            biomes: HashMap::new(),
            scorers: vec![],
        }
    }

    pub fn add_biome_type(&mut self, biome: BiomeData) -> BiomeId {
        let id = self.id_index;
        self.biomes.insert(id, biome);
        self.id_index += 1;

        id
    }

    pub(crate) fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) {
        self.scorers.push(Box::new(scorer))
    }

    pub fn calculate_biome_for_chunk(&self, pos: IVec3) -> BiomeId {
        let mut biggest = (0.0, BiomeId::MAX);
        for (id, biome) in &self.biomes {
            let mut current = 0.0;
            for scorer in &self.scorers {
                current += scorer.get_weight_for_coordinates(pos, &biome);
            }
            if current > biggest.0 {
                biggest.0 = current;
                biggest.1 = *id;
            }
        }

        biggest.1
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
            panic!("Failed to insert attribute. Invalid attribute kind for {}. Given kind is {value_kind:?} but expected {:?}", attribute.name, attribute.kind);
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
    /// Creates a new BiomeAttribute from the given arguments.
    /// 
    /// The `name` and `value` fields can be anything, but the `id` field has to be a _unique_ number.
    /// This can be chosen by setting a very large or random number.
    /// Do not use a close-to-zero pattern, as this will collide with the engine attribute IDs.
    pub const fn new(name: &'static str, id: u32, value: AttributeKind) -> Self {
        BiomeAttribute { name, id, kind: value }
    }
}