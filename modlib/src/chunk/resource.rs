use std::collections::BTreeMap;
use bevy::prelude::{Resource, Entity};

type ChunkCoordinate = (u32, u32, u32);

#[derive(Resource)]
pub struct ChunkRegistry {
    registry: BTreeMap<u32, BTreeMap<u32, BTreeMap<u32, Entity>>>,
}

impl ChunkRegistry {
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
                        return Err(ChunkOperationError::NoChunkInPosition(coord))
                    },
                }
            },
            None => { return Err(ChunkOperationError::NoChunkInPosition(coord)) },
        }
    }
}

pub enum ChunkOperationError {
    NoChunkInPosition(ChunkCoordinate),
    ChunkAlreadyPresent(ChunkCoordinate),
}