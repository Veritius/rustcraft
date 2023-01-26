use ndarray::{Array3, Axis};
use crate::world::{block::{BlockId, registry::BLOCK_REGISTRY, data::BlockData}, chunk::{CHUNK_SIZE, meshing::solid::color_extend}};
use super::{MeshingPass, greedy::greedy_determine_quads, MeshingPassIdentifier};

pub const LIQUID_MESHER_PASS: MeshingPassIdentifier = MeshingPassIdentifier::new("engine_liquid", 1);

/// Simplistic liquid mesh implementation.
pub struct LiquidMesher;
impl MeshingPass for LiquidMesher {
    fn do_pass(
        &self,
        positions: &mut Vec<[f32;3]>,
        normals: &mut Vec<[f32;3]>,
        uvs: &mut Vec<[f32;2]>,
        colors: &mut Vec<[f32;4]>,
        data: &Array3<BlockId>
    ) {
        todo!("");
        let registry = BLOCK_REGISTRY.read().unwrap();
        
        for y in 1..CHUNK_SIZE+1 {
            let mut layer = [[BlockId::EMPTY; CHUNK_SIZE]; CHUNK_SIZE];
            for x in 1..CHUNK_SIZE+1 {
            for z in 1..CHUNK_SIZE+1 {
                let above = data[[x, y+1, z]];
                let this = data[[x, y, z]];
                if registry.get_by_numerical_id(this).unwrap().get_attribute(BlockData::ATTRIBUTE_USE_LIQUID_MESHER).is_some() && above != this {
                    layer[x-1][z-1] = this;
                }
            }}

            let y = y as f32 + 3.0; //- 0.85;

            const UVS: [[f32; 2]; 6] = [
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [1.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
            ];

            for (block, quad) in greedy_determine_quads(&layer, &registry) {
                positions.extend([
                    [quad[0] as f32, y, quad[3] as f32],
                    [quad[0] as f32, y, quad[1] as f32],
                    [quad[2] as f32, y, quad[1] as f32],
                    [quad[2] as f32, y, quad[3] as f32],
                    [quad[0] as f32, y, quad[3] as f32],
                    [quad[2] as f32, y, quad[1] as f32],
                ]);
                normals.extend([[0.0, 1.0, 0.0]; 6]);
                uvs.extend(UVS);
                color_extend(colors, block, &registry);
            }
        }
    }
}