use std::collections::BTreeMap;
use bevy::prelude::{Resource, Entity};

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
}

#[derive(Debug)]
pub enum ChunkOperationError {
    NoChunkInPosition(ChunkCoordinate),
    ChunkAlreadyPresent(ChunkCoordinate),
}