use bevy::prelude::Plugin;
use block::registry::BlockRegistry;

pub mod block;
pub mod chunk;

pub struct BlockRegistryPlugin;
impl Plugin for BlockRegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BlockRegistry::new());
    }
}