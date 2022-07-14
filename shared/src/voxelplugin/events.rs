use super::voxel::Voxel;

// TODO: All of these events should be able to be 'cancelled', a lá Robust Engine
// or find another solution allowing listeners to block events should be found

/// Event fired off when a block is removed
pub struct BlockRemovedEvent {
    obj: Voxel,
}

/// Event fired off when a block is placed in an empty voxel
pub struct BlockPlacedEvent {
    obj: Voxel
}

/// Event fired off when a block is replaced with another
pub struct BlockReplacedEvent {
    new: Voxel
}

/// Event fired off when a block updates
pub struct BlockUpdateEvent {

}