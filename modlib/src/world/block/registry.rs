use std::{collections::BTreeMap, ops::Deref};
use bevy::prelude::Resource;
use super::{traits::BlockDefinition, BlockId};

#[derive(Resource)]
pub struct BlockRegistry {
    last_assigned_id: u16,
    registry: BTreeMap<BlockId, Box<dyn BlockDefinition>>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self {
            last_assigned_id: 0,
            registry: BTreeMap::new(),
        }
    }
    
    pub fn register_new<T: 'static + BlockDefinition>(&mut self) -> BlockId {
        let new_def = T::new();

        // Check for collisions
        for (_key, value) in self.registry.iter() {
            if value.deref().id() == new_def.id() {
                panic!("Block ID collision occurred for \"{}\"", new_def.id());
            }
        }

        let id = BlockId(self.last_assigned_id);
        self.registry.insert(id, Box::new(new_def));
        self.last_assigned_id += 1;
        return id;
    }

    pub fn get_by_id(&self, id: BlockId) -> Option<&Box<dyn BlockDefinition>> {
        self.registry.get(&id)
    }

    pub fn get_by_type<T: BlockDefinition>(&self) -> Option<BlockId> {
        for (key, value) in self.registry.iter() {
            if value.deref().id() == T::new().id() {
                return Some(*key)
            }
        }
        return None
    }
}