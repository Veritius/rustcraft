use bevy::{prelude::*, render::{render_resource::PrimitiveTopology}};
use crate::world::{block::{registry::BlockRegistry, Block, entity::BlockEntity}, WorldMapHelpers};
use super::{registry::ChunkRegistry, Chunk, CHUNK_SIZE};

/// Used for generating a mesh for a chunk.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MeshingVisibility {
    /// Produces faces on all sides, and prevents faces being produced for other blocks.
    Opaque,
    /// Produces faces on all sides, but does not prevent faces being produced for other blocks.
    Translucent,
    /// Does not produce faces at all, and allows faces to be produced for other blocks.
    /// This may also be used for blocks that have their own meshes and should not be included in the chunk mesh generation, i.e. entities.
    Invisible,
}

/// Added to chunks to indicate the need to regenerate their mesh.
#[derive(Component)]
pub struct RemeshChunkMarker;

pub fn remesh_chunk_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    block_registry: Res<BlockRegistry>,
    chunk_registry: Res<ChunkRegistry>,
    world_map: WorldMapHelpers,
    blocks: Query<(Entity, &BlockEntity)>,
    chunks: Query<(Entity, &Chunk, Option<&RemeshChunkMarker>)>,
) {
    for (chunk_entityid, this_chunk, chunk_remesh_marker) in chunks.iter() {
        if let Some(_) = chunk_remesh_marker {
            let this_chunk_position = this_chunk.get_position();

            for x in 0..CHUNK_SIZE as i32 {
                for y in 0..CHUNK_SIZE as i32 {
                    for z in 0..CHUNK_SIZE as i32 {
                            // // This section decides whether a face should be generated or not
                            // if match (this_visibility, other_visibility) {
                            //     (MeshingVisibility::Opaque, MeshingVisibility::Opaque) => false,
                            //     (MeshingVisibility::Opaque, MeshingVisibility::Translucent) => true,
                            //     (MeshingVisibility::Opaque, MeshingVisibility::Invisible) => true,
                            //     (MeshingVisibility::Translucent, MeshingVisibility::Opaque) => false,
                            //     (MeshingVisibility::Translucent, MeshingVisibility::Translucent) => false,
                            //     (MeshingVisibility::Translucent, MeshingVisibility::Invisible) => true,
                            //     // All other options start with us being invisible, so we don't bother.
                            //     _ => false,
                            // } == false { continue }

                            // Consts for each vertex on a cube
                            // const IDX_A: [f32; 3] = [0.0, 0.0, 1.0];
                            // const IDX_B: [f32; 3] = [1.0, 0.0, 1.0];
                            // const IDX_C: [f32; 3] = [1.0, 0.0, 0.0];
                            // const IDX_D: [f32; 3] = [0.0, 0.0, 0.0];
                            // const IDX_E: [f32; 3] = [0.0, 1.0, 1.0];
                            // const IDX_F: [f32; 3] = [1.0, 1.0, 1.0];
                            // const IDX_G: [f32; 3] = [1.0, 1.0, 0.0];
                            // const IDX_H: [f32; 3] = [0.0, 1.0, 0.0];
                            
                            // match idx { 
                            //     0 => {
                            //         // top side (red)
                            //         positions.append(&mut offset_verts(vec![
                            //             IDX_E, IDX_F, IDX_G,
                            //             IDX_E, IDX_G, IDX_H],
                            //             offset_x, offset_y, offset_z));
                            //         normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            //         uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                            //         colors.append(&mut vec![[1.0, 0.0, 0.0, 1.0]; 6]);
                            //     },
                            //     1 => {
                            //         // bottom side (blue)
                            //         positions.append(&mut offset_verts(vec![
                            //             IDX_A, IDX_D, IDX_C,
                            //             IDX_A, IDX_C, IDX_B],
                            //             offset_x, offset_y, offset_z));
                            //         normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            //         uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                            //         colors.append(&mut vec![[0.0, 0.0, 1.0, 1.0]; 6]);
                            //     },
                            //     2 => {
                            //         // left side (green)
                            //         positions.append(&mut offset_verts(vec![
                            //             IDX_C, IDX_D, IDX_H,
                            //             IDX_C, IDX_H, IDX_G],
                            //             offset_x, offset_y, offset_z));
                            //         normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            //         uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                            //         colors.append(&mut vec![[0.0, 1.0, 0.0, 1.0]; 6]);
                            //     },
                            //     3 => {
                            //         // right side (yellow)
                            //         positions.append(&mut offset_verts(vec![
                            //             IDX_E, IDX_A, IDX_B,
                            //             IDX_E, IDX_B, IDX_F],
                            //             offset_x, offset_y, offset_z));
                            //         normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            //         uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                            //         colors.append(&mut vec![[1.0, 1.0, 1.0, 1.0]; 6]);
                            //     },
                            //     4 => {
                            //         // front side (purple)
                            //         positions.append(&mut offset_verts(vec![
                            //             IDX_G, IDX_F, IDX_B,
                            //             IDX_G, IDX_B, IDX_C],
                            //             offset_x, offset_y, offset_z));
                            //         normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            //         uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                            //         colors.append(&mut vec![[1.0, 0.0, 1.0, 1.0]; 6]);
                            //     }
                            //     5 => {
                            //         // back side (teal)
                            //         positions.append(&mut offset_verts(vec![
                            //             IDX_D, IDX_A, IDX_E,
                            //             IDX_D, IDX_E, IDX_H],
                            //             offset_x, offset_y, offset_z));
                            //         normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            //         uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                            //         colors.append(&mut vec![[0.0, 1.0, 1.0, 1.0]; 6]);
                            //     },
                            //     _ => panic!("Cosmic ray detected")
                            // };
                        }
                    }
                }
            }

            // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_0);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            // Update mesh and remove marker component
            commands.entity(chunk_entityid)
                // .insert(meshes.add(mesh))
                .remove::<RemeshChunkMarker>();
                
    }
}