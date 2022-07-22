use bevy::app::{App, Plugin};

mod chunk;
mod voxel;
mod events;
mod mesh;

use voxel::VoxelDataTable;
use chunk::ChunkManager;
use events::*;

/// An implementation for a voxel world system
pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        //No multi-world/dimensions support (yet).
        app.init_resource::<VoxelDataTable>();
        app.init_resource::<ChunkManager>();
        
        //TODO: Put this in events.rs
        app.add_event::<BlockUpdateEvent>();
        app.add_event::<BlockRemovalAttemptEvent>();
        app.add_event::<BlockRemovalEvent>();
        app.add_event::<BlockPlacementAttemptEvent>();
        app.add_event::<BlockPlacementEvent>();
        app.add_event::<BlockReplacementAttemptEvent>();
        app.add_event::<BlockReplacementEvent>();
    }
}