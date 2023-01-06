use std::marker::PhantomData;

use bevy::{prelude::*, ecs::system::SystemParam};
use self::{
    chunk::registry::ChunkRegistry,
    block::{
        registry::BlockRegistry,
        Block
    }
};

pub mod block;
pub mod chunk;

#[derive(SystemParam)]
pub struct WorldMap<'w, 's> {
    block_registry: Res<'w, BlockRegistry>,
    chunk_registry: Res<'w, ChunkRegistry>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl WorldMap<'_, '_> {
    /// Gets a block from any coordinates in the world. Returns `Some` if the chunk is loaded, `None` if not.
    pub fn get_block(&self, pos: IVec3) -> Option<Block> {
        None
    }
}