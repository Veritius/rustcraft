/// A unique, short numerical identifier for a block, created by a [`BlockRegistryBuilder`](super::registry::BlockRegistryBuilder).
/// A `BlockId` is only valid when using the `BlockRegistry` that created it. Using it with another registry may return unexpected results or panic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(pub(super) u16);