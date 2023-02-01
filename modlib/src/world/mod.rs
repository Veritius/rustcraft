//! World map systems and traits.

use bevy::{prelude::*, ecs::system::SystemParam};
use crate::debug::{DebugMenuOpen, AppendDebugMenuMessage};
use self::{
    chunk::{registry::{Chunks, ChunkCoordinate, ChunkState}, Chunk, meshing::BeingRemeshed},
    block::{
        entity::BlockComponent,
        Block,
        registry::Blocks,
    },
    generation::biome::registry::{BiomesInternal, Biomes},
};

pub mod block;
pub mod chunk;
pub mod generation;
pub mod render;

/// Helpful tools and shortcuts for manipulating the world.
#[derive(SystemParam)]
pub struct WorldMapHelpers<'w, 's> {
    pub block_registry: Res<'w, Blocks>,
    pub chunk_registry: Res<'w, Chunks>,
    pub biome_registry: Res<'w, Biomes>,
    pub blocks: Query<'w, 's, (Entity, &'static BlockComponent)>,
    pub chunks: Query<'w, 's, (Entity, &'static Chunk)>,
}

impl WorldMapHelpers<'_, '_> {
    /// Gets a block from any coordinates in the world. Returns `Some` if the chunk is loaded, `None` if not.
    pub fn get_block(&self, pos: IVec3) -> Option<Block> {
        let chunk_offset = pos / 16;
        let block_offset = (pos + IVec3::splat(16)) % 16;

        let chunk = match self.chunk_registry.get(chunk_offset.into()) {
            ChunkState::Present(entity) => entity,
            _ => { return None }
        };

        let chunk = self.chunks.get(chunk).expect("Chunk was in registry but not in query!").1;
        Some(chunk.get_block(block_offset.x as usize, block_offset.y as usize, block_offset.z as usize).clone())
    }

    pub fn get_chunk(&self, coord: ChunkCoordinate) -> Option<&Chunk> {
        match self.chunk_registry.get(coord) {
            ChunkState::Present(entity) => {
                match self.chunks.get(entity) {
                    Ok(query_result) => Some(query_result.1),
                    Err(_) => None,
                }
            },
            _ => None,
        }
    }
}