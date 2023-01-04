use bevy::prelude::Entity;

pub mod defaults;
pub mod entity;
pub mod traits;
pub mod registry;

pub enum Block {
    Entity(Entity),
    Generic(BlockId),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(u16);

impl BlockId {
    pub const EMPTY: BlockId = BlockId(0);
}

impl Default for BlockId {
    fn default() -> Self {
        Self::EMPTY
    }
}