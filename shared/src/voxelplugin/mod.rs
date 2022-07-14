use bevy_app::{App, Plugin};
use bevy_ecs::prelude::{World, FromWorld};

mod chunk;
mod table;
mod voxel;
mod events;

use table::{VoxelDataTable, VoxelDataTableEntry};
use chunk::Chunk;
use events::*;

/// An implementation for a voxel world system
pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalVoxelData>();
        
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

/// Globally accessible voxel plugin related information
struct GlobalVoxelData {
    pub table: VoxelDataTable,
    //chunks: Chunk, //TODO: make this a 3d array, placeholder atm
}

impl Default for GlobalVoxelData {
    fn default() -> Self {
        let table = VoxelDataTable::new();
        //let chunks = Chunk {};

        GlobalVoxelData { table }//, chunks }
    }
}