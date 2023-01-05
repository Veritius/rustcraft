use bevy::prelude::*;
use ndarray::Axis;
use crate::block::{registry::BlockRegistry, Block, entity::BlockEntity};
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
    blocks: Query<(Entity, &BlockEntity)>,
    chunks: Query<(Entity, &Chunk, &Handle<Mesh>, Option<&RemeshChunkMarker>)>,
) {
    for (chunk_entityid, chunk_data, mesh_handle, chunk_remesh_marker) in chunks.iter() {
        if let Some(_) = chunk_remesh_marker {
            let c_pos = chunk_data.get_position();

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let block = chunk_data.get_block(x, y, z);
                        let visibility = match block {
                            crate::block::Block::Empty => MeshingVisibility::Invisible,
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
                    }
                }
            }
            //
            //let mut vertices: = vec![];
        }

        commands.entity(chunk_entityid).remove::<RemeshChunkMarker>();
    }
}