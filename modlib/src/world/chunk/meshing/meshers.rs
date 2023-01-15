use bevy::prelude::Mesh;
use crate::world::block::{BlockId, registry::BlockRegistry, Block};

use super::{SHAPE_SIZE_USIZE, MeshingVisibility};

// pub(super) fn simple_mesh(
//     mesh: &mut Mesh,
//     array: &[[[BlockId; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE],
//     registry: &BlockRegistry,
// ) {
//     let mut positions = vec![];
//     let mut 
// }

/// Greedy meshing algorithm based on the following resources:
/// - https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/ 
/// - https://devforum.roblox.com/t/consume-everything-how-greedy-meshing-works/452717
pub(super) fn greedy_mesh(
    mesh: &mut Mesh,
    array: &[[[BlockId; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE],
    registry: &BlockRegistry,
) {
    // TODO: This can be optimised by not creating slices (not the Rust kind) of the array and instead accessing it better. Maybe with ndarray?

    // Left and right
    for x in 1..SHAPE_SIZE_USIZE-1 {
        let mut left_slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        let mut right_slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        for y in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array[x][y][z];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x-1][y][z], registry)) {
                    left_slice[y][z] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x+1][y][z], registry)) {
                    right_slice[y][z] = this_block;
                }
            }
        }
    }

    // Top and bottom
    for y in 1..SHAPE_SIZE_USIZE-1 {
        let mut top_slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        let mut bottom_slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        for x in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array[x][y][z];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y-1][z], registry)) {
                    top_slice[y][z] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y+1][z], registry)) {
                    bottom_slice[y][z] = this_block;
                }
            }
        }
    }

    // Forward and back
    for z in 1..SHAPE_SIZE_USIZE-1 {
        let mut fwd_slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        let mut back_slice = [[BlockId::EMPTY; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];
        for x in 1..SHAPE_SIZE_USIZE-1 {
            for y in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array[x][y][z];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y][z-1], registry)) {
                    fwd_slice[y][z] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y][z+1], registry)) {
                    back_slice[y][z] = this_block;
                }
            }
        }
    }
}

fn greedy_determine_quads(slice: &[[BlockId; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE]) -> Vec<(BlockId, [u8; 4])> {
    let mut quads = vec![];
    let mut visited_mask = [[false; SHAPE_SIZE_USIZE]; SHAPE_SIZE_USIZE];

    for x in 0..SHAPE_SIZE_USIZE-1 {
        for y in 0..SHAPE_SIZE_USIZE-1 {
            if visited_mask[x][y] == true { continue; }

            visited_mask[x][y] = true;
        }
    }

    quads
}

fn get_visibility(block: BlockId, registry: &BlockRegistry) -> MeshingVisibility {
    match registry.get_by_id(block) {
        Some(entry) => entry.visibility(),
        None => MeshingVisibility::Invisible,
    }
}