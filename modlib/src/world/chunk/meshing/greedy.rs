use bevy::{prelude::Mesh, render::mesh::Indices};
use ndarray::{Array3, Axis};
use crate::world::{block::{BlockId, registry::BlockRegistry, Block}, chunk::{CHUNK_SIZE, CHUNK_SIZE_U8}};

use super::{SHAPE_SIZE_USIZE, MeshingVisibility};

// pub(super) fn simple_mesh(
//     mesh: &mut Mesh,
//     array: &Array3<BlockId>,
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
    array: &Array3<BlockId>,
    registry: &BlockRegistry,
) {
    // TODO: This can be optimised by not copying data in the array passed in arguments. Possibly use subviews from ndarray?

    let mut positions = vec![];
    // let mut normals = vec![];
    // let mut uvs = vec![];

    // Left and right
    for x in 1..SHAPE_SIZE_USIZE-1 {
        let array_subview = array.index_axis(Axis(0), x);
        let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        for y in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array_subview[[y, z]];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[[x-1, y, z]], registry)) {
                    left_slice[y-1][z-1] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[[x+1, y, z]], registry)) {
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
        for (_blockid, quad) in greedy_determine_quads(&right_slice) {
            positions.extend([
               [x as f32, quad[0] as f32, quad[1] as f32],
               [x as f32, quad[0] as f32, quad[3] as f32],
               [x as f32, quad[2] as f32, quad[1] as f32],
               [x as f32, quad[0] as f32, quad[3] as f32],
               [x as f32, quad[2] as f32, quad[3] as f32],
               [x as f32, quad[2] as f32, quad[1] as f32],
            ]);
        }
    }

    // Up and down
    for y in 1..SHAPE_SIZE_USIZE-1 {
        let array_subview = array.index_axis(Axis(1), y);
        let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 1..SHAPE_SIZE_USIZE-1 {
            for z in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array_subview[[x, z]];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[[x, y-1, z]], registry)) {
                    left_slice[x-1][z-1] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[[x, y+1, z]], registry)) {
                    right_slice[x-1][z-1] = this_block;
                }
            }
        }

        let y = y - 1;

        for (_blockid, quad) in greedy_determine_quads(&left_slice) {
            positions.extend([
                [quad[0] as f32, y as f32, quad[1] as f32],
                [quad[0] as f32, y as f32, quad[3] as f32],
                [quad[2] as f32, y as f32, quad[1] as f32],
                [quad[0] as f32, y as f32, quad[3] as f32],
                [quad[2] as f32, y as f32, quad[3] as f32],
                [quad[2] as f32, y as f32, quad[1] as f32],
            ]);
        }
        for (_blockid, quad) in greedy_determine_quads(&right_slice) {
            positions.extend([
                [quad[0] as f32, y as f32, quad[1] as f32],
                [quad[0] as f32, y as f32, quad[3] as f32],
                [quad[2] as f32, y as f32, quad[1] as f32],
                [quad[0] as f32, y as f32, quad[3] as f32],
                [quad[2] as f32, y as f32, quad[3] as f32],
                [quad[2] as f32, y as f32, quad[1] as f32],
            ]);
        }
    }

    // Forward and backward
    for z in 1..SHAPE_SIZE_USIZE-1 {
        let array_subview = array.index_axis(Axis(2), z);
        let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 1..SHAPE_SIZE_USIZE-1 {
            for y in 1..SHAPE_SIZE_USIZE-1 {
                let this_block = array_subview[[x, y]];
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[[x, y, z-1]], registry)) {
                    left_slice[x-1][y-1] = this_block;
                }
                if get_visibility(this_block, registry).is_visible_against(&get_visibility(array[[x, y, z+1]], registry)) {
                    right_slice[x-1][y-1] = this_block;
                }
            }
        }

        let z = z - 1;

        for (_blockid, quad) in greedy_determine_quads(&left_slice) {
            positions.extend([
               [quad[0] as f32, quad[1] as f32, z as f32],
               [quad[0] as f32, quad[3] as f32, z as f32],
               [quad[2] as f32, quad[1] as f32, z as f32],
               [quad[0] as f32, quad[3] as f32, z as f32],
               [quad[2] as f32, quad[3] as f32, z as f32],
               [quad[2] as f32, quad[1] as f32, z as f32],
            ]);
        }
        for (_blockid, quad) in greedy_determine_quads(&right_slice) {
            positions.extend([
               [quad[0] as f32, quad[1] as f32, z as f32],
               [quad[0] as f32, quad[3] as f32, z as f32],
               [quad[2] as f32, quad[1] as f32, z as f32],
               [quad[0] as f32, quad[3] as f32, z as f32],
               [quad[2] as f32, quad[3] as f32, z as f32],
               [quad[2] as f32, quad[1] as f32, z as f32],
            ]);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
}

#[doc(hidden)]
fn greedy_determine_quads(slice: &[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]) -> Vec<(BlockId, [u8; 4])> {
    let mut quads = vec![];
    let mut occupied = [[false; CHUNK_SIZE]; CHUNK_SIZE];

    // Iterate each block
    for block_x in 0..CHUNK_SIZE {
        for block_y in 0..CHUNK_SIZE {
            // Skip the block if it's already occupied by a quad or it's empty
            if occupied[block_x][block_y] || slice[block_x][block_y] == BlockId::EMPTY { continue; }

            // Remember our current block type
            let current_block = slice[block_x][block_y];

            // Check rows
            let mut offset_x = 0;
            for check_x in block_x..CHUNK_SIZE {
                if slice[check_x][block_y] != current_block { break; }
                offset_x += 1;
            }

            // Check columns
            let mut offset_y = 0;
            'column_checker: for check_y in block_y..CHUNK_SIZE {
                for b in block_x..block_x+offset_x {
                    if occupied[b][check_y] || slice[b][check_y] != current_block { break 'column_checker; }
                }
                offset_y += 1;
            }

            // Mark blocks as occupied
            for occupied_x in block_x..block_x+offset_x {
                for occupied_y in block_y..block_y+offset_y {
                    occupied[occupied_x][occupied_y] = true;
                }
            }

            quads.push((current_block, [block_x as u8, block_y as u8, (block_x+offset_x) as u8, (block_y+offset_y) as u8]));
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