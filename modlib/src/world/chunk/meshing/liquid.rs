use ndarray::{Array3, Axis};
use crate::world::{block::{BlockId, registry::{BLOCK_REGISTRY, BlockRegistryInternal}, data::BlockData}, chunk::{CHUNK_SIZE, meshing::solid::color_extend}};
use super::{MeshingPass, greedy::greedy_determine_quads, MeshingPassIdentifier};

pub const LIQUID_MESHER_PASS: MeshingPassIdentifier = MeshingPassIdentifier::new("engine_liquid", 1);

/// Simplistic liquid mesh implementation.
pub struct LiquidMesher;
impl MeshingPass for LiquidMesher {
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
            registry.get_by_numerical_id(*block).unwrap().get_attribute(BlockData::ATTRIBUTE_USE_LIQUID_MESHER).is_some()
        }
        
        for y in 1..CHUNK_SIZE+1 {
            let array_subview = array.index_axis(Axis(1), y);
            let mut layer = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for x in 1..CHUNK_SIZE+1 {
                for z in 1..CHUNK_SIZE+1 {
                    let this_block = array_subview[[x, z]];
                    if selector(&this_block, &registry) && this_block != array[[x, y+1, z]] {
                        layer[x-1][z-1] = this_block;
                    }
                }
            }

            let y = y as f32 - 0.15;

            const UVS: [[f32; 2]; 6] = [
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [1.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
            ];

            for (block, quad) in greedy_determine_quads(&layer, &registry, selector) {
                positions.extend([
                    [quad[0] as f32, y, quad[1] as f32],
                    [quad[0] as f32, y, quad[3] as f32],
                    [quad[2] as f32, y, quad[1] as f32],
                    [quad[0] as f32, y, quad[3] as f32],
                    [quad[2] as f32, y, quad[3] as f32],
                    [quad[2] as f32, y, quad[1] as f32],
                ]);
                normals.extend([[0.0, -1.0, 0.0]; 6]);
                uvs.extend(UVS);
                color_extend(colors, block, &registry);
            }
        }
    }
} 