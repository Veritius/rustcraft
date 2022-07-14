use bevy_ecs::{component::Component, entity::Entity};

pub struct Chunk {
    entity: Entity //World representaton of the chunk
}

#[derive(Component)]
/// A tag component for a chunk entity
struct ChunkComponent {
    
}