use std::{collections::BTreeMap, ops::{Rem, Div}};
use bevy::ecs::{component::Component, entity::Entity};
use ndarray::Array3;
use super::voxel::Voxel;

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
}

pub struct Chunk {
    /// World representaton of the chunk
    pub entity: Entity,
    pub voxels: [[[Voxel; 16]; 16]; 16]
}

#[derive(Component)]
/// Chunk information
struct ChunkComponent {}