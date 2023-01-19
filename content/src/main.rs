use bevy::{
    prelude::*,
    DefaultPlugins,
    pbr::wireframe::{
        WireframePlugin, WireframeConfig
    }
};
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};
use blocks::*;
use rustcraft_modlib::{
    BlockRegistryPlugin,
    world::{
        block::traits::AddBlock,
        generation::WorldGenPlugin,
        chunk::events::LoadChunkMessage,
    },
    ChunkedWorldPlugin,
    debug::DebugMenuPlugin
};

pub mod blocks;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(DebugMenuPlugin);
    app.add_plugin(WireframePlugin);
    app.add_plugin(NoCameraPlayerPlugin);
    
    app.add_plugin(BlockRegistryPlugin);
    app.add_plugin(ChunkedWorldPlugin);
    app.add_plugin(WorldGenPlugin);

    app.add_block::<Grass>();
    app.add_block::<Dirt>();
    app.add_block::<Sand>();
    app.add_block::<Stone>();

    app.add_system(wireframe_toggle_system);

    app.add_startup_system(funny_startup_system);

    app.run();
}

fn wireframe_toggle_system(
    keys: Res<Input<KeyCode>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    if keys.just_pressed(KeyCode::H) {
        if wireframe_config.global {
            wireframe_config.global = false;
        } else {
            wireframe_config.global = true;
        }
    }
}

fn funny_startup_system(
    mut commands: Commands,
    mut event_writer: EventWriter<LoadChunkMessage>,
) {
    commands.spawn((
        Camera3dBundle::default(),
        FlyCam,
        PointLight {
            color: Color::WHITE,
            intensity: 1000.0,
            range: 10000.0,
            // radius: todo!(),
            ..default()
        },
    ));
    for x in -8..8 {
        for y in -4..4 {
            for z in -8..8 {
                event_writer.send(LoadChunkMessage(IVec3 { x, y, z }));
            }
        }
    }
}