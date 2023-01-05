use bevy::prelude::{Bundle, PbrBundle};

use super::Chunk;

#[derive(Bundle)]
pub struct ChunkBundle {
    chunk: Chunk,
    pbr: PbrBundle,
}