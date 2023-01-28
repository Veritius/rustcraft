use bevy::prelude::Component;
use crate::world::chunk::meshing::MeshingVisibility;

use super::BlockId;

/// Allows blocks to be stored in a chunk. Stores a `BlockId` for rapid block-to-block comparisons and asynchronous access.
#[derive(Component)]
pub struct BlockComponent(pub BlockId);