/// A unique, short numerical identifier for a block, created by a [BlockRegistryBuilder](super::registry::BlockRegistryBuilder).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(pub(super) u16);