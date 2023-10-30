use std::{sync::Arc, collections::BTreeMap};
use bevy::{prelude::*, utils::HashMap};
use crate::content::id::{Identifier, NamespacedIdentifier};
use super::{id::BlockId, attributes::{BlockAttribute, AttributeIdentifier}};

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
        ident: NamespacedIdentifier,
        attributes: impl Iterator<Item = (AttributeIdentifier, Box<dyn BlockAttribute>)>
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
        ident: &NamespacedIdentifier,
        attribute: (AttributeIdentifier, Box<dyn BlockAttribute>),
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
    identifiers: HashMap<NamespacedIdentifier, BlockId>,
    definitions: BTreeMap<BlockId, BlockDefinition>
}

pub struct BlockDefinition {
    attributes: BTreeMap<AttributeIdentifier, Box<dyn BlockAttribute>>,
}