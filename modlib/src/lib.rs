use bevy::prelude::Plugin;
use block::{registry::BlockRegistry, traits::AddBlock, defaults::Air};

pub mod block;

pub struct BlockRegistryPlugin;
impl Plugin for BlockRegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BlockRegistry::new());
        app.add_block::<Air>();
    }
}