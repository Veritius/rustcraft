use bevy_app::{App, Plugin};

mod chunk;
mod table;
mod voxel;
mod events;

use table::VoxelDataTable;
use events::*;

/// An implementation for a voxel world system
pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelDataTable>();
        
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