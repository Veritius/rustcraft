/// A table of voxel information
/// Directly
pub struct VoxelDataTable {
    
}

impl VoxelDataTable {
    ///Get the flyweighted voxel information for a specific voxel id
    pub fn get_block_data(id: u16) {

    }

    ///Create a new type of voxel in the table. This cannot be undone.
    pub fn add_block_type(&self, data: VoxelDataTableEntry) -> u16 {
        0u16
    }
}

///Flyweighted information for a voxel
pub struct VoxelDataTableEntry<'a> {
    pub string_id: &'a str,
    pub name: &'a str,
    pub opaque: bool,
}