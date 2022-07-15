use super::voxel::Voxel;

/// Event fired off when a block updates
pub struct BlockUpdateEvent {

}

/// Cancellable event fired off when a block is attempted to be removed
pub struct BlockRemovalAttemptEvent {
    /// Set to `true` to cancel the removal
    cancelled: bool,
    obj: Voxel,
}

/// Event fired off when a block is removed
pub struct BlockRemovalEvent {
    obj: Voxel
}

/// Cancellable event fired off when a block is attempted to be placed
pub struct BlockPlacementAttemptEvent {
    /// Set to `true` to cancel the placement
    cancelled: bool,
    obj: Voxel,
}

/// Event fired off when a block is placed
pub struct BlockPlacementEvent {
    obj: Voxel,
}

/// Cancellable event fired off when a block is attempted to be replaced with another
pub struct BlockReplacementAttemptEvent {
    /// Set to `true` to cancel the replacement
    cancelled: bool,
    new: Voxel,
    old: Voxel,
}

/// Event fired off when a block is replaced with another
pub struct BlockReplacementEvent {
    new: Voxel,
    old: Voxel,
}