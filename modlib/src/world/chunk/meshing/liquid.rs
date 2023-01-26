use ndarray::Array3;
use crate::world::block::BlockId;
use super::MeshingPass;

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
        todo!()
    }
}