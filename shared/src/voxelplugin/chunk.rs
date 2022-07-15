use bevy_ecs::{component::Component, entity::Entity};

/// A world's chunk table
pub struct ChunkTable {

}

impl Default for ChunkTable {
    fn default() -> ChunkTable {
        ChunkTable { }
    }
}

pub struct Chunk {
    entity: Entity //World representaton of the chunk
}

#[derive(Component)]
/// A tag component for a chunk entity
struct ChunkComponent {
    
}