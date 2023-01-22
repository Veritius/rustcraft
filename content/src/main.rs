use bevy::{
    prelude::*,
    DefaultPlugins,
    pbr::wireframe::{
        WireframePlugin, WireframeConfig
    }
};
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};
use rustcraft_modlib::{
    BlockRegistryPlugin,
    world::{
        block::data::AddBlock,
        generation::{WorldGenPlugin, biome::AddBiome},
        chunk::events::LoadChunkMessage,
    },
    ChunkedWorldPlugin,
    debug::DebugMenuPlugin
};

pub mod blocks;
pub mod biomes;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(DebugMenuPlugin);
    app.add_plugin(WireframePlugin);
    app.add_plugin(NoCameraPlayerPlugin);
    
    app.add_plugin(BlockRegistryPlugin);
    app.add_plugin(ChunkedWorldPlugin);
    app.add_plugin(WorldGenPlugin);

    app.add_block(blocks::water());
    app.add_block(blocks::dirt());
    app.add_block(blocks::stone());
    app.add_block(blocks::sand());
    app.add_block(blocks::grass());
    app.add_block(blocks::glass());

    app.add_biome(biomes::ocean());
    app.add_biome(biomes::plains());
    app.add_biome(biomes::forest());
    app.add_biome(biomes::jungle());
    app.add_biome(biomes::desert());
    app.add_biome(biomes::tundra());

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
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 128.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZ, 10.0, 40.0, 70.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Camera3dBundle::default(),
        FlyCam,
    ));
    for x in -32..32 {
        for y in -8..8 {
            for z in -32..32 {
                event_writer.send(LoadChunkMessage(IVec3 { x, y, z }));
            }
        }
    }
}