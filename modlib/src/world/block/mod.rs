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