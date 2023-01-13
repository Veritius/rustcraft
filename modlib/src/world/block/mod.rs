use bevy::prelude::Entity;

pub mod entity;
pub mod traits;
pub mod registry;

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Generic(BlockId),
    Entity(Entity),
}

impl Block {
    pub fn empty() -> Self {
        Self::Generic(BlockId(0))
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::Generic(BlockId(0))
    }
}

impl From<BlockId> for Block {
    fn from(value: BlockId) -> Self {
        Self::Generic(value)
    }
}

impl From<Entity> for Block {
    fn from(value: Entity) -> Self {
        Self::Entity(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(pub u16);