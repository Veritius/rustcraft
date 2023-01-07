use bevy::prelude::{App, ResMut, Color};
use crate::world::chunk::meshing::MeshingVisibility;
use super::registry::BlockRegistry;

pub trait BlockDefinition: 'static + Send + Sync {
    fn new() -> Self where Self: Sized;
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn color(&self) -> Color;
    fn visibility(&self) -> MeshingVisibility;
}

pub trait AddBlock {
    fn add_block<T: BlockDefinition>(&mut self) -> &mut Self;
}

impl AddBlock for App {
    fn add_block<T: BlockDefinition>(&mut self) -> &mut Self {
        self.add_startup_system(|mut registry: ResMut<BlockRegistry>| {
            registry.register_new::<T>();
        });

        self
    }
}