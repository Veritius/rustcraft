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

use super::{MeshingPass, MeshingVisibility, SHAPE_SIZE_USIZE, MeshVertexAttributeOrderable};

// pub(super) fn simple_mesh(
//     mesh: &mut Mesh,
//     array: &Array3<BlockId>,
//     registry: &BlockRegistry,
// ) {
//     let mut positions = vec![];
//     let mut
// }

pub struct SolidBlockMesher;
impl MeshingPass for SolidBlockMesher {
    fn do_pass(
        &self,
        attributes: &mut BTreeMap<MeshVertexAttributeOrderable, VertexAttributeValues>,
        array: &Array3<BlockId>,
    ) {
        let registry = BLOCK_REGISTRY.read().unwrap();

        let mut positions = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut colors: Vec<[f32; 4]> = vec![];

        const UVS: [[f32; 2]; 6] = [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
        ];

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

            for (blockid, quad) in greedy_determine_quads(&left_slice) {
                positions.extend([
                    [x as f32, quad[0] as f32, quad[1] as f32],
                    [x as f32, quad[0] as f32, quad[3] as f32],
                    [x as f32, quad[2] as f32, quad[1] as f32],
                    [x as f32, quad[0] as f32, quad[3] as f32],
                    [x as f32, quad[2] as f32, quad[3] as f32],
                    [x as f32, quad[2] as f32, quad[1] as f32],
                ]);
                normals.extend([[1.0, 0.0, 0.0]; 6]);
                uvs.extend(UVS);
                color_extend(&mut colors, blockid, &registry);
            }
            for (blockid, quad) in greedy_determine_quads(&right_slice) {
                positions.extend([
                    [x as f32 + 1.0, quad[0] as f32, quad[3] as f32],
                    [x as f32 + 1.0, quad[0] as f32, quad[1] as f32],
                    [x as f32 + 1.0, quad[2] as f32, quad[1] as f32],
                    [x as f32 + 1.0, quad[2] as f32, quad[3] as f32],
                    [x as f32 + 1.0, quad[0] as f32, quad[3] as f32],
                    [x as f32 + 1.0, quad[2] as f32, quad[1] as f32],
                ]);
                normals.extend([[-1.0, 0.0, 0.0]; 6]);
                uvs.extend(UVS);
                color_extend(&mut colors, blockid, &registry);
            }
        }

        // Up and down
        for y in 1..SHAPE_SIZE_USIZE - 1 {
            let array_subview = array.index_axis(Axis(1), y);
            let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for x in 1..SHAPE_SIZE_USIZE - 1 {
                for z in 1..SHAPE_SIZE_USIZE - 1 {
                    let this_block = array_subview[[x, z]];
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y - 1, z]], &registry))
                    {
                        left_slice[x - 1][z - 1] = this_block;
                    }
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y + 1, z]], &registry))
                    {
                        right_slice[x - 1][z - 1] = this_block;
                    }
                }
            }

            let y = y - 1;

            for (blockid, quad) in greedy_determine_quads(&left_slice) {
                positions.extend([
                    [quad[0] as f32, y as f32, quad[3] as f32],
                    [quad[0] as f32, y as f32, quad[1] as f32],
                    [quad[2] as f32, y as f32, quad[1] as f32],
                    [quad[2] as f32, y as f32, quad[3] as f32],
                    [quad[0] as f32, y as f32, quad[3] as f32],
                    [quad[2] as f32, y as f32, quad[1] as f32],
                ]);
                normals.extend([[0.0, 1.0, 0.0]; 6]);
                uvs.extend(UVS);
                color_extend(&mut colors, blockid, &registry);
            }
            for (blockid, quad) in greedy_determine_quads(&right_slice) {
                positions.extend([
                    [quad[0] as f32, y as f32 + 1.0, quad[1] as f32],
                    [quad[0] as f32, y as f32 + 1.0, quad[3] as f32],
                    [quad[2] as f32, y as f32 + 1.0, quad[1] as f32],
                    [quad[0] as f32, y as f32 + 1.0, quad[3] as f32],
                    [quad[2] as f32, y as f32 + 1.0, quad[3] as f32],
                    [quad[2] as f32, y as f32 + 1.0, quad[1] as f32],
                ]);
                normals.extend([[0.0, -1.0, 0.0]; 6]);
                uvs.extend(UVS);
                color_extend(&mut colors, blockid, &registry);
            }
        }

        // Forward and backward
        for z in 1..SHAPE_SIZE_USIZE - 1 {
            let array_subview = array.index_axis(Axis(2), z);
            let mut left_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            let mut right_slice = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for x in 1..SHAPE_SIZE_USIZE - 1 {
                for y in 1..SHAPE_SIZE_USIZE - 1 {
                    let this_block = array_subview[[x, y]];
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y, z - 1]], &registry))
                    {
                        left_slice[x - 1][y - 1] = this_block;
                    }
                    if get_visibility(this_block, &registry)
                        .is_visible_against(&get_visibility(array[[x, y, z + 1]], &registry))
                    {
                        right_slice[x - 1][y - 1] = this_block;
                    }
                }
            }

            let z = z - 1;

            for (blockid, quad) in greedy_determine_quads(&left_slice) {
                positions.extend([
                    [quad[0] as f32, quad[1] as f32, z as f32],
                    [quad[0] as f32, quad[3] as f32, z as f32],
                    [quad[2] as f32, quad[1] as f32, z as f32],
                    [quad[0] as f32, quad[3] as f32, z as f32],
                    [quad[2] as f32, quad[3] as f32, z as f32],
                    [quad[2] as f32, quad[1] as f32, z as f32],
                ]);
                normals.extend([[0.0, 0.0, 1.0]; 6]);
                uvs.extend(UVS);
                color_extend(&mut colors, blockid, &registry);
            }
            for (blockid, quad) in greedy_determine_quads(&right_slice) {
                positions.extend([
                    [quad[0] as f32, quad[3] as f32, z as f32 + 1.0],
                    [quad[0] as f32, quad[1] as f32, z as f32 + 1.0],
                    [quad[2] as f32, quad[1] as f32, z as f32 + 1.0],
                    [quad[2] as f32, quad[3] as f32, z as f32 + 1.0],
                    [quad[0] as f32, quad[3] as f32, z as f32 + 1.0],
                    [quad[2] as f32, quad[1] as f32, z as f32 + 1.0],
                ]);
                normals.extend([[0.0, 0.0, -1.0]; 6]);
                uvs.extend(UVS);
                color_extend(&mut colors, blockid, &registry);
            }
        }

        // Positions
        if attributes.contains_key(&Mesh::ATTRIBUTE_POSITION.into()) {
            let m = attributes.get_mut(&Mesh::ATTRIBUTE_POSITION.into()).unwrap();
            match m {
                VertexAttributeValues::Float32x3(vec) => {
                    vec.append(&mut positions);
                },
                _ => panic!(),
            }
        }

        // Normals
        if attributes.contains_key(&Mesh::ATTRIBUTE_NORMAL.into()) {
            let m = attributes.get_mut(&Mesh::ATTRIBUTE_NORMAL.into()).unwrap();
            match m {
                VertexAttributeValues::Float32x3(vec) => {
                    vec.append(&mut normals);
                },
                _ => panic!(),
            }
        }

        // Uvs
        if attributes.contains_key(&Mesh::ATTRIBUTE_UV_0.into()) {
            let m = attributes.get_mut(&Mesh::ATTRIBUTE_UV_0.into()).unwrap();
            match m {
                VertexAttributeValues::Float32x2(vec) => {
                    vec.append(&mut uvs);
                },
                _ => panic!(),
            }
        }

        // Uvs
        if attributes.contains_key(&Mesh::ATTRIBUTE_COLOR.into()) {
            let m = attributes.get_mut(&Mesh::ATTRIBUTE_COLOR.into()).unwrap();
            match m {
                VertexAttributeValues::Float32x4(vec) => {
                    vec.append(&mut colors);
                },
                _ => panic!(),
            }
        }
    }
}

fn color_extend(
    colors: &mut Vec<[f32; 4]>,
    blockid: BlockId,
    registry: &RwLockReadGuard<BlockRegistryInternal>,
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

fn get_visibility(
    block: BlockId,
    registry: &RwLockReadGuard<BlockRegistryInternal>,
) -> MeshingVisibility {
    match registry.get_by_numerical_id(block) {
        Some(entry) => entry.block_visibility,
        None => MeshingVisibility::Invisible,
    }
}
