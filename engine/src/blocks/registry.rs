use std::{sync::Arc, collections::BTreeMap, any::Any};
use bevy::{prelude::*, utils::HashMap};
use crate::content::id::ContentIdentifier;
use super::id::BlockId;

/// A value that is added to a [BlockDefinition] in the [BlockRegistry] to define behaviors for a block..
pub trait BlockAttribute: Send + Sync + Any {}
impl<T: Send + Sync + Any> BlockAttribute for T {}

/// Used during the setup stage of the game to create the [BlockRegistry].
#[derive(Resource)]
pub struct BlockRegistryBuilder {
    seq: u16,
    inner: BlockRegistryInner,
}

impl BlockRegistryBuilder {
    /// Registers a block.
    /// Panics if the block already exists
    pub fn add_block(
        &mut self,
        ident: ContentIdentifier,
        attributes: impl Iterator<Item = (ContentIdentifier, Box<dyn BlockAttribute>)>
    ) {
        let id = BlockId(self.seq);
        self.seq += 1;
        self.inner.identifiers.insert(ident, id)
            .expect("Registry already contained block {ident}");
        self.inner.definitions.insert(id, BlockDefinition {
            attributes: attributes.collect()
        });
    }

    /// Adds an attribute to an already-existing block.
    /// Panics if `ident` isn't already registered.
    pub fn insert_attribute(
        &mut self,
        ident: &ContentIdentifier,
        attribute: (ContentIdentifier, Box<dyn BlockAttribute>),
    ) {
        let id = self.inner.identifiers.get(ident)
            .expect(&format!("Block {ident} was not registered"));
        self.inner.definitions.get_mut(id).unwrap()
            .attributes.insert(attribute.0, attribute.1);
    }
}

#[derive(Resource, Clone)]
pub struct BlockRegistry(Arc<BlockRegistryInner>);

/// Storage for block data.
struct BlockRegistryInner {
    identifiers: HashMap<ContentIdentifier, BlockId>,
    definitions: BTreeMap<BlockId, BlockDefinition>
}

pub struct BlockDefinition {
    attributes: BTreeMap<ContentIdentifier, Box<dyn BlockAttribute>>,
}