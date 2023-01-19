use bevy::prelude::{Plugin, IntoSystemDescriptor};
use rand::Rng;
use world::{
    block::registry::BlockRegistry,
    chunk::{
        registry::ChunkRegistry,
        meshing::{
            chunk_remesh_dispatch_system,
            chunk_remesh_polling_system,
            remesh_changed_chunks_system,
        },
        events::{
            UnloadChunkMessage,
            LoadChunkMessage,
            ChunkModifiedEvent,
            chunk_change_system
        },
        SystemLabels as ChunkSystemLabels,
    },
    debug_data_system,
};

pub mod world;
pub mod debug;

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
        app.add_event::<ChunkModifiedEvent>();

        app.add_system(chunk_change_system
            .label(ChunkSystemLabels::ChunkChangeEventSystem));
        app.insert_resource(ChunkRegistry::new());
        app.add_system(chunk_remesh_dispatch_system
            .label(ChunkSystemLabels::ChunkMeshingDispatchSystem));
        app.add_system(chunk_remesh_polling_system
            .label(ChunkSystemLabels::ChunkMeshingPollingSystem)
            .after(ChunkSystemLabels::ChunkMeshingDispatchSystem));
        app.add_system(remesh_changed_chunks_system
            .after(ChunkSystemLabels::ChunkChangeEventSystem));

        app.add_system(debug_data_system
            .before(debug::SystemLabels::DebugMenuDisplaySystem));
    }
}