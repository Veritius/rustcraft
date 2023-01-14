use bevy::prelude::Component;
use crate::world::chunk::meshing::MeshingVisibility;

use super::BlockId;

// Block entities store a
#[derive(Component)]
pub enum BlockEntity {
    Custom {
        visibility: MeshingVisibility,
    },
    Generic(BlockId),
}