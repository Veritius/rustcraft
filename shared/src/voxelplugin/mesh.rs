use bevy::{prelude::Mesh, render::mesh::{PrimitiveTopology, Indices}};
use super::{chunk::Chunk, voxel::VoxelDataTable};

pub(crate) fn generate_visual_mesh(table: &VoxelDataTable, chunk: &Chunk) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut vertices: Vec::<[f32; 3]> = Vec::new();
    let mut normals: Vec::<[f32; 3]> = Vec::new();
    let mut tangents: Vec::<[f32; 3]> = Vec::new();
    let mut uvs: Vec::<[f32; 2]> = Vec::new();
    let mut indices: Vec::<u32> = Vec::new();

    // TODO: This is a shitty implementation, and the objectively worst performing one
    for (x, aa) in chunk.voxels.into_iter().enumerate() {
        for (y, ba) in aa.into_iter().enumerate() {
            for (z, block) in ba.into_iter().enumerate() {
                match block {
                    Some(blockdata) => {
                        // Upper face
                        vertices.push([0.0, 0.0, 1.0]);
                        vertices.push([1.0, 0.0, 1.0]);
                        vertices.push([0.0, 1.0, 1.0]);
                        vertices.push([1.0, 1.0, 1.0]);
                        vertices.push([0.0, 1.0, 1.0]);
                        vertices.push([1.0, 0.0, 0.0]);
                        // Lower face
                        vertices.push([1.0, 0.0, 0.0]);
                        vertices.push([1.0, 1.0, 0.0]);
                        vertices.push([0.0, 1.0, 0.0]);
                        vertices.push([1.0, 1.0, 1.0]);
                        vertices.push([0.0, 1.0, 1.0]);
                        vertices.push([0.0, 0.0, 0.0]);
                        // North face
                        vertices.push([0.0, 0.0, 1.0]);
                        vertices.push([0.0, 0.0, 0.0]);
                        vertices.push([0.0, 1.0, 0.0]);
                        vertices.push([0.0, 0.0, 1.0]);
                        vertices.push([0.0, 1.0, 1.0]);
                        vertices.push([1.0, 0.0, 0.0]);
                        // South face
                        vertices.push([1.0, 0.0, 1.0]);
                        vertices.push([1.0, 0.0, 0.0]);
                        vertices.push([1.0, 1.0, 0.0]);
                        vertices.push([1.0, 0.0, 1.0]);
                        vertices.push([1.0, 0.0, 1.0]);
                        vertices.push([1.0, 1.0, 1.0]);
                        // West face
                        vertices.push([0.0, 0.0, 1.0]);
                        vertices.push([0.0, 0.0, 0.0]);
                        vertices.push([1.0, 0.0, 0.0]);
                        vertices.push([0.0, 0.0, 1.0]);
                        vertices.push([1.0, 0.0, 1.0]);
                        vertices.push([1.0, 0.0, 0.0]);
                        // East face
                        vertices.push([1.0, 1.0, 1.0]);
                        vertices.push([1.0, 1.0, 0.0]);
                        vertices.push([0.0, 1.0, 0.0]);
                        vertices.push([1.0, 1.0, 1.0]);
                        vertices.push([0.0, 1.0, 1.0]);
                        vertices.push([0.0, 1.0, 0.0]);
                    }
                    None => {}
                }
            }
        }
    }

    for i in 0..vertices.len() {
        normals.push([0.0, 0.0, 0.0]);
        tangents.push([0.0, 0.0, 0.0]);
        uvs.push([0.0, 1.0]);
        indices.push(i.try_into().unwrap());
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    return mesh;
}