use bevy::{ecs::component::Component, prelude::Color};

/// A table of voxel information
pub struct VoxelDataTable {
    table: Vec<VoxelDataTableEntry>
}

impl Default for VoxelDataTable {
    fn default() -> VoxelDataTable {
        let table = Vec::<VoxelDataTableEntry>::new();
        return VoxelDataTable { table };
    }
}

impl VoxelDataTable {
    ///Get the flyweighted voxel information from a voxel object
    pub fn get_block_data(self, voxel: Voxel) -> Option<VoxelDataTableEntry> {
        // TODO: Replace with try_get
        let id: u16;
        match voxel {
            Voxel::SimpleVoxel(v) => { id = v.id; },
            Voxel::RotatableVoxel(v) => { id = v.id; },
            Voxel::EntityVoxel(v) => { id = v.id; },
        }
        self.table.into_iter().nth(id.into())
    }

    ///Get the flyweighted voxel information for a specific integer voxel id
    pub fn get_block_data_by_id(self, id: u16) -> Option<VoxelDataTableEntry> {
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
    /// Solid color, mostly for debugging
    pub color: Color,
    /// Don't create adjacent faces
    pub opaque: bool,
    /// Can water and temperature flow through this block
    pub permeable: bool,
    /// Can players/other physics objects move through this block
    pub solid: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Voxel {
    SimpleVoxel(SimpleVoxel),
    RotatableVoxel(RotatableVoxel),
    EntityVoxel(EntityVoxel), //make this an entity ref?
}

#[derive(Debug, Copy, Clone)]
pub struct SimpleVoxel {
    pub id: u16
}

#[derive(Debug, Copy, Clone)]
pub struct RotatableVoxel {
    pub id: u16,
    pub rot: bool //figure out what type should be used for 6 directional rotation
}

#[derive(Debug, Copy, Clone)]
#[derive(Component)]
pub struct EntityVoxel {
    pub id: u16
}