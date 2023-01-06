use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh}};
use ndarray::Axis;
use crate::block::{registry::BlockRegistry, Block, entity::BlockEntity};
use super::{registry::{ChunkRegistry, ChunkCoordinate}, Chunk, CHUNK_SIZE};

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
    blocks: Query<(Entity, &BlockEntity)>,
    chunks: Query<(Entity, &Chunk, Option<&RemeshChunkMarker>)>,
) {
    for (chunk_entityid, chunk_data, chunk_remesh_marker) in chunks.iter() {
        if let Some(_) = chunk_remesh_marker {
            let c_pos = chunk_data.get_position();
            
            // TODO: These numbers are almost definitely wrong, check them later.
            // let chunk_up = get_from_chunk_registry(&chunk_registry, &chunks, (c_pos.0, c_pos.1, c_pos.2 + 1));
            // let chunk_down = get_from_chunk_registry(&chunk_registry, &chunks, (c_pos.0, c_pos.1, c_pos.2 - 1));
            // let chunk_left = get_from_chunk_registry(&chunk_registry, &chunks, (c_pos.0, c_pos.1 + 1, c_pos.2));
            // let chunk_right = get_from_chunk_registry(&chunk_registry, &chunks, (c_pos.0, c_pos.1 - 1, c_pos.2));
            // let chunk_fwd = get_from_chunk_registry(&chunk_registry, &chunks, (c_pos.0 + 1, c_pos.1, c_pos.2));
            // let chunk_back = get_from_chunk_registry(&chunk_registry, &chunks, (c_pos.0 - 1, c_pos.1, c_pos.2));

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            let mut positions = vec![];
            let mut normals = vec![];

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let block = chunk_data.get_block(x, y, z);
                        let visibility = match block {
                            crate::block::Block::Empty => MeshingVisibility::Opaque,
                            crate::block::Block::Entity(entityid) => {
                                match blocks.get(*entityid) {
                                    Ok((_entity, block)) => {
                                        block.visibility
                                    },
                                    Err(_) => {
                                        warn!("Entity id {:?} was stored in a chunk but wasn't available in a query", entityid);
                                        MeshingVisibility::Invisible
                                    },
                                }
                            },
                            crate::block::Block::Generic(blockid) => {
                                block_registry.get_by_id(*blockid).expect(&format!("Block ID {:?} didn't have an entry in the registry!", blockid)).visibility()
                            },
                        };


                        // Temporary inefficient implementation just to see if everything works.
                        match visibility {
                            MeshingVisibility::Opaque | MeshingVisibility::Translucent => {

                                let mut vertices = vec![];
                                
                                // front side
                                vertices.append(&mut vec![[1.0, 1.0, -1.0], [1.0, 1.0, 1.0], [1.0, -1.0, 1.0]]);
                                vertices.append(&mut vec![[1.0, 1.0, -1.0], [1.0, -1.0, -1.0], [1.0, -1.0, 1.0]]);
                                // back side
                                vertices.append(&mut vec![[-1.0, -1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, 1.0, 1.0]]);
                                vertices.append(&mut vec![[-1.0, -1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]]);
                                // left side
                                vertices.append(&mut vec![[1.0, -1.0, -1.0], [-1.0, -1.0, -1.0], [-1.0, 1.0, -1.0]]);
                                vertices.append(&mut vec![[1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, -1.0]]);
                                // right side
                                vertices.append(&mut vec![[1.0, -1.0, 1.0], [-1.0, -1.0, 1.0], [-1.0, 1.0, 1.0]]);
                                vertices.append(&mut vec![[1.0, -1.0, 1.0], [1.0, 1.0, 1.0], [-1.0, 1.0, 1.0]]);
                                // top side
                                vertices.append(&mut vec![[-1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, -1.0]]);
                                vertices.append(&mut vec![[-1.0, 1.0, 1.0], [-1.0, 1.0, -1.0], [1.0, 1.0, -1.0]]);
                                // bottom side
                                vertices.append(&mut vec![[-1.0, -1.0, 1.0], [-1.0, -1.0, -1.0], [1.0, -1.0, -1.0]]);
                                vertices.append(&mut vec![[-1.0, -1.0, 1.0], [1.0, -1.0, 1.0], [1.0, -1.0, -1.0]]);

                                for v in vertices.iter() {
                                    positions.push(*v);
                                    normals.push(*v);
                                }
                            },
                            MeshingVisibility::Invisible => { continue },
                        }
                    }
                }
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

            // Update mesh and remove marker component
            commands.entity(chunk_entityid)
                .insert(meshes.add(mesh))
                .remove::<RemeshChunkMarker>();
        }
    }
}

// fn get_from_chunk_registry<'a>(registry: &Res<ChunkRegistry>, query: &'a Query<(Entity, &Chunk, &Handle<Mesh>, Option<&RemeshChunkMarker>)>, coord: ChunkCoordinate) -> Option<&'a Chunk> {
//     match registry.get(coord) {
//         Ok(result) => {
//             match result {
//                 Some(entity) => {
//                     match query.get(*entity) {
//                         Ok(success) => {
//                             Some(success.1)
//                         },
//                         Err(_) => None,
//                     }
//                 },
//                 None => None,
//             }
//         },
//         Err(_) => None,
//     }
// }