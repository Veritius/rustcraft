use bevy::{prelude::*, ecs::system::SystemParam};
use self::{
    chunk::{registry::{ChunkRegistry, ChunkCoordinate, ChunkState}, Chunk},
    block::{
        registry::BlockRegistry,
        Block, entity::BlockEntity
    }
};

pub mod block;
pub mod chunk;
pub mod generation;

#[derive(SystemParam)]
pub struct WorldMapHelpers<'w, 's> {
    block_registry: Res<'w, BlockRegistry>,
    chunk_registry: Res<'w, ChunkRegistry>,
    blocks: Query<'w, 's, (Entity, &'static BlockEntity)>,
    chunks: Query<'w, 's, (Entity, &'static Chunk)>,
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

    pub fn get_chunk_or_none(&self, coord: ChunkCoordinate) -> Option<&Chunk> {
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