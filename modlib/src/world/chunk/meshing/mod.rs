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
            
            let chunk_package = (
                this_chunk,
                quick_get_chunk(&chunk_registry, &chunks, this_chunk_position, (0, 1, 0)),
                quick_get_chunk(&chunk_registry, &chunks, this_chunk_position, (0, -1, 0)),
                quick_get_chunk(&chunk_registry, &chunks, this_chunk_position, (1, 0, 0)),
                quick_get_chunk(&chunk_registry, &chunks, this_chunk_position, (-1, 0, 0)),
                quick_get_chunk(&chunk_registry, &chunks, this_chunk_position, (0, 0, -1)),
                quick_get_chunk(&chunk_registry, &chunks, this_chunk_position, (0, 0, 1)),
            );

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            let mut positions = vec![];
            let mut normals = vec![];
            let mut uv_0 = vec![];
            let mut colors = vec![];

            for x in 0..CHUNK_SIZE as i32 {
                for y in 0..CHUNK_SIZE as i32 {
                    for z in 0..CHUNK_SIZE as i32 {
                        // Temporary inefficient implementation just to see if everything works.

                        let offset_x: i32 = x as i32 - 8;
                        let offset_y: i32 = y as i32 - 8;
                        let offset_z: i32 = z as i32 - 8;

                        fn get_visibility(block: &Block, blocks: &Query<(Entity, &BlockEntity)>, block_registry: &Res<BlockRegistry>) -> MeshingVisibility {
                            match block {
                                Block::Empty => MeshingVisibility::Invisible,
                                Block::Entity(entityid) => {
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
                                Block::Generic(blockid) => {
                                    block_registry.get_by_id(*blockid).expect(&format!("Block ID {:?} didn't have an entry in the registry!", blockid)).visibility()
                                },
                            }
                        }
                        
                        // unnecessary usize to i32 to usize conversion lol
                        let this_block = this_chunk.get_block(x as usize, y as usize, z as usize);
                        let this_visibility = get_visibility(this_block, &blocks, &block_registry);

                        for idx in 0..6u8 {
                            let other_block = match idx {
                                // above
                                0 => efficient_get_block(IVec3 { x, y: y + 1, z }, chunk_package),
                                // below
                                1 => efficient_get_block(IVec3 { x, y: y - 1, z }, chunk_package),
                                // left
                                2 => efficient_get_block(IVec3 { x: x + 1, y, z }, chunk_package),
                                // right
                                3 => efficient_get_block(IVec3 { x: x - 1, y, z }, chunk_package),
                                // forward
                                4 => efficient_get_block(IVec3 { x, y, z: z - 1 }, chunk_package),
                                // backward
                                5 => efficient_get_block(IVec3 { x, y, z: z + 1 }, chunk_package),
                                // explode
                                _ => panic!("Cosmic ray detected!"),
                            };
                            let other_visibility = get_visibility(&other_block, &blocks, &block_registry);

                            // This section decides whether a face should be generated or not
                            if match (this_visibility, other_visibility) {
                                (MeshingVisibility::Opaque, MeshingVisibility::Opaque) => false,
                                (MeshingVisibility::Opaque, MeshingVisibility::Translucent) => true,
                                (MeshingVisibility::Opaque, MeshingVisibility::Invisible) => true,
                                (MeshingVisibility::Translucent, MeshingVisibility::Opaque) => false,
                                (MeshingVisibility::Translucent, MeshingVisibility::Translucent) => false,
                                (MeshingVisibility::Translucent, MeshingVisibility::Invisible) => true,
                                // All other options start with us being invisible, so we don't bother.
                                _ => false,
                            } == false { continue }

                            // Consts for each vertex on a cube
                            const IDX_A: [f32; 3] = [0.0, 0.0, 1.0];
                            const IDX_B: [f32; 3] = [1.0, 0.0, 1.0];
                            const IDX_C: [f32; 3] = [1.0, 0.0, 0.0];
                            const IDX_D: [f32; 3] = [0.0, 0.0, 0.0];
                            const IDX_E: [f32; 3] = [0.0, 1.0, 1.0];
                            const IDX_F: [f32; 3] = [1.0, 1.0, 1.0];
                            const IDX_G: [f32; 3] = [1.0, 1.0, 0.0];
                            const IDX_H: [f32; 3] = [0.0, 1.0, 0.0];
                            
                            match idx { 
                                0 => {
                                    // top side (red)
                                    positions.append(&mut offset_verts(vec![
                                        IDX_E, IDX_F, IDX_G,
                                        IDX_E, IDX_G, IDX_H],
                                        offset_x, offset_y, offset_z));
                                    normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                                    uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                                    colors.append(&mut vec![[1.0, 0.0, 0.0, 1.0]; 6]);
                                },
                                1 => {
                                    // bottom side (blue)
                                    positions.append(&mut offset_verts(vec![
                                        IDX_A, IDX_D, IDX_C,
                                        IDX_A, IDX_C, IDX_B],
                                        offset_x, offset_y, offset_z));
                                    normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                                    uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                                    colors.append(&mut vec![[0.0, 0.0, 1.0, 1.0]; 6]);
                                },
                                2 => {
                                    // left side (green)
                                    positions.append(&mut offset_verts(vec![
                                        IDX_C, IDX_D, IDX_H,
                                        IDX_C, IDX_H, IDX_G],
                                        offset_x, offset_y, offset_z));
                                    normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                                    uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                                    colors.append(&mut vec![[0.0, 1.0, 0.0, 1.0]; 6]);
                                },
                                3 => {
                                    // right side (yellow)
                                    positions.append(&mut offset_verts(vec![
                                        IDX_E, IDX_A, IDX_B,
                                        IDX_E, IDX_B, IDX_F],
                                        offset_x, offset_y, offset_z));
                                    normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                                    uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                                    colors.append(&mut vec![[1.0, 1.0, 1.0, 1.0]; 6]);
                                },
                                4 => {
                                    // front side (purple)
                                    positions.append(&mut offset_verts(vec![
                                        IDX_G, IDX_F, IDX_B,
                                        IDX_G, IDX_B, IDX_C],
                                        offset_x, offset_y, offset_z));
                                    normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                                    uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                                    colors.append(&mut vec![[1.0, 0.0, 1.0, 1.0]; 6]);
                                }
                                5 => {
                                    // back side (teal)
                                    positions.append(&mut offset_verts(vec![
                                        IDX_D, IDX_A, IDX_E,
                                        IDX_D, IDX_E, IDX_H],
                                        offset_x, offset_y, offset_z));
                                    normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                                    uv_0.append(&mut vec![[0.0, 0.0]; 6]);
                                    colors.append(&mut vec![[0.0, 1.0, 1.0, 1.0]; 6]);
                                },
                                _ => panic!("Cosmic ray detected")
                            };
                        }
                    }
                }
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_0);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            // Update mesh and remove marker component
            commands.entity(chunk_entityid)
                .insert(meshes.add(mesh))
                .remove::<RemeshChunkMarker>();
        }
    }
}

fn offset_verts(positions: Vec<[f32; 3]>, offset_x: i32, offset_y: i32, offset_z: i32) -> Vec<[f32; 3]> {
    let mut new_positions = vec![];
    for position in positions {
        let mut position = position;
        position[0] += offset_x as f32;
        position[1] += offset_y as f32;
        position[2] += offset_z as f32;
        new_positions.push(position);
    }
    new_positions
}

fn quick_get_chunk<'q>(registry: &ChunkRegistry, chunk_query: &'q Query<(Entity, &Chunk, Option<&RemeshChunkMarker>)>, chunk_pos: (i32, i32, i32), offset: (i32, i32, i32)) -> Option<&'q Chunk> {
    match registry.get((chunk_pos.0 + offset.0, chunk_pos.1 + offset.1, chunk_pos.2 + offset.2)) {
        Ok(value) => {
            match value {
                Some(value) => {
                    match chunk_query.get(*value) {
                        Ok(value) => Some(value.1),
                        Err(_) => None,
                    }
                },
                None => None,
            }
        },
        Err(_) => None,
    }
}

fn efficient_get_block(relative_coords: IVec3, chunks: (&Chunk, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>)) -> Block {
    let chunk_size = CHUNK_SIZE as i32 - 1;
    // top
    if relative_coords.y > chunk_size {
        let chunk = chunks.1;
        let coords = IVec3 {
            x: relative_coords.x.clamp(0, chunk_size),
            y: relative_coords.y - chunk_size,
            z: relative_coords.z.clamp(0, chunk_size),
        };
        match chunk {
            Some(chunk) => {
                return chunk.get_block(coords.x as usize, coords.y as usize, coords.z as usize).clone()
            },
            None => { return Block::Empty },
        }
    }

    // bottom
    if relative_coords.y < 0 {
        let chunk = chunks.2;
        let coords = IVec3 {
            x: relative_coords.x.clamp(0, chunk_size),
            y: relative_coords.y + chunk_size,
            z: relative_coords.z.clamp(0, chunk_size),
        };
        match chunk {
            Some(chunk) => {
                return chunk.get_block(coords.x as usize, coords.y as usize, coords.z as usize).clone()
            },
            None => { return Block::Empty },
        }
    }

    // left
    if relative_coords.x < 0 {
        let chunk = chunks.3;
        let coords = IVec3 {
            x: relative_coords.x + chunk_size,
            y: relative_coords.y.clamp(0, chunk_size),
            z: relative_coords.z.clamp(0, chunk_size),
        };
        match chunk {
            Some(chunk) => {
                return chunk.get_block(coords.x as usize, coords.y as usize, coords.z as usize).clone()
            },
            None => { return Block::Empty },
        }
    }

    // right
    if relative_coords.x > chunk_size {
        let chunk = chunks.4;
        let coords = IVec3 {
            x: relative_coords.x - chunk_size,
            y: relative_coords.y.clamp(0, chunk_size),
            z: relative_coords.z.clamp(0, chunk_size),
        };
        match chunk {
            Some(chunk) => {
                return chunk.get_block(coords.x as usize, coords.y as usize, coords.z as usize).clone()
            },
            None => { return Block::Empty },
        }
    }

    // front
    if relative_coords.z > chunk_size {
        let chunk = chunks.5;
        let coords = IVec3 {
            x: relative_coords.x.clamp(0, 16),
            y: relative_coords.y.clamp(0, chunk_size),
            z: relative_coords.z - chunk_size,
        };
        match chunk {
            Some(chunk) => {
                return chunk.get_block(coords.x as usize, coords.y as usize, coords.z as usize).clone()
            },
            None => { return Block::Empty },
        }
    }

    // back
    if relative_coords.z < 0 {
        let chunk = chunks.6;
        let coords = IVec3 {
            x: relative_coords.x.clamp(0, 16),
            y: relative_coords.y.clamp(0, 16),
            z: relative_coords.z.clamp(0, 16),
        };
        match chunk {
            Some(chunk) => {
                return chunk.get_block(coords.x as usize, coords.y as usize, coords.z as usize).clone()
            },
            None => { return Block::Empty },
        }
    }

    chunks.0.get_block(relative_coords.x as usize, relative_coords.y as usize, relative_coords.z as usize).clone()
}