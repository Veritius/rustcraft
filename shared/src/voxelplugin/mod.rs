use bevy_app::{App, Plugin};
use bevy_ecs::prelude::{World, FromWorld};

mod chunk;
mod table;
mod voxel;

use table::{VoxelDataTable, VoxelDataTableEntry};
use chunk::Chunk;

/// An implementation for a voxel world system
pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalVoxelData>();
    }
}

/// Globally accessible voxel plugin related information
struct GlobalVoxelData {
    table: VoxelDataTable,
    //chunks: Chunk, //TODO: make this a 3d array, placeholder atm
}

impl FromWorld for GlobalVoxelData {
    fn from_world(world: &mut World) -> Self {
        let table = VoxelDataTable {};
        let air_block = VoxelDataTableEntry {
            string_id: "air",
            name: "Empty",
            opaque: false
        };
        table.add_block_type(air_block);
        //let chunks = Chunk {};

        GlobalVoxelData { table }//, chunks }
    }
}

impl GlobalVoxelData {
    /// not implemented
    fn get_voxel_table() {
        
    }
}