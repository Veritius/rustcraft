use crate::world::{
    block::{
        data::BlockData,
        registry::{BlockRegistryInternal, BLOCK_REGISTRY},
        Block, BlockId,
    },
    chunk::{CHUNK_SIZE, CHUNK_SIZE_U8, meshing::greedy::greedy_determine_quads},
};
use bevy::{
    prelude::{Color, Mesh},
    render::mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
};
use ndarray::{Array3, Axis};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLockReadGuard},
};
use super::{MeshingPass, MeshingVisibility, SHAPE_SIZE_USIZE, MeshingPassIdentifier};

pub const SOLID_BLOCK_MESHER_PASS: MeshingPassIdentifier = MeshingPassIdentifier::new("engine_solid", 0);

pub(crate) struct SolidBlockMesher;
impl MeshingPass for SolidBlockMesher {
    fn do_pass(
        &self,
        array: &Array3<BlockId>,
        positions: &mut Vec<[f32;3]>,
        normals: &mut Vec<[f32;3]>,
        uvs: &mut Vec<[f32;2]>,
        colors: &mut Vec<[f32;4]>,
        repeat: &mut Vec<[u32;2]>,
    ) {
        let registry = BLOCK_REGISTRY.read().unwrap();

        fn selector(block: &BlockId, registry: &BlockRegistryInternal) -> bool {
            registry.get_by_numerical_id(*block).unwrap().get_attribute(BlockData::ATTRIBUTE_USE_SOLID_MESHER).is_some()
        }

        /*
        (0,0) --- (1,0)
         |           |
         |           |
         |           |
         |           |
        (0,1) --- (1,1)

        A ---------- B
        |            |
        |            |
        |            |
        |            |
        C ---------- D
        */

        // Left and right
        for x in 1..SHAPE_SIZE_USIZE - 1 {
            let array_subview = array.index_axis(Axis(0), x);
            let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for y in 1..SHAPE_SIZE_USIZE - 1 {
                for z in 1..SHAPE_SIZE_USIZE - 1 {
                    let this_block = array_subview[[y, z]];
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x - 1, y, z]], &registry))
                    {
                        left_slice[y - 1][z - 1] = this_block;
                    }
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x + 1, y, z]], &registry))
                    {
                        right_slice[y - 1][z - 1] = this_block;
                    }
                }
            }

            let x = x - 1;

            for (blockid, quad) in greedy_determine_quads(&left_slice, &registry, selector) {
                positions.extend([
                    [x as f32, quad[0] as f32, quad[1] as f32],
                    [x as f32, quad[0] as f32, quad[3] as f32],
                    [x as f32, quad[2] as f32, quad[1] as f32],
                    [x as f32, quad[0] as f32, quad[3] as f32],
                    [x as f32, quad[2] as f32, quad[3] as f32],
                    [x as f32, quad[2] as f32, quad[1] as f32],
                ]);
                normals.extend([[1.0, 0.0, 0.0]; 6]);
                uvs.extend([
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [0.0, 0.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                ]);
                color_extend(colors, blockid, &registry);
                repeat.extend([[
                    ((quad[3]-quad[1])) as u32,
                    ((quad[2]-quad[0])) as u32,
                    ]; 6]);
            }
            for (blockid, quad) in greedy_determine_quads(&right_slice, &registry, selector) {
                positions.extend([
                    [x as f32 + 1.0, quad[0] as f32, quad[3] as f32],
                    [x as f32 + 1.0, quad[0] as f32, quad[1] as f32],
                    [x as f32 + 1.0, quad[2] as f32, quad[1] as f32],
                    [x as f32 + 1.0, quad[2] as f32, quad[3] as f32],
                    [x as f32 + 1.0, quad[0] as f32, quad[3] as f32],
                    [x as f32 + 1.0, quad[2] as f32, quad[1] as f32],
                ]);
                normals.extend([[-1.0, 0.0, 0.0]; 6]);
                uvs.extend([
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 0.0],
                ]);
                color_extend(colors, blockid, &registry);
                repeat.extend([[
                    ((quad[3]-quad[1])) as u32,
                    ((quad[2]-quad[0])) as u32,
                    ]; 6]);
            }
        }

        // Up and down
        for y in 1..SHAPE_SIZE_USIZE - 1 {
            let array_subview = array.index_axis(Axis(1), y);
            let mut bottom_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            let mut top_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for x in 1..SHAPE_SIZE_USIZE - 1 {
                for z in 1..SHAPE_SIZE_USIZE - 1 {
                    let this_block = array_subview[[x, z]];
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y - 1, z]], &registry))
                    {
                        bottom_slice[x - 1][z - 1] = this_block;
                    }
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y + 1, z]], &registry))
                    {
                        top_slice[x - 1][z - 1] = this_block;
                    }
                }
            }

            let y = y - 1;

            for (blockid, quad) in greedy_determine_quads(&bottom_slice, &registry, selector) {
                positions.extend([
                    [quad[0] as f32, y as f32, quad[3] as f32],
                    [quad[0] as f32, y as f32, quad[1] as f32],
                    [quad[2] as f32, y as f32, quad[1] as f32],
                    [quad[2] as f32, y as f32, quad[3] as f32],
                    [quad[0] as f32, y as f32, quad[3] as f32],
                    [quad[2] as f32, y as f32, quad[1] as f32],
                ]);
                normals.extend([[0.0, 1.0, 0.0]; 6]);
                uvs.extend([
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 0.0],
                ]);
                color_extend(colors, blockid, &registry);
                repeat.extend([[
                    ((quad[2]-quad[0])) as u32,
                    ((quad[3]-quad[1])) as u32,
                    ]; 6]);
            }
            for (blockid, quad) in greedy_determine_quads(&top_slice, &registry, selector) {
                positions.extend([
                    [quad[0] as f32, y as f32 + 1.0, quad[1] as f32],
                    [quad[0] as f32, y as f32 + 1.0, quad[3] as f32],
                    [quad[2] as f32, y as f32 + 1.0, quad[1] as f32],
                    [quad[0] as f32, y as f32 + 1.0, quad[3] as f32],
                    [quad[2] as f32, y as f32 + 1.0, quad[3] as f32],
                    [quad[2] as f32, y as f32 + 1.0, quad[1] as f32],
                ]);
                normals.extend([[0.0, -1.0, 0.0]; 6]);
                uvs.extend([
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                ]);
                color_extend(colors, blockid, &registry);
                repeat.extend([[
                    ((quad[2]-quad[0])) as u32,
                    ((quad[3]-quad[1])) as u32,
                    ]; 6]);
            }
        }

        // Forward and backward
        for z in 1..SHAPE_SIZE_USIZE - 1 {
            let array_subview = array.index_axis(Axis(2), z);
            let mut forward_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            let mut backward_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for x in 1..SHAPE_SIZE_USIZE - 1 {
                for y in 1..SHAPE_SIZE_USIZE - 1 {
                    let this_block = array_subview[[x, y]];
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y, z - 1]], &registry))
                    {
                        forward_slice[x - 1][y - 1] = this_block;
                    }
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y, z + 1]], &registry))
                    {
                        backward_slice[x - 1][y - 1] = this_block;
                    }
                }
            }

            let z = z - 1;

            for (blockid, quad) in greedy_determine_quads(&forward_slice, &registry, selector) {
                positions.extend([
                    [quad[0] as f32, quad[1] as f32, z as f32],
                    [quad[0] as f32, quad[3] as f32, z as f32],
                    [quad[2] as f32, quad[1] as f32, z as f32],
                    [quad[0] as f32, quad[3] as f32, z as f32],
                    [quad[2] as f32, quad[3] as f32, z as f32],
                    [quad[2] as f32, quad[1] as f32, z as f32],
                ]);
                normals.extend([[0.0, 0.0, 1.0]; 6]);
                uvs.extend([
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                ]);
                color_extend(colors, blockid, &registry);
                repeat.extend([[
                    ((quad[2]-quad[0])) as u32,
                    ((quad[3]-quad[1])) as u32,
                    ]; 6]);
            }
            for (blockid, quad) in greedy_determine_quads(&backward_slice, &registry, selector) {
                positions.extend([
                    [quad[0] as f32, quad[3] as f32, z as f32 + 1.0],
                    [quad[0] as f32, quad[1] as f32, z as f32 + 1.0],
                    [quad[2] as f32, quad[1] as f32, z as f32 + 1.0],
                    [quad[2] as f32, quad[3] as f32, z as f32 + 1.0],
                    [quad[0] as f32, quad[3] as f32, z as f32 + 1.0],
                    [quad[2] as f32, quad[1] as f32, z as f32 + 1.0],
                ]);
                normals.extend([[0.0, 0.0, -1.0]; 6]);
                uvs.extend([
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [1.0, 1.0],
                ]);
                color_extend(colors, blockid, &registry);
                repeat.extend([[
                    ((quad[2]-quad[0])) as u32,
                    ((quad[3]-quad[1])) as u32,
                    ]; 6]);
            }
        }
    }
}

pub(crate) fn color_extend(
    colors: &mut Vec<[f32; 4]>,
    blockid: BlockId,
    registry: &BlockRegistryInternal,
) {
    const EMPTY_COLOR: [[f32; 4]; 6] = [[1.0, 1.0, 1.0, 1.0]; 6];
    colors.extend(match registry.get_by_numerical_id(blockid) {
        Some(blockdata) => match blockdata.get_attribute(BlockData::ATTRIBUTE_BASE_COLOR) {
            Some(value) => {
                let value: Color = value.clone().try_into().unwrap();
                [value.as_rgba_f32(); 6]
            }
            None => EMPTY_COLOR,
        },
        None => EMPTY_COLOR,
    });
}

pub(crate) fn get_visibility(
    block: BlockId,
    registry: &BlockRegistryInternal,
) -> MeshingVisibility {
    match registry.get_by_numerical_id(block) {
        Some(entry) => entry.block_visibility,
        None => MeshingVisibility::Invisible,
    }
}
