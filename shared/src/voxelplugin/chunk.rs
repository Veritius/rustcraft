use std::{collections::{BTreeMap, btree_map}, ops::{Rem, Div}};
use bevy::{ecs::{component::Component, entity::Entity}, prelude::Commands};
use ndarray::Array3;
use super::voxel::Voxel;

pub type ChunkPosition = (i32, i32, i32);

/// A world's chunk table and methods for interacting with voxels
pub struct ChunkManager {
    pub chunks: ChunkTable
}

impl Default for ChunkManager {
    fn default() -> ChunkManager {
        let chunks = ChunkTable::new();
        ChunkManager { chunks }
    }
}

pub struct ChunkTable {
    pub tbl: BTreeMap<i32, BTreeMap<i32, BTreeMap<i32, Option<Chunk>>>>
}

impl ChunkTable {
    pub fn new() -> ChunkTable {
        let tbl = BTreeMap::new();
        ChunkTable { tbl }
    }

    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> &Option<Chunk> {
        let one = self.tbl.get(&x);
        match one {
            Some(two_uv) => {
                let two = two_uv.get(&y);
                match two {
                    Some(three_uv) => {
                        let three = three_uv.get(&z);
                        match three {
                            Some(result) => { return result; }
                            None => { return &None; }
                        }
                    }
                    None => { return &None; }
                }
            }
            None => { return &None; }
        }
    }

    pub fn get_chunk_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Option<Chunk>> {
        let one = self.tbl.get_mut(&x);
        match one {
            Some(two_uv) => {
                let two = two_uv.get_mut(&y);
                match two {
                    Some(three_uv) => {
                        let three = three_uv.get_mut(&z);
                        match three {
                            Some(result) => { return Some(result); }
                            None => { return None; }
                        }
                    }
                    None => { return None; }
                }
            }
            None => { return None; }
        }
    }

    pub fn add_chunk(&mut self, mut commands: Commands, x: i32, y: i32, z: i32) {
        let mut entcommands = commands.spawn();
        let entity = entcommands.id();

        let mut chunk = Chunk {
            entity,
            voxels: [[[None; 16]; 16]; 16]
        };
        
        entcommands.insert(ChunkComponent { chunk: (x, y, z) });
        let mut tz = BTreeMap::new();
        tz.insert(z, Some(chunk));
        let mut ty = BTreeMap::new();
        ty.insert(y, tz);
        self.tbl.insert(x, ty);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Chunk {
    /// World representaton of the chunk
    pub entity: Entity,
    pub voxels: [[[Option<Voxel>; 16]; 16]; 16]
}

impl Chunk {
    // TODO: Make this return a Result like this: Result<Option<Voxel>, Err>
    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> Option<Voxel> {
        let xa = self.voxels.into_iter().nth(x);
        match xa {
            Some(xav) => {
                let ya = xav.into_iter().nth(y);
                match ya {
                    Some(yav) => {
                        let za = yav.into_iter().nth(z);
                        match za {
                            Some(zav) => {
                                return zav;
                            }
                            None => { return None; }
                        }
                    }
                    None => { return None; }
                }
            }
            None => { return None; }
        }
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, to: Option<Voxel>) {
        let xa = self.voxels.into_iter().nth(x);
        match xa {
            Some(xav) => {
                let ya = xav.into_iter().nth(y);
                match ya {
                    Some(mut yav) => {
                        yav[z] = to;
                    }
                    None => { return; }
                }
            }
            None => { return; }
        }
    }
}

#[derive(Component)]
/// Chunk information
struct ChunkComponent {
    pub chunk: ChunkPosition
}