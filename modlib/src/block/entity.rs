use bevy::prelude::Component;
use block_mesh::VoxelVisibility;

#[derive(Component)]
pub struct BlockEntity {
    pub visibility: VoxelVisibility,
}