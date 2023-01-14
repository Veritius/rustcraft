use crate::world::block::{BlockId, registry::BlockRegistry};

use super::SHAPE_SIZE_USIZE;

pub(super) fn greedy_mesh(
    array: &[[[BlockId; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE],
    registry: &BlockRegistry,
) {
    
}