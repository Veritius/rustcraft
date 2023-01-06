use bevy::prelude::Component;
use crate::world::chunk::meshing::MeshingVisibility;

#[derive(Component)]
pub struct BlockEntity {
    pub visibility: MeshingVisibility,
}