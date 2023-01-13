use bevy::prelude::Plugin;
use world::{
    block::registry::BlockRegistry,
    chunk::{
        registry::ChunkRegistry,
        meshing::remesh_chunk_system, loader::{LoadChunkMessage, UnloadChunkMessage},
    }
};

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
        app.add_event::<UnloadChunkMessage>();
        app.add_event::<LoadChunkMessage>();
        
        app.insert_resource(ChunkRegistry::new());
        app.add_system(remesh_chunk_system);
    }
}