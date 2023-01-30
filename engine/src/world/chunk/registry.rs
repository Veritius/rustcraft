use std::collections::BTreeMap;
use bevy::prelude::{Resource, Entity, warn, IVec3};

pub type ChunkCoordinate = (i32, i32, i32);

#[derive(Resource)]
pub struct Chunks {
    registry: BTreeMap<ChunkCoordinate, ChunkState>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    /// This variant has multiple meanings.
    /// If it's returned from `get`, the chunk does not exist.
    /// If it's passed in `set`, it removes the chunk from the registry.
    Absent,
    BeingGenerated,
    Present(Entity),
}

impl Chunks {
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
        }
    }

    pub fn get(&self, coord: ChunkCoordinate) -> ChunkState {
        match self.registry.get(&coord) {
            Some(value) => *value,
            None => ChunkState::Absent,
        }
    }

    /// Removes the chunk from the registry. This does not delete the entity itself! Use WorldMapHelpers for confidence in deleting chunks.
    pub fn set(&mut self, coord: ChunkCoordinate, to: ChunkState){
        match to {
            ChunkState::Absent => {
                self.registry.remove(&coord);
            },
            _ => {
                self.registry.insert(coord, to);
            },
        }
    }

    pub fn get_inner_registry(&self) -> &BTreeMap<ChunkCoordinate, ChunkState> {
        &self.registry
    }
}