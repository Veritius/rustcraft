use std::{sync::Arc, collections::BTreeMap};
use bevy::{prelude::*, utils::HashMap};
use crate::{content::id::ContentIdentifier, attributes::{value::Attribute, map::Attributes}};
use super::id::{BlockId, BlockIdGenerator};

/// Used during the setup stage of the game to create the [BlockRegistry].
#[derive(Resource)]
pub struct BlockRegistryBuilder {
    seq: BlockIdGenerator,
    inner: BlockRegistryInner,
}

impl BlockRegistryBuilder {
    /// Registers a block.
    /// Panics if the block already exists
    pub fn add_block(
        &mut self,
        ident: ContentIdentifier,
        attributes: impl Iterator<Item = (ContentIdentifier, Attribute)>
    ) {
        let id = self.seq.next();
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
        attribute: (ContentIdentifier, Attribute),
    ) {
        let id = self.inner.identifiers.get(ident)
            .expect(&format!("Block {ident} was not registered"));
        self.inner.definitions.get_mut(id).unwrap()
            .attributes.insert(attribute.0, attribute.1);
    }
}

#[derive(Resource, Clone)]
pub struct BlockRegistry(Arc<BlockRegistryInner>);

impl BlockRegistry {
    pub fn block_exists(&self, id: ContentIdentifier) -> bool {
        self.0.identifiers.contains_key(&id)
    }

    pub fn get_definition(&self, id: ContentIdentifier) -> Option<&BlockDefinition> {
        let id = self.0.identifiers.get(&id)?;
        self.0.definitions.get(id)
    }
}

/// Storage for block data.
struct BlockRegistryInner {
    identifiers: HashMap<ContentIdentifier, BlockId>,
    definitions: BTreeMap<BlockId, BlockDefinition>
}

#[derive(Debug)]
pub struct BlockDefinition {
    attributes: Attributes,
}