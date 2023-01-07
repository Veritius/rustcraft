use std::marker::PhantomData;

use bevy::{prelude::*, ecs::system::SystemParam};
use self::{
    chunk::{registry::ChunkRegistry, Chunk},
    block::{
        registry::BlockRegistry,
        Block, entity::BlockEntity
    }
};

pub mod block;
pub mod chunk;

#[derive(SystemParam)]
pub struct WorldMapHelpers<'w, 's> {
    block_registry: Res<'w, BlockRegistry>,
    chunk_registry: Res<'w, ChunkRegistry>,
    blocks: Query<'w, 's, (Entity, &'static BlockEntity)>,
    chunks: Query<'w, 's, (Entity, &'static Chunk)>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl WorldMapHelpers<'_, '_> {
    /// Gets a block from any coordinates in the world. Returns `Some` if the chunk is loaded, `None` if not.
    pub fn get_block(&self, pos: IVec3) -> Option<Block> {
        let chunk_offset = pos / 16;
        let block_offset = (pos + IVec3::splat(16)) % 16;

        let chunk = match self.chunk_registry.get(chunk_offset.into()) {
            Ok(chunk) => {
                match chunk {
                    Some(chunk) => chunk,
                    None => { return None },
                }
            },
            Err(_) => { return None },
        };

        let chunk = self.chunks.get(*chunk).expect("Chunk was in registry but not in query!").1;
        Some(chunk.get_block(block_offset.x as usize, block_offset.y as usize, block_offset.z as usize).clone())
    }
}