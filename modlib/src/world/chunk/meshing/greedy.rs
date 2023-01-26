use crate::world::{chunk::CHUNK_SIZE, block::BlockId};

/// Greedy meshing algorithm based on the following resources:
/// - https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/
/// - https://devforum.roblox.com/t/consume-everything-how-greedy-meshing-works/452717
#[doc(hidden)]
pub fn greedy_determine_quads(slice: &[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]) -> Vec<(BlockId, [u8; 4])> {
    let mut quads = vec![];
    let mut occupied = [[false; CHUNK_SIZE]; CHUNK_SIZE];

    // Iterate each block
    for block_x in 0..CHUNK_SIZE {
        for block_y in 0..CHUNK_SIZE {
            // Skip the block if it's already occupied by a quad or it's empty
            if occupied[block_x][block_y] || slice[block_x][block_y] == BlockId::EMPTY {
                continue;
            }

            // Remember our current block type
            let current_block = slice[block_x][block_y];

            // Check rows
            let mut offset_x = 0;
            for check_x in block_x..CHUNK_SIZE {
                if slice[check_x][block_y] != current_block {
                    break;
                }
                offset_x += 1;
            }

            // Check columns
            let mut offset_y = 0;
            'column_checker: for check_y in block_y..CHUNK_SIZE {
                for b in block_x..block_x + offset_x {
                    if occupied[b][check_y] || slice[b][check_y] != current_block {
                        break 'column_checker;
                    }
                }
                offset_y += 1;
            }

            // Mark blocks as occupied
            for occupied_x in block_x..block_x + offset_x {
                for occupied_y in block_y..block_y + offset_y {
                    occupied[occupied_x][occupied_y] = true;
                }
            }

            quads.push((
                current_block,
                [
                    block_x as u8,
                    block_y as u8,
                    (block_x + offset_x) as u8,
                    (block_y + offset_y) as u8,
                ],
            ));
        }
    }

    quads
}