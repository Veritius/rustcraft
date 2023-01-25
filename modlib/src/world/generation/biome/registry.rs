use std::{collections::BTreeMap, sync::{Arc, RwLock}, ops::Deref};
use bevy::{prelude::*, utils::HashMap, render::once_cell::sync::Lazy};
use crate::attributes::{AttributeKind, AttributeValue};
use super::{BiomeId, scorer::BiomeSelectionScorer};

pub static BIOME_REGISTRY: Lazy<Arc<RwLock<BiomesInternal>>> = Lazy::new(||{Arc::new(RwLock::new(BiomesInternal::new()))});

#[derive(Resource)]
pub struct Biomes(pub Arc<RwLock<BiomesInternal>>);

impl Biomes {
    pub fn add_biome(&self, biome: BiomeData) {
        self.0.write().unwrap().add_biome(biome);
    }

    pub fn add_biome_scorer(&self, scorer: impl BiomeSelectionScorer) {
        self.0.write().unwrap().add_biome_scorer(scorer);
    }

    fn get_biome_data(&self, id: BiomeId) -> Option<BiomeData> {
        self.0.read().unwrap().get_biome_data(id)
    }
}

impl Default for Biomes {
    fn default() -> Self {
        Self(BIOME_REGISTRY.clone())
    }
}

#[derive(Clone)]
pub struct BiomesInternal {
    last_idx: u32,
    biomes: HashMap<u32, BiomeData>,
    scorers: Vec<Box<dyn BiomeSelectionScorer>>,
}

impl BiomesInternal {
    pub(crate) fn new() -> Self {
        Self {
            last_idx: 0,
            biomes: HashMap::new(),
            scorers: vec![],
        }
    }

    pub fn add_biome(&mut self, biome: BiomeData) {
        let id = self.last_idx;
        self.biomes.insert(id, biome);
        self.last_idx += 1;
    }

    pub fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) {
        self.scorers.push(Box::new(scorer));
    }

    pub fn get_biome_data(&self, id: BiomeId) -> Option<BiomeData> {
        match self.biomes.get(&id) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn calculate_biome_for_chunk(&self, pos: IVec3) -> BiomeId {
        let mut biggest = (0.0, BiomeId::MAX);
        for (id, biome) in &self.biomes {
            let mut current = 0.0;
            for scorer in &self.scorers {
                current += scorer.get_point_score_for_coordinates(pos, &biome);
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

    pub fn get_attribute(&self, attribute: BiomeAttribute) -> Option<&AttributeValue> {
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