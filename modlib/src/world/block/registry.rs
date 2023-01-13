use std::{collections::BTreeMap, ops::Deref};
use bevy::prelude::{Resource, info, Color};
use crate::world::chunk::meshing::MeshingVisibility;

use super::{traits::BlockDefinition, BlockId};

#[derive(Resource)]
pub struct BlockRegistry {
    last_assigned_id: u16,
    registry: BTreeMap<BlockId, Box<dyn BlockDefinition>>,
}

impl Clone for BlockRegistry {
    fn clone(&self) -> Self {
        let mut new_registry: BTreeMap<BlockId, Box<dyn BlockDefinition>> = BTreeMap::new();

        for entry in self.registry.iter() {
            new_registry.insert(*entry.0, dyn_clone::clone_box(&**entry.1));
        }

        Self {
            last_assigned_id: self.last_assigned_id.clone(),
            registry: new_registry,
        }
    }
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut v_self = Self {
            last_assigned_id: 0,
            registry: BTreeMap::new(),
        };
        v_self.register_new::<Air>();
        v_self
    }
    
    pub fn register_new<T: 'static + BlockDefinition>(&mut self) -> BlockId {
        let new_def = T::new();

        // Check for collisions
        for (_key, value) in self.registry.iter() {
            if value.deref().str_id() == new_def.str_id() {
                panic!("Block string ID collision occurred for \"{}\"", new_def.str_id());
            }
        }

        let id = BlockId(self.last_assigned_id);
        info!("Registered new block {} ({}) under {}", new_def.name(), new_def.str_id(), id.0);
        self.registry.insert(id, Box::new(new_def));
        self.last_assigned_id += 1;
        return id;
    }

    pub fn get_by_id(&self, id: BlockId) -> Option<&Box<dyn BlockDefinition>> {
        self.registry.get(&id)
    }

    pub fn get_by_type<T: BlockDefinition>(&self) -> Option<BlockId> {
        for (key, value) in self.registry.iter() {
            if value.deref().str_id() == T::new().str_id() {
                return Some(*key)
            }
        }
        return None
    }

    pub fn get_inner_registry(&self) -> &BTreeMap<BlockId, Box<dyn BlockDefinition>> {
        &self.registry
    }
}

#[derive(Clone, Copy)]
pub struct Air;
impl BlockDefinition for Air {
    fn new() -> Self where Self: Sized { Self {} }
    fn str_id(&self) -> &'static str { "engine_empty" }
    fn name(&self) -> &'static str { "Air" }
    fn color(&self) -> Color { Color::NONE }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Invisible }
}

pub type Empty = Air;