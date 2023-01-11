use bevy::{
    prelude::*,
    DefaultPlugins,
    pbr::wireframe::{
        WireframePlugin, WireframeConfig
    }
};
use bevy_flycam::PlayerPlugin;
use blocks::*;
use rand::Rng;
use rustcraft_modlib::{BlockRegistryPlugin, world::{block::{traits::AddBlock, BlockId, Block}, chunk::{registry::ChunkRegistry, Chunk, bundle::ChunkBundle, meshing::RemeshChunkMarker, CHUNK_SIZE}}, ChunkedWorldPlugin};

pub mod blocks;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(WireframePlugin);
    app.add_plugin(PlayerPlugin);
    
    app.add_plugin(BlockRegistryPlugin);
    app.add_plugin(ChunkedWorldPlugin);

    app.add_block::<Grass>();
    app.add_block::<Dirt>();
    app.add_block::<Sand>();
    app.add_block::<Stone>();

    app.add_startup_system(sus_entry_system);
    app.add_system(wireframe_toggle_system);

    app.run();
}

fn sus_entry_system(
    mut commands: Commands,
    mut registry: ResMut<ChunkRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE.into(),
        brightness: 0.25
    });

    commands.spawn(PointLightBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 19.0, 0.0),
            ..default()
        },
        point_light: PointLight {
            intensity: 100000.0,
            ..default()
        },
        ..default()
    });

    let material = materials.add(Color::rgb(0.3, 0.6, 0.4).into());

    for coords in [
        (1,0,1), (0,0,1), (-1,0,1),
        (1,0,0), (0,0,0), (-1,0,0),
        (1,0,-1), (0,0,-1), (-1,0,-1)] {
        let mut chunk = Chunk::new(coords);
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let mut rng = rand::thread_rng();
                    let block = match rng.gen_range(0..20) {
                        5 => Block::Generic(BlockId(1)),
                        _ => Block::Empty,
                        // 2 => Block::Generic(BlockId(2)),
                        // _ => panic!(),
                    };
                    chunk.set_block(x, y, z, block);
                }
            }
        }

        let mut pbr = PbrBundle::default();
        pbr.material = material.clone();
        pbr.transform.translation = Vec3 {
            x: CHUNK_SIZE as f32 * coords.0 as f32,
            y: CHUNK_SIZE as f32 * coords.1 as f32,
            z: CHUNK_SIZE as f32 * coords.2 as f32,
        };
        let id = commands.spawn((ChunkBundle { chunk, pbr }, RemeshChunkMarker)).id();
        registry.set(coords, Some(id)).unwrap();
    }
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