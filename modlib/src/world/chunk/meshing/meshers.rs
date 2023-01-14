use bevy::prelude::Mesh;
use crate::world::block::{BlockId, registry::BlockRegistry};

use super::{SHAPE_SIZE_USIZE, MeshingVisibility};

// pub(super) fn simple_mesh(
//     mesh: &mut Mesh,
//     array: &[[[BlockId; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE],
//     registry: &BlockRegistry,
// ) {
//     let mut positions = vec![];
//     let mut 
// }

pub(super) fn greedy_mesh(
    mesh: &mut Mesh,
    array: &[[[BlockId; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE],
    registry: &BlockRegistry,
) {
    // Left
    for x in 1..SHAPE_SIZE_USIZE-1 {
        let mut slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        for y in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let b_cand = array[x][y][z];
                if get_visibility(b_cand, registry).is_visible_against(&get_visibility(array[x-1][y][z], registry)) {
                    slice[y][z] = b_cand;
                }
            }
        }
    }

    // Right
    for x in 1..SHAPE_SIZE_USIZE-1 {
        let mut slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        for y in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let b_cand = array[x][y][z];
                if get_visibility(b_cand, registry).is_visible_against(&get_visibility(array[x+1][y][z], registry)) {
                    slice[y][z] = b_cand;
                }
            }
        }
    }
}

fn get_visibility(block: BlockId, registry: &BlockRegistry) -> MeshingVisibility {
    match registry.get_by_id(block) {
        Some(entry) => entry.visibility(),
        None => MeshingVisibility::Invisible,
    }
}