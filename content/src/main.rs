use bevy::{prelude::{App, Commands, ResMut, PbrBundle}, DefaultPlugins};
use bevy_flycam::PlayerPlugin;
use blocks::{Dirt, Stone};
use rand::Rng;
use rustcraft_modlib::{BlockRegistryPlugin, world::{block::{traits::AddBlock, BlockId, Block}, chunk::{registry::ChunkRegistry, Chunk, bundle::ChunkBundle, meshing::RemeshChunkMarker}}, ChunkedWorldPlugin};

pub mod blocks;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(PlayerPlugin);
    
    app.add_plugin(BlockRegistryPlugin);
    app.add_plugin(ChunkedWorldPlugin);

    app.add_block::<Dirt>();
    app.add_block::<Stone>();

    app.add_startup_system(sus_entry_system);

    app.run();
}

fn sus_entry_system(
    mut commands: Commands,
    mut registry: ResMut<ChunkRegistry>,
) {
    let mut chunk = Chunk::new((0,0,0));
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let mut rng = rand::thread_rng();
                let block = match rng.gen_range(0..20) {
                    5 => Block::Generic(BlockId(1)),
                    _ => Block::Empty,
                    // 2 => Block::Generic(BlockId(2)),
                    // _ => panic!(),
                };
                chunk.set_block(x as usize, y as usize, z as usize, block);
            }
        }
    }

    let id = commands.spawn((ChunkBundle { chunk, pbr: PbrBundle::default() }, RemeshChunkMarker)).id();
    registry.set((0,0,0), Some(id)).unwrap();
}