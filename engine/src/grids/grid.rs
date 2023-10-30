use bevy::prelude::*;

/// A grid of voxels.
#[derive(Component)]
pub struct Grid {
    /// If `true`, the chunking system will never unload this grid's chunks.
    pub keep_loaded: bool,
}