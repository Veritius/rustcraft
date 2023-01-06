use bevy::prelude::{Plugin, Commands, ResMut, PbrBundle};
use world::{
    block::registry::BlockRegistry,
    chunk::{registry::ChunkRegistry, meshing::{remesh_chunk_system, RemeshChunkMarker}, bundle::ChunkBundle, Chunk}, WorldMapHelpers};

pub mod world;

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
    }
}