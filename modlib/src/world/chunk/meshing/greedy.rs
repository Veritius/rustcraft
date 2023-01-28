use crate::world::{chunk::CHUNK_SIZE, block::{BlockId, registry::BlockRegistryInternal}};

/// Somewhat flexible greedy meshing algorithm. Operates over a 2D slice of `BlockId` objects to generate a set of quads.
/// Each quad is a single `BlockId` type. The algorithm will not create a quad that would contain multiple BlockIds. Quads will always be rectangular and will never create quads that overlap.
/// 
/// Takes the following arguments:
/// - A 2D 'slice' of the chunk (not in the Rust sense) that will be looped over.
/// - A reference to a BlockRegistryInternal to use for comparisons.
/// - A `Fn(&BlockId, &BlockRegistryInternal) -> bool` (called the Selector) object to check if a block should be meshed.
/// 
/// Implementation of the greedy meshing algorithm based on the following resources.
/// - https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/s
/// - https://devforum.roblox.com/t/consume-everything-how-greedy-meshing-works/452717
#[doc(hidden)]
pub fn greedy_determine_quads<Selector: Fn(&BlockId, &BlockRegistryInternal) -> bool>(slice: &[[BlockId; CHUNK_SIZE]; CHUNK_SIZE], registry: &BlockRegistryInternal, selector: Selector) -> Vec<(BlockId, [u8; 4])> {
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
                if slice[check_x][block_y] != current_block || !selector(&current_block, registry) {
                    break;
                }
                offset_x += 1;
            }

            // Check columns
            let mut offset_y = 0;
            'column_checker: for check_y in block_y..CHUNK_SIZE {
                for b in block_x..block_x + offset_x {
                    if occupied[b][check_y] || slice[b][check_y] != current_block || !selector(&current_block, registry) {
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