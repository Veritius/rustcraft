/// A table of voxel information
pub struct VoxelDataTable {
    table: Vec<VoxelDataTableEntry>
}

impl VoxelDataTable {
    pub fn new() -> VoxelDataTable {
        let mut table = Vec::<VoxelDataTableEntry>::new();
        /// Ensure index 0 is always "empty"
        let air = VoxelDataTableEntry {
            string_id: "air".to_string(),
            name: "Air".to_string(),
            opaque: false
        };
        table.push(air);

        return VoxelDataTable { table };
    }

    ///Get the flyweighted voxel information for a specific integer voxel id
    pub fn get_block_data(self, id: u16) -> Option<VoxelDataTableEntry> {
        // TODO: Replace with try_get
        self.table.into_iter().nth(id.into())
    }

    ///Get the flyweighted voxel information for a specific string voxel id
    pub fn get_block_data_by_string(self, id: &str) -> Option<VoxelDataTableEntry> {
        for i in self.table {
            if i.string_id == id {
                return Some(i);
            }
        }
        return None;
    }

    ///Create a new type of voxel in the table. This cannot be undone.
    pub fn add_block_type(&mut self, data: VoxelDataTableEntry) -> u16 {
        self.table.push(data);
        // If the maximum length of 65535 (u16 max) is exceeded, this will panic.
        // I don't care enough to add decent handling, because
        // if you're reaching this number, what the fuck are you doing.
        return self.table.len().try_into().unwrap();
    }
}

///Flyweighted information for a voxel
pub struct VoxelDataTableEntry {
    // These two should probably be a str, not a String, but I don't understand lifetimes so
    pub string_id: String,
    pub name: String,
    /// Don't create adjacent faces
    pub opaque: bool,
}