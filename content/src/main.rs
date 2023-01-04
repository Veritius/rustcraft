use bevy::{prelude::App, DefaultPlugins};
use blocks::{Dirt, Stone};
use rustcraft_modlib::{BlockRegistryPlugin, block::traits::AddBlock};

pub mod blocks;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(BlockRegistryPlugin);

    app.add_block::<Dirt>();
    app.add_block::<Stone>();

    app.run();
}
