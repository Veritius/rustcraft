use bevy::prelude::{Bundle, PbrBundle, MaterialMeshBundle, Material};
use super::Chunk;

#[derive(Bundle)]
pub struct ChunkBundle<M: Material> {
    pub chunk: Chunk,
    pub pbr: MaterialMeshBundle<M>,
}