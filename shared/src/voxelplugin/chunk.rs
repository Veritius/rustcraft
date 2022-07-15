use bevy_ecs::{component::Component, entity::Entity};

/// A world's chunk table and methods for interacting with voxels
pub struct ChunkManager {

}

impl Default for ChunkManager {
    fn default() -> ChunkManager {
        ChunkManager { }
    }
}

pub struct Chunk {
    entity: Entity //World representaton of the chunk
}

#[derive(Component)]
/// A tag component for a chunk entity
struct ChunkComponent {
    
}