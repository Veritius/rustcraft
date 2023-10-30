use std::{sync::Arc, collections::BTreeMap};
use bevy::prelude::*;
use super::{id::BlockId, attributes::{BlockAttribute, AttributeIdentifier}};

/// Used during the setup stage of the game to create the [BlockRegistry].
#[derive(Resource)]
pub struct BlockRegistryBuilder {
    seq: u16,
    inner: BlockRegistryInner,
}

#[derive(Resource, Clone)]
pub struct BlockRegistry(Arc<BlockRegistryInner>);

/// Storage for block data.
struct BlockRegistryInner(BTreeMap<BlockId, BlockDefinition>);

struct BlockDefinition {
    pub id: BlockId,
    pub attributes: BTreeMap<AttributeIdentifier, Box<dyn BlockAttribute>>,
}