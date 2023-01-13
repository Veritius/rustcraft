use std::collections::BTreeMap;
use bevy::prelude::{Resource, Entity, warn};

pub type ChunkCoordinate = (i32, i32, i32);

#[derive(Resource)]
pub struct ChunkRegistry {
    registry: BTreeMap<i32, BTreeMap<i32, BTreeMap<i32, Entity>>>,
}

impl ChunkRegistry {
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
        }
    }

    pub fn get(&self, coord: ChunkCoordinate) -> Result<Option<&Entity>, ChunkOperationError> {
        match self.registry.get(&coord.0) {
            Some(one) => {
                match one.get(&coord.1) {
                    Some(two) => {
                        return Ok(two.get(&coord.2))
                    },
                    None => {
                        return Err(ChunkOperationError::NoChunkInPosition(coord))
                    },
                }
            },
            None => { return Err(ChunkOperationError::NoChunkInPosition(coord)) },
        }
    }

    pub fn set(&mut self, coord: ChunkCoordinate, to: Option<Entity>) -> Result<(), ChunkOperationError> {
        match self.registry.get_mut(&coord.0) {
            Some(one) => {
                match one.get_mut(&coord.1) {
                    Some(two) => {
                        match to {
                            Some(value) => {
                                match two.get(&coord.2) {
                                    Some(_) => { return Err(ChunkOperationError::ChunkAlreadyPresent(coord)) },
                                    None => {
                                        two.insert(coord.2, value);
                                        return Ok(())
                                    },
                                }
                            },
                            None => todo!(),
                        }
                    },
                    None => {
                        let mut layer_3 = BTreeMap::new();
                        if let Some(to) = to {
                            layer_3.insert(coord.2, to);
                        }
                        one.insert(coord.0, layer_3);
                        Ok(())
                    },
                }
            },
            None => {
                let mut layer_3 = BTreeMap::new();
                if let Some(to) = to {
                    layer_3.insert(coord.2, to);
                }
                let mut layer_2 = BTreeMap::new();
                layer_2.insert(coord.1, layer_3);
                self.registry.insert(coord.0, layer_2);
                Ok(())
            },
        }
    }

    /// Like `set` but doesn't give any result, rather it logs when there's an error and carries on.
    pub fn set_uncaring(&mut self, coord: ChunkCoordinate, to: Option<Entity>) {
        match self.set(coord, to) {
            Ok(_) => {},
            Err(error) => { warn!("Failed to set chunk coordinate at {:?}: {:?}", coord, error) },
        }
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