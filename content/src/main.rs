use bevy::{
    prelude::*,
    DefaultPlugins,
    pbr::wireframe::{
        WireframePlugin, WireframeConfig
    }
};
use bevy_flycam::PlayerPlugin;
use blocks::{Dirt, Stone};
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

    app.add_block::<Dirt>();
    app.add_block::<Stone>();

    app.add_startup_system(sus_entry_system);

    app.run();
}

fn sus_entry_system(
    mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    mut registry: ResMut<ChunkRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = true;

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 48.0, 0.0),
        ..default()
    });

    let material = materials.add(Color::rgb(0.3, 0.6, 0.4).into());

    for coords in [(0,0,0), (0,1,0), (1,0,0), (0,0,1), (1,1,0)] {
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
            x: 16.0 * coords.0 as f32,
            y: 16.0 * coords.1 as f32,
            z: 16.0 * coords.2 as f32,
        };
        let id = commands.spawn((ChunkBundle { chunk, pbr }, RemeshChunkMarker)).id();
        registry.set(coords, Some(id)).unwrap();
    }
}