use std::{collections::BTreeMap, sync::Arc};
use bevy::prelude::*;
use super::{BlockId, data::BlockData};

#[derive(Resource)]
pub struct BlockRegistryStartupBuffer {
    id_idx: u16,
    internal: BlockRegistryInternal,
}

impl BlockRegistryStartupBuffer {
    pub(crate) fn new() -> Self {
        Self {
            id_idx: 0,
            internal: BlockRegistryInternal::new(),
        }
    }

    pub fn add_block_type(&mut self, block: BlockData) -> BlockId {
        // Check for collisions
        for (_key, value) in self.internal.data_map.iter() {
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

        self.internal.name_map.insert(block.string_identifier.to_owned(), id);
        self.internal.data_map.insert(id, block);
        self.id_idx += 1;
        return id;
    }
}

#[derive(Resource)]
pub struct BlockRegistry {
    internal: Arc<BlockRegistryInternal>,
}

impl BlockRegistry {
    pub(crate) fn new(internal: BlockRegistryInternal) -> Self {
        Self {
            internal: Arc::new(internal),
        }
    }

    /// Clones the Arc storing the internal registry and passes a new clone as output.
    pub fn get_internal_registry(&self) -> Arc<BlockRegistryInternal> {
        self.internal.clone()
    }
}

#[derive(Clone)]
pub struct BlockRegistryInternal {
    data_map: BTreeMap<BlockId, BlockData>,
    name_map: BTreeMap<String, BlockId>,
}

impl BlockRegistryInternal {
    pub(crate) fn new() -> Self {
        Self {
            data_map: BTreeMap::new(),
            name_map: BTreeMap::new(),
        }
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

pub(crate) fn block_buffer_transfer_system(
    mut commands: Commands,
    buffer: Res<BlockRegistryStartupBuffer>,
) {
    commands.insert_resource(BlockRegistry::new(buffer.internal.clone()));
    commands.remove_resource::<BlockRegistryStartupBuffer>();
}