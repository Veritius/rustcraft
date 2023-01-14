use std::collections::BTreeMap;
use bevy::prelude::{Resource, Entity, warn, IVec3};

pub type ChunkCoordinate = (i32, i32, i32);

#[derive(Resource)]
pub struct ChunkRegistry {
    registry: BTreeMap<ChunkCoordinate, ChunkState>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Absent,
    BeingGenerated,
    Present(Entity),
}

impl ChunkRegistry {
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

#[derive(Debug)]
pub enum ChunkOperationError {
    NoChunkInPosition(ChunkCoordinate),
    ChunkAlreadyPresent(ChunkCoordinate),
}

impl std::fmt::Display for ChunkOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkOperationError::NoChunkInPosition(coord) => { f.write_str(&format!("No chunk in position {:?}", coord)) },
            ChunkOperationError::ChunkAlreadyPresent(coord) => { f.write_str(&format!("Chunk already present at {:?}", coord)) },
        }
    }
}