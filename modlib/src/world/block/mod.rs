use bevy::prelude::Entity;

pub mod entity;
pub mod traits;
pub mod registry;

#[derive(Debug, Default, Clone, Copy)]
pub enum Block {
    #[default]
    Empty,
    Entity(Entity),
    Generic(BlockId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(pub u16);

impl BlockId {
    pub const EMPTY: BlockId = BlockId(0);
}

impl Default for BlockId {
    fn default() -> Self {
        Self::EMPTY
    }
}