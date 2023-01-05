use bevy::prelude::{Plugin, Commands, ResMut, PbrBundle};
use block::registry::BlockRegistry;
use chunk::{registry::ChunkRegistry, meshing::{remesh_chunk_system, RemeshChunkMarker}, bundle::ChunkBundle, Chunk};

pub mod block;
pub mod chunk;

pub struct BlockRegistryPlugin;
impl Plugin for BlockRegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BlockRegistry::new());
    }
}

pub struct ChunkedWorldPlugin;
impl Plugin for ChunkedWorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ChunkRegistry::new());
        app.add_system(remesh_chunk_system);
        app.add_startup_system(sus_entry_system);
    }
}

fn sus_entry_system(
    mut commands: Commands,
    mut registry: ResMut<ChunkRegistry>,
) {
    let id = commands.spawn((ChunkBundle { chunk: Chunk::new((0,0,0)), pbr: PbrBundle::default() }, RemeshChunkMarker)).id();
    registry.set((0,0,0), Some(id)).unwrap();
}