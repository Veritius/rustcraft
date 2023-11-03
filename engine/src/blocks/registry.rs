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
    /// 
    /// Panics if `ident` is already assigned.
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

    /// Registers `alias` to refer to the same `BlockId` as `ident`.
    /// 
    /// If `ident` does not exist, this will silently fail.
    /// This behavior may change in future.
    pub fn add_alias(
        &mut self,
        ident: &ContentIdentifier,
        alias: ContentIdentifier,
    ) {
        let id = self.inner.identifiers.get(ident);
        if id.is_none() { return }
        self.inner.identifiers.insert(alias, *id.unwrap());
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
    #[inline]
    pub fn block_exists(&self, id: &ContentIdentifier) -> bool {
        self.0.block_exists(id)
    }

    #[inline]
    pub fn block_id(&self, id: &ContentIdentifier) -> Option<BlockId> {
        self.0.block_id(id)
    }

    #[inline]
    pub fn get_definition(&self, id: &ContentIdentifier) -> Option<&BlockDefinition> {
        self.0.get_definition(id)
    }
}

/// Storage for block data.
pub(super) struct BlockRegistryInner {
    identifiers: HashMap<ContentIdentifier, BlockId>,
    definitions: BTreeMap<BlockId, BlockDefinition>
}

impl BlockRegistryInner {
    pub fn block_exists(&self, id: &ContentIdentifier) -> bool {
        self.identifiers.contains_key(id)
    }

    pub fn block_id(&self, id: &ContentIdentifier) -> Option<BlockId> {
        self.identifiers.get(id).cloned()
    }

    pub fn get_definition(&self, id: &ContentIdentifier) -> Option<&BlockDefinition> {
        let id = self.block_id(id)?;
        self.definitions.get(&id)
    }
}

#[derive(Debug)]
pub struct BlockDefinition {
    attributes: Attributes,
}