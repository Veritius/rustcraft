use std::any::Any;
use bevy::prelude::*;
use crate::blocks::id::BlockId;

/// An object that can be associated with a voxel in a chunk.
pub trait BlockAssociatedData: Any + Send + Sync {}
impl<T: Any + Send + Sync> BlockAssociatedData for T {}

/// A chunk's size in all dimensions.
pub const CHUNK_SIZE: usize = 16;

#[test]
fn size_validity_test() {
    assert!(CHUNK_SIZE.pow(3) <= u16::MAX as usize,
        "CHUNK_SIZE constant exceeded 2^16. Due to how information is associated with voxels in a chunk, this could make the chunk unable to store unique information for all blocks.");
}

/// A cube of voxels used that is loaded/unloaded in bulk.
#[derive(Component)]
pub struct Chunk {
    table: [[[ChunkBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    data: Vec<BlockData>,
}

/// An entry in a [`Chunk`]'s array.
#[derive(Debug)]
struct ChunkBlock {
    /// The simple ID used to access data from the [BlockRegistry](crate::blocks::registry::BlockRegistry).
    block: BlockId,
    /// An ID that is used to access extra information stored in the Chunk.
    /// If equal to zero, this can be thought of as having no attached information.
    // This is implemented like this to increase the information density of the array.
    // If this was a full size pointer or Option<Box<T>>, there would be a lot of wasted space.
    ptr: u16,
}

enum BlockData {
    EntityId(Entity),
    DynBox(Box<dyn BlockAssociatedData>),
}