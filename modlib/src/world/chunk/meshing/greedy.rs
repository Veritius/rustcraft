use bevy::{prelude::Mesh, render::mesh::Indices};
use crate::world::{block::{BlockId, registry::BlockRegistry, Block}, chunk::{CHUNK_SIZE, CHUNK_SIZE_U8}};

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

    let mut positions = vec![];
    // let mut normals = vec![];
    // let mut uvs = vec![];

    // Left and right
    for x in 1..SHAPE_SIZE_USIZE-1 {
        let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        for y in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array[x][y][z];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x-1][y][z], registry)) {
                    left_slice[y-1][z-1] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x+1][y][z], registry)) {
                    right_slice[y-1][z-1] = this_block;
                }
            }
        }

        let x = x - 1;

        for (_blockid, quad) in greedy_determine_quads(&left_slice) {
            positions.extend([
               [x as f32, quad[0] as f32, quad[1] as f32],
               [x as f32, quad[0] as f32, quad[3] as f32],
               [x as f32, quad[2] as f32, quad[1] as f32],
               [x as f32, quad[0] as f32, quad[3] as f32],
               [x as f32, quad[2] as f32, quad[3] as f32],
               [x as f32, quad[2] as f32, quad[1] as f32],
            ]);
        }
        // for (blockid, quad) in greedy_determine_quads(&right_slice) {

        // }
    }

    // Top and bottom
    // for y in 1..SHAPE_SIZE_USIZE-1 {
    //     let mut top_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
    //     let mut btm_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
    //     for x in 1..SHAPE_SIZE_USIZE-1 {
    //         for z in 1..SHAPE_SIZE_USIZE-1 {
    //             let this_block = array[x][y][z];
    //             if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y-1][z], registry)) {
    //                 top_slice[y-1][z-1] = this_block;
    //             }
    //             if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y+1][z], registry)) {
    //                 btm_slice[y-1][z-1] = this_block;
    //             }
    //         }
    //     }

    //     greedy_determine_quads(&top_slice);
    //     greedy_determine_quads(&btm_slice);
    // }

    // Forward and back
    // for z in 1..SHAPE_SIZE_USIZE-1 {
    //     let mut fwd_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
    //     let mut back_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
    //     for x in 1..SHAPE_SIZE_USIZE-1 {
    //         for y in 1..SHAPE_SIZE_USIZE-1 {
    //             let this_block = array[x][y][z];
    //             if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y][z-1], registry)) {
    //                 fwd_slice[y-1][z-1] = this_block;
    //             }
    //             if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[x][y][z+1], registry)) {
    //                 back_slice[y-1][z-1] = this_block;
    //             }
    //         }
    //     }

    //     greedy_determine_quads(&fwd_slice);
    //     greedy_determine_quads(&back_slice);
    // }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
}

#[doc(hidden)]
fn greedy_determine_quads(slice: &[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]) -> Vec<(BlockId, [u8; 4])> {
    let mut quads = vec![];
    let mut occupied = [[false; CHUNK_SIZE]; CHUNK_SIZE];

    // Iterate each block
    for block_x in 0..CHUNK_SIZE {
        for block_y in 0..CHUNK_SIZE {
            // Check we're not already using this block and that we're not empty
            if occupied[block_x][block_y] || slice[block_x][block_y] == BlockId::EMPTY { continue; }

            // Memorise what block we're currently on
            let current_block_id = slice[block_x][block_y];

            // Expand along x
            let mut offset_x = 0;
            for check_x in 0..CHUNK_SIZE-block_x {
                if slice[block_x+check_x][block_y] != current_block_id || occupied[block_x+check_x][block_y] { break; }

                offset_x += 1;
            }

            // Expand along y
            let mut offset_y = 0;
            'check_y: for check_y in 0..CHUNK_SIZE-block_y {
                // Loop over the row and check it's valid
                for r_offset_x in 0..offset_x {
                    if occupied[block_x+r_offset_x][check_y] || slice[block_x+r_offset_x][check_y] != current_block_id {
                        break 'check_y;
                    }
                }

                offset_y += 1;
            }

            // Mark the new quad as occupied
            for vx in block_x..block_x+offset_x {
                for vy in block_y..block_y+offset_y {
                    occupied[vx][vy] = true;
                }
            }

            // Add to list of quads
            quads.push((current_block_id, [block_x as u8, block_y as u8, (block_x+offset_x) as u8, (block_y+offset_y) as u8]));
        }
    }

    if quads.len() != 0 { println!("quads: {:?}", quads); }

    quads
}

fn get_visibility(block: BlockId, registry: &BlockRegistry) -> MeshingVisibility {
    match registry.get_by_id(block) {
        Some(entry) => entry.visibility(),
        None => MeshingVisibility::Invisible,
    }
}