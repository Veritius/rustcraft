use std::{collections::BTreeMap};
use bevy::prelude::{Resource, info};

use super::{BlockId, data::BlockData};

#[derive(Resource, Clone)]
pub struct BlockRegistry {
    id_idx: u16,
    data_map: BTreeMap<BlockId, BlockData>,
    name_map: BTreeMap<String, BlockId>,
}

impl BlockRegistry {
    pub const fn new() -> Self {
        Self {
            id_idx: 0,
            data_map: BTreeMap::new(),
            name_map: BTreeMap::new(),
        }
    }
    
    pub fn add_block_type(&mut self, block: BlockData) -> BlockId {
        // Check for collisions
        for (_key, value) in self.data_map.iter() {
            if value.string_identifier == block.string_identifier {
                panic!("Block string ID collision occurred for \"{}\"", block.string_identifier);
            }
        }

        let id = BlockId(self.id_idx);
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
        self.id_idx += 1;
        return id;
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