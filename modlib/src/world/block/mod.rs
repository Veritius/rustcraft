use bevy::prelude::*;
use self::registry::Blocks;

pub mod entity;
pub mod data;
pub mod registry;

pub struct BlockRegistryPlugin;
impl Plugin for BlockRegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let block_registry = Blocks::default();
        block_registry.add_block_type(data::air_block());
        app.insert_resource(block_registry);
    }
}

#[derive(Clone, Copy)]
pub enum Block {
    Generic(BlockId),
    Entity(Entity),
}

impl Block {
    pub const EMPTY: Block = Block::Generic(BlockId::EMPTY);
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