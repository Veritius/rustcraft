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
            let positions = vec![];
            // let normals = vec![];
            // let uvs = vec![];
            // let colors = vec![];

            for x in 0..CHUNK_SIZE as i32 {
                for y in 0..CHUNK_SIZE as i32 {
                    for z in 0..CHUNK_SIZE as i32 {
                            const POS_IDX_A: [f32; 3] = [-0.5, -0.5, -0.5]; // Bottom left near
                            const POS_IDX_B: [f32; 3] = [0.5, -0.5, -0.5]; // Bottom right near
                            const POS_IDX_C: [f32; 3] = [0.5, -0.5, 0.5]; // Bottom right far
                            const POS_IDX_D: [f32; 3] = [-0.5, -0.5, 0.5]; // Bottom left far
                            const POS_IDX_E: [f32; 3] = [-0.5, 0.5, -0.5]; // Top left near
                            const POS_IDX_F: [f32; 3] = [0.5, 0.5, -0.5]; // Top right near
                            const POS_IDX_G: [f32; 3] = [0.5, 0.5, 0.5]; // Bottom right far
                            const POS_IDX_H: [f32; 3] = [-0.5, 0.5, 0.5]; // Bottom left far
                        }
                    }
                }
            }

            let mesh = Mesh::new(PrimitiveTopology::TriangleList);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_0);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            // Update mesh and remove marker component
            commands.entity(chunk_entityid)
                // .insert(meshes.add(mesh))
                .remove::<RemeshChunkMarker>();
                
    }
}