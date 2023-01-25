use std::{collections::BTreeMap, sync::{Arc, RwLock}};
use bevy::{prelude::*, render::once_cell::sync::Lazy};
use super::{BlockId, data::BlockData};

pub static BLOCK_REGISTRY: Lazy<Arc<RwLock<BlockRegistryInternal>>> = Lazy::new(||{Arc::new(RwLock::new(BlockRegistryInternal::new()))});

#[derive(Resource)]
pub struct Blocks(Arc<RwLock<BlockRegistryInternal>>);

impl Blocks {
    pub fn add_block_type(&self, block: BlockData) {
        self.0.write().unwrap().add_block_type(block);
    }
}

impl Default for Blocks {
    fn default() -> Self {
        Self(BLOCK_REGISTRY.clone())
    }
}

pub struct BlockRegistryInternal {
    last_idx: u32,
    data_map: BTreeMap<BlockId, BlockData>,
    name_map: BTreeMap<String, BlockId>,
}

impl BlockRegistryInternal {
    pub(crate) fn new() -> Self {
        Self {
            last_idx: 0,
            data_map: BTreeMap::new(),
            name_map: BTreeMap::new(),
        }
    }

    pub fn add_block_type(&mut self, block: BlockData) {
        // Check for collisions
        for (_key, value) in self.data_map.iter() {
            if value.string_identifier == block.string_identifier {
                panic!("Block string ID collision occurred for \"{}\"", block.string_identifier);
            }
        }

        let id = BlockId(self.last_idx as u16);
        match block.get_attribute(BlockData::ATTRIBUTE_DISPLAY_NAME) {
            Some(name) => {
                info!("Added block {} ({:?}) under id {:?}", block.string_identifier, name, id);
            },
            None => {
                info!("Added block {} under id {:?}", block.string_identifier, id);
            },
        }

        self.name_map.insert(block.string_identifier.to_owned(), id);
        self.data_map.insert(id, block);
        self.last_idx += 1;
    }
    
    pub fn get_by_numerical_id(&self, id: BlockId) -> Option<&BlockData> {
        self.data_map.get(&id)
    }

    pub fn get_by_string_id(&self, id: String) -> Option<&BlockData> {
        match self.name_map.get(&id) {
            Some(id) => {
                self.data_map.get(id)
            },
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.data_map.len()
    }
}