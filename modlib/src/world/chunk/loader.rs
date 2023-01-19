use bevy::prelude::*;

#[derive(Component)]
pub struct ChunkLoader {
    distance: f32,
}

impl Default for ChunkLoader {
    fn default() -> Self {
        Self {
            distance: 16.0,
        }
    }
}