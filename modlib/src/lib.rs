use bevy::prelude::Plugin;
use block::registry::BlockRegistry;
use chunk::{registry::ChunkRegistry, meshing::remesh_chunk_system};

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
    }
} 