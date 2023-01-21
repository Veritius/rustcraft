use std::{
    collections::BTreeMap,
    ops::Range,
};
use bevy::{
    prelude::*,
    utils::HashMap,
};

pub trait AddBiome {
   fn add_biome(&mut self, biome: BiomeData) -> &mut Self;
}

impl AddBiome for App {
    fn add_biome(&mut self, biome: BiomeData) -> &mut Self {
        self.add_startup_system(move |mut biome_table: ResMut<BiomeTable>| {
            biome_table.add_biome_type(biome.clone());
        });

        self
    }
}

type BiomeId = u32;

#[derive(Resource)]
pub struct BiomeTable {
    id_index: u32,
    map: HashMap<u32, BiomeData>,
}

impl BiomeTable {
    pub(crate) fn new() -> Self {
        Self {
            id_index: 0,
            map: HashMap::new(),
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
    attributes: BTreeMap<u32, BiomeAttributeValue>,  
}

impl BiomeData {
    pub const ATTRIBUTE_DISPLAY_NAME: BiomeAttribute =
        BiomeAttribute::new("biome_display_name", 0, BiomeAttributeKind::StaticStr);
    pub const ATTRIBUTE_GENVAR_HEIGHT: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_height", 1, BiomeAttributeKind::RangeI32);
    pub const ATTRIBUTE_GENVAR_TEMPERATURE: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_temperature", 2, BiomeAttributeKind::RangeI32);
    pub const ATTRIBUTE_GENVAR_HUMIDITY: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_humidity", 3, BiomeAttributeKind::RangeU16);

    pub fn new() -> Self {
        Self {
            attributes: BTreeMap::new()
        }
    }

    pub fn insert_attribute(&mut self, attribute: BiomeAttribute, value: BiomeAttributeValue) {
        let value_kind = BiomeAttributeKind::from(&value);
        if attribute.kind != value_kind {
            panic!("Failed to insert attribute. Invalid attribute kind for {}. Given kind is {value_kind:?} but expected {:?}",
            attribute.name, attribute.kind);
        }

        self.attributes.insert(attribute.id, value);
    }

    pub(crate) fn get_attribute(&self, attribute: BiomeAttribute) -> Option<&BiomeAttributeValue> {
        self.attributes.get(&attribute.id)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct BiomeAttribute {
    name: &'static str,
    /// _Unique_ id for this attribute. If in doubt, make a very large or random number.
    /// Built in attributes follow a close-to-zero pattern.
    id: u32,
    kind: BiomeAttributeKind,
}

impl BiomeAttribute {
    pub const fn new(name: &'static str, id: u32, value: BiomeAttributeKind) -> Self {
        BiomeAttribute { name, id, kind: value }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BiomeAttributeKind {
    String,
    StaticStr,
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

impl From<&BiomeAttributeValue> for BiomeAttributeKind {
    fn from(value: &BiomeAttributeValue) -> Self {
        match value {
            BiomeAttributeValue::String(_) => BiomeAttributeKind::String,
            BiomeAttributeValue::StaticStr(_) => BiomeAttributeKind::StaticStr,
            BiomeAttributeValue::Uint16(_) => BiomeAttributeKind::Uint16,
            BiomeAttributeValue::Uint32(_) => BiomeAttributeKind::Uint32,
            BiomeAttributeValue::Uint64(_) => BiomeAttributeKind::Uint64,
            BiomeAttributeValue::Sint16(_) => BiomeAttributeKind::Sint16,
            BiomeAttributeValue::Sint32(_) => BiomeAttributeKind::Sint32,
            BiomeAttributeValue::Sint64(_) => BiomeAttributeKind::Sint64,
            BiomeAttributeValue::Float32(_) => BiomeAttributeKind::Float32,
            BiomeAttributeValue::Float64(_) => BiomeAttributeKind::Float64,
            BiomeAttributeValue::RangeU16(_) => BiomeAttributeKind::RangeU16,
            BiomeAttributeValue::RangeU32(_) => BiomeAttributeKind::RangeU32,
            BiomeAttributeValue::RangeI16(_) => BiomeAttributeKind::RangeI16,
            BiomeAttributeValue::RangeI32(_) => BiomeAttributeKind::RangeU32,
            BiomeAttributeValue::RangeF32(_) => BiomeAttributeKind::RangeF32,
        }
    }
}

#[derive(Clone)]
pub enum BiomeAttributeValue {
    String(String),
    StaticStr(&'static str),
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