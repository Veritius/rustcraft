use bevy::prelude::{Bundle, PbrBundle};
use super::Chunk;

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: Chunk,
    pub pbr: PbrBundle,
}