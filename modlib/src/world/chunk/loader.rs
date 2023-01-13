use bevy::prelude::*;

/// Raise to unload a chunk
pub struct UnloadChunkMessage(pub Entity);


/// Raise to load a chunk
pub struct LoadChunkMessage(pub IVec3);

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