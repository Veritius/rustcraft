use bevy::prelude::{Entity, IVec3, EventWriter, Changed, Query};

use super::Chunk;

/// Raise to unload a chunk
pub struct UnloadChunkMessage(pub Entity);


/// Raise to load a chunk
pub struct LoadChunkMessage(pub IVec3);

/// Raised when a chunk is modified
pub struct ChunkModifiedEvent(pub IVec3);

pub(crate) fn chunk_change_system(
    query: Query<&Chunk, Changed<Chunk>>,
    mut events: EventWriter<ChunkModifiedEvent>,
) {
    for chunk in query.iter() {
        events.send(ChunkModifiedEvent(chunk.position.into()));
    }
}