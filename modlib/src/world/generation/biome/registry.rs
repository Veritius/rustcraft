use std::{collections::BTreeMap, sync::{Arc, RwLock}, ops::Deref};
use bevy::{prelude::*, utils::HashMap};
use crate::{attributes::{AttributeKind, AttributeValue}, world::{block::registry::BlockRegistryStartupBuffer, generation::noise::NoiseTableInternal}};

use super::{BiomeId, scorer::BiomeSelectionScorer};

/// Temporary resource that only exists at startup for systems to add new kinds of 
#[derive(Resource)]
pub(crate) struct BiomeRegistryStartupBuffer {
    id_index: u32,
    internal: BiomeRegistryInternal,
}

impl BiomeRegistryStartupBuffer {
    pub fn new() -> Self {
        Self {
            id_index: 0,
            internal: BiomeRegistryInternal::new(),
        }
    }
    
    pub fn add_biome_type(&mut self, biome: BiomeData) {
        let id = self.id_index;
        self.internal.biomes.insert(id, biome);
        self.id_index += 1;
    }

    pub(crate) fn add_biome_scorer(&mut self, scorer: impl BiomeSelectionScorer) {
        self.internal.scorers.push(Box::new(scorer))
    }
}

/// Stores an arc of the internal biome registry.
#[derive(Resource)]
pub struct BiomeRegistry {
    internal: Arc<BiomeRegistryInternal>,
}

impl BiomeRegistry {
    pub(crate) fn new(from: BiomeRegistryInternal) -> Self {
        Self { internal: Arc::new(from) }
    }

    pub fn get_internal_registry(&self) -> Arc<BiomeRegistryInternal> {
        self.internal.clone()
    }
}

impl Deref for BiomeRegistry {
    type Target = BiomeRegistryInternal;

    fn deref(&self) -> &Self::Target {
        self.internal.deref()
    }
}

#[derive(Clone)]
pub struct BiomeRegistryInternal {
    biomes: HashMap<u32, BiomeData>,
    scorers: Vec<Box<dyn BiomeSelectionScorer>>,
}

impl BiomeRegistryInternal {
    pub(crate) fn new() -> Self {
        Self {
            biomes: HashMap::new(),
            scorers: vec![],
        }
    }

    pub fn get_biome_data(&self, id: BiomeId) -> Option<&BiomeData> {
        self.biomes.get(&id)
    }

    pub fn calculate_biome_for_chunk(&self, pos: IVec3, noise_table: &NoiseTableInternal) -> BiomeId {
        let mut biggest = (0.0, BiomeId::MAX);
        for (id, biome) in &self.biomes {
            let mut current = 0.0;
            for scorer in &self.scorers {
                current += scorer.get_point_score_for_coordinates(pos, &biome, noise_table);
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

pub(crate) fn biome_buffer_transfer_system(
    mut commands: Commands,
    buffer: Res<BiomeRegistryStartupBuffer>,
) {
    commands.insert_resource(BiomeRegistry::new(buffer.internal.clone()));
    commands.remove_resource::<BiomeRegistryStartupBuffer>();
}