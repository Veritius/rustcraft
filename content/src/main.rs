use bevy::{prelude::App, DefaultPlugins};
use bevy_flycam::PlayerPlugin;
use blocks::{Dirt, Stone};
use rustcraft_modlib::{BlockRegistryPlugin, world::block::traits::AddBlock, ChunkedWorldPlugin};

pub mod blocks;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(PlayerPlugin);
    
    app.add_plugin(BlockRegistryPlugin);
    app.add_plugin(ChunkedWorldPlugin);

    app.add_block::<Dirt>();
    app.add_block::<Stone>();

    app.run();
}
