use bevy::{
    prelude::*,
    DefaultPlugins,
    pbr::wireframe::{
        WireframePlugin, WireframeConfig
    }
};
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};
use rustcraft_modlib::{
    world::{
        block::{data::AddBlock, BlockRegistryPlugin},
        generation::{WorldGenPlugin, WorldGenExtensionFns, noise::SimpleNoiseLayer2D},
        chunk::{events::LoadChunkMessage, ChunkedWorldPlugin},
    },
    debug::DebugMenuPlugin, noise_rs::Perlin
};
use worldgen::{noise::{NOISE_LAYER_HEIGHT, NOISE_LAYER_TEMPERATURE, NOISE_LAYER_HUMIDITY}, scorers::BaseSelectionScorer, passes::BaseTerrainPass};

pub mod blocks;
pub mod biomes;
pub mod worldgen;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(DebugMenuPlugin);
    app.add_plugin(WireframePlugin);
    app.add_plugin(NoCameraPlayerPlugin);
    
    app.add_plugin(BlockRegistryPlugin);
    app.add_plugin(ChunkedWorldPlugin);
    app.add_plugin(WorldGenPlugin);

    app.add_block(blocks::defs::water());
    app.add_block(blocks::defs::dirt());
    app.add_block(blocks::defs::stone());
    app.add_block(blocks::defs::sand());
    app.add_block(blocks::defs::grass());
    app.add_block(blocks::defs::glass());

    app.add_biome(biomes::defs::ocean());
    app.add_biome(biomes::defs::plains());
    app.add_biome(biomes::defs::forest());
    app.add_biome(biomes::defs::jungle());
    app.add_biome(biomes::defs::desert());
    app.add_biome(biomes::defs::tundra());

    app.add_noise_layer(NOISE_LAYER_HEIGHT.to_owned(), SimpleNoiseLayer2D::new(2524123412, vec![
        (5.0, Perlin::new(0), 0.029592342),
        (7.0, Perlin::new(0), 0.008732425),
        (10.0, Perlin::new(0), 0.003241255),
    ]));
    app.add_noise_layer(NOISE_LAYER_TEMPERATURE.to_owned(), SimpleNoiseLayer2D::new(42512352, vec![
        (5.0, Perlin::new(0), 0.029592342),
        (7.0, Perlin::new(0), 0.008732425),
        (10.0, Perlin::new(0), 0.003241255),
    ]));
    app.add_noise_layer(NOISE_LAYER_HUMIDITY.to_owned(), SimpleNoiseLayer2D::new(1235212379, vec![
        (5.0, Perlin::new(0), 0.029592342),
        (7.0, Perlin::new(0), 0.008732425),
        (10.0, Perlin::new(0), 0.003241255),
    ]));

    app.add_biome_scorer(BaseSelectionScorer);

    app.add_world_generator_pass(BaseTerrainPass);

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
    for x in -16..16 {
        for y in -4..4 {
            for z in -16..16 {
                event_writer.send(LoadChunkMessage(IVec3 { x, y, z }));
            }
        }
    }
}