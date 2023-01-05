use std::collections::BTreeMap;
use bevy::prelude::{Resource, Entity};

use super::Chunk;

type ChunkCoordinate = (u32, u32, u32);

#[derive(Resource)]
pub struct ChunkRegistry {
    registry: BTreeMap<u32, BTreeMap<u32, BTreeMap<u32, Option<Entity>>>>,
}

impl ChunkRegistry {
    pub fn get(&self, coord: ChunkCoordinate) -> Result<&Option<Entity>, ChunkOperationError> {
        match self.registry.get(&coord.0) {
            Some(one) => {
                match one.get(&coord.1) {
                    Some(two) => {
                        match two.get(&coord.2) {
                            Some(chunk) => {
                                return Ok(chunk)
                            },
                            None => {
                                return Err(ChunkOperationError::NoChunkInPosition(coord))
                            },
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

    fn get_mut(&mut self, coord: ChunkCoordinate) -> Result<&mut Option<Entity>, ChunkOperationError> {
        match self.registry.get_mut(&coord.0) {
            Some(one) => {
                match one.get_mut(&coord.1) {
                    Some(two) => {
                        match two.get_mut(&coord.2) {
                            Some(chunk) => {
                                return Ok(chunk)
                            },
                            None => {
                                return Err(ChunkOperationError::NoChunkInPosition(coord))
                            },
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

    pub fn assign_chunk(&mut self, coord: ChunkCoordinate, to: Option<Entity>) -> Result<(), ChunkOperationError> {
        match self.get_mut(coord) {
            Ok(value) => {
                *value = to;
                return Ok(())
            },
            Err(error) => { return Err(error) },
        }
    }

    pub fn clear_chunk(&mut self, coord: ChunkCoordinate) -> Result<(), ChunkOperationError> {
        self.assign_chunk(coord, None)
    }
}

pub enum ChunkOperationError {
    NoChunkInPosition(ChunkCoordinate)
}