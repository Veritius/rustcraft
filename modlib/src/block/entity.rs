use bevy::prelude::Component;
use crate::chunk::meshing::MeshingVisibility;

#[derive(Component)]
pub struct BlockEntity {
    pub visibility: MeshingVisibility,
}