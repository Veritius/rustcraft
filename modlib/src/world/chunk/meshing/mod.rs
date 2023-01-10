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

impl MeshingVisibility {
    pub fn is_visible_against(&self, other: &MeshingVisibility) -> bool {
        match (self, other) {
            (MeshingVisibility::Invisible, _) => false,
            (MeshingVisibility::Opaque, MeshingVisibility::Opaque) => false,
            (MeshingVisibility::Opaque, MeshingVisibility::Translucent) => true,
            (MeshingVisibility::Opaque, MeshingVisibility::Invisible) => true,
            (MeshingVisibility::Translucent, MeshingVisibility::Opaque) => false,
            (MeshingVisibility::Translucent, MeshingVisibility::Translucent) => false,
            (MeshingVisibility::Translucent, MeshingVisibility::Invisible) => false,
        }
    }
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
            let chunk_tuple = (
                this_chunk,
                world_map.get_chunk_or_none((this_chunk_position.0 + 1, this_chunk_position.1, this_chunk_position.2)), // left
                world_map.get_chunk_or_none((this_chunk_position.0 - 1, this_chunk_position.1, this_chunk_position.2)), // right
                world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1 + 1, this_chunk_position.2)), // up
                world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1 - 1, this_chunk_position.2)), // down
                world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1, this_chunk_position.2 + 1)), // forward
                world_map.get_chunk_or_none((this_chunk_position.0, this_chunk_position.1, this_chunk_position.2 - 1)), // back
            );

            let mut positions = vec![];
            let mut normals = vec![];
            let mut uvs = vec![];
            let mut colors = vec![];

            for x in 0..CHUNK_SIZE as i32 {
                for y in 0..CHUNK_SIZE as i32 {
                    for z in 0..CHUNK_SIZE as i32 {
                        let offset = (x as i32 - 8, y as i32 - 8, z as i32 - 8);

                        //    H-----------G    Each corner of the cube is a const with a letter from A to H.
                        //   /|          /|    This little ASCII diagram tells you which corner of the cube the const is.
                        //  / |         / |    Look at https://bevy-cheatbook.github.io/features/coords.html to see
                        // E-----------F  |    what coordinate system Bevy uses and which axis is what.
                        // |  |        |  |
                        // |  D--------|--C
                        // | /         | /
                        // |/          |/
                        // A-----------B

                        // FRONT FACE: [GCF,FCB] [0,0,1]
                        // BACK FACE: [HED,DEA] [0,0,-1]
                        // RIGHT FACE: [AEB,EFB] [1,0,0]
                        // LEFT FACE: [HDC,CGH] [-1,0,0]
                        // TOP FACE: [EHF,GFH] [0,1,0]
                        // BOTTOM FACE: [DAB,CDB] [0,-1,0]

                        const POS_IDX_A: [f32; 3] = [-0.5, -0.5, -0.5];
                        const POS_IDX_B: [f32; 3] = [0.5, -0.5, -0.5];
                        const POS_IDX_C: [f32; 3] = [0.5, -0.5, 0.5];
                        const POS_IDX_D: [f32; 3] = [-0.5, -0.5, 0.5];
                        const POS_IDX_E: [f32; 3] = [-0.5, 0.5, -0.5];
                        const POS_IDX_F: [f32; 3] = [0.5, 0.5, -0.5];
                        const POS_IDX_G: [f32; 3] = [0.5, 0.5, 0.5];
                        const POS_IDX_H: [f32; 3] = [-0.5, 0.5, 0.5];

                        let this_block_pos = IVec3 { x, y, z };
                        let this_block = *this_chunk.get_block(x as usize, y as usize, z as usize);

                        // Back face (yellow)
                        let other_visibility = get_block_visibility(&smart_get_block(this_block_pos + IVec3 { x: 0, y: 0, z: -1 }, chunk_tuple), &block_registry, &blocks);
                        if get_block_visibility(&this_block, &block_registry, &blocks).is_visible_against(&other_visibility) {
                            positions.append(&mut offset_verts(vec![
                                POS_IDX_H, POS_IDX_E, POS_IDX_D,
                                POS_IDX_D, POS_IDX_E, POS_IDX_A,
                            ], offset));
                            normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            uvs.append(&mut vec![[0.0, 0.0]; 6]);
                            colors.append(&mut vec![[1.0, 1.0, 0.0, 1.0]; 6]);
                        }

                        // Front face (green)
                        let other_visibility = get_block_visibility(&smart_get_block(this_block_pos + IVec3 { x: 0, y: 0, z: 1 }, chunk_tuple), &block_registry, &blocks);
                        if get_block_visibility(&this_block, &block_registry, &blocks).is_visible_against(&other_visibility) {
                            positions.append(&mut offset_verts(vec![
                                POS_IDX_G, POS_IDX_C, POS_IDX_F,
                                POS_IDX_F, POS_IDX_C, POS_IDX_B,
                            ], offset));
                            normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            uvs.append(&mut vec![[0.0, 0.0]; 6]);
                            colors.append(&mut vec![[0.0, 1.0, 0.0, 1.0]; 6]);
                        }

                        // Left face (cyan)
                        let other_visibility = get_block_visibility(&smart_get_block(this_block_pos + IVec3 { x: -1, y: 0, z: 0 }, chunk_tuple), &block_registry, &blocks);
                        if get_block_visibility(&this_block, &block_registry, &blocks).is_visible_against(&other_visibility) {
                            positions.append(&mut offset_verts(vec![
                                POS_IDX_H, POS_IDX_D, POS_IDX_C,
                                POS_IDX_C, POS_IDX_G, POS_IDX_H,
                            ], offset));
                            normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            uvs.append(&mut vec![[0.0, 0.0]; 6]);
                            colors.append(&mut vec![[0.0, 1.0, 1.0, 1.0]; 6]);
                        }

                        // Right face (blue)
                        let other_visibility = get_block_visibility(&smart_get_block(this_block_pos + IVec3 { x: 1, y: 0, z: 0 }, chunk_tuple), &block_registry, &blocks);
                        if get_block_visibility(&this_block, &block_registry, &blocks).is_visible_against(&other_visibility) {
                            positions.append(&mut offset_verts(vec![
                                POS_IDX_A, POS_IDX_E, POS_IDX_B,
                                POS_IDX_E, POS_IDX_F, POS_IDX_B,
                            ], offset));
                            normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            uvs.append(&mut vec![[0.0, 0.0]; 6]);
                            colors.append(&mut vec![[0.0, 0.0, 1.0, 1.0]; 6]);
                        }

                        // Top face (red)
                        let other_visibility = get_block_visibility(&smart_get_block(this_block_pos + IVec3 { x: 0, y: 1, z: 0 }, chunk_tuple), &block_registry, &blocks);
                        if get_block_visibility(&this_block, &block_registry, &blocks).is_visible_against(&other_visibility) {
                            positions.append(&mut offset_verts(vec![
                                POS_IDX_E, POS_IDX_H, POS_IDX_F,
                                POS_IDX_G, POS_IDX_F, POS_IDX_H,
                            ], offset));
                            normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            uvs.append(&mut vec![[0.0, 0.0]; 6]);
                            colors.append(&mut vec![[1.0, 0.0, 0.0, 1.0]; 6]);
                        }

                        // Bottom face (magenta)
                        let other_visibility = get_block_visibility(&smart_get_block(this_block_pos + IVec3 { x: 0, y: -1, z: 0 }, chunk_tuple), &block_registry, &blocks);
                        if get_block_visibility(&this_block, &block_registry, &blocks).is_visible_against(&other_visibility) {
                            positions.append(&mut offset_verts(vec![
                                POS_IDX_D, POS_IDX_A, POS_IDX_B,
                                POS_IDX_C, POS_IDX_D, POS_IDX_B,
                            ], offset));
                            normals.append(&mut vec![[0.0, 1.0, 0.0]; 6]);
                            uvs.append(&mut vec![[0.0, 0.0]; 6]);
                            colors.append(&mut vec![[1.0, 0.0, 1.0, 1.0]; 6]);
                        }
                    }
                }
            }

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            // Update mesh and remove marker component
            commands.entity(chunk_entityid)
                .insert(meshes.add(mesh))
                .remove::<RemeshChunkMarker>();
        }     
    }
}

fn offset_verts(positions: Vec<[f32; 3]>, offset: (i32, i32, i32)) -> Vec<[f32; 3]> {
    let mut new_positions = vec![];
    for position in positions {
        let mut position = position;
        position[0] += offset.0 as f32;
        position[1] += offset.1 as f32;
        position[2] += offset.2 as f32;
        new_positions.push(position);
    }
    new_positions
}

fn smart_get_block(relative_position: IVec3, chunk_tuple: (&Chunk, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>, Option<&Chunk>)) -> Block {
    const ADJUSTED_CHUNK_SIZE: i32 = CHUNK_SIZE as i32 - 1;

    // FRONT FACE: [GCF,FCB] [0,0,1]
    // BACK FACE: [HED,DEA] [0,0,-1]
    // RIGHT FACE: [AEB,EFB] [1,0,0]
    // LEFT FACE: [HDC,CGH] [-1,0,0]
    // TOP FACE: [EHF,GFH] [0,1,0]
    // BOTTOM FACE: [DAB,CDB] [0,-1,0]

    if relative_position.x > ADJUSTED_CHUNK_SIZE {
        match chunk_tuple.1 {
            Some(chunk) => {
                return *chunk.get_block((relative_position.x - ADJUSTED_CHUNK_SIZE) as usize, relative_position.y as usize, relative_position.z as usize);
            },
            None => { return Block::Empty },
        }
    }
    
    if relative_position.x < 0 {
        match chunk_tuple.2 {
            Some(chunk) => {
                return *chunk.get_block((relative_position.x + ADJUSTED_CHUNK_SIZE) as usize,  relative_position.y as usize, relative_position.z as usize);
            },
            None => { return Block::Empty },
        }
    }

    if relative_position.y > ADJUSTED_CHUNK_SIZE {
        match chunk_tuple.3 {
            Some(chunk) => {
                return *chunk.get_block(relative_position.x as usize, (relative_position.y - ADJUSTED_CHUNK_SIZE) as usize, relative_position.z as usize);
            },
            None => { return Block::Empty },
        }
    }
    
    if relative_position.y < 0 {
        match chunk_tuple.4 {
            Some(chunk) => {
                return *chunk.get_block(relative_position.x as usize,  (relative_position.y + ADJUSTED_CHUNK_SIZE) as usize, relative_position.z as usize);
            },
            None => { return Block::Empty },
        }
    }
    
    if relative_position.z > ADJUSTED_CHUNK_SIZE {
        match chunk_tuple.5 {
            Some(chunk) => {
                return *chunk.get_block(relative_position.x as usize, relative_position.y as usize, (relative_position.z - ADJUSTED_CHUNK_SIZE) as usize);
            },
            None => { return Block::Empty },
        }
    }
    
    if relative_position.z < 0 {
        match chunk_tuple.6 {
            Some(chunk) => {
                return *chunk.get_block(relative_position.x as usize,  relative_position.y as usize, (relative_position.z + ADJUSTED_CHUNK_SIZE) as usize);
            },
            None => { return Block::Empty },
        }
    }

    *chunk_tuple.0.get_block(relative_position.x as usize, relative_position.y as usize, relative_position.z as usize)
}

fn get_block_visibility(block: &Block, registry: &BlockRegistry, query: &Query<(Entity, &BlockEntity)>) -> MeshingVisibility {
    match block {
        Block::Empty => MeshingVisibility::Invisible,
        Block::Entity(entityid) => {
            match query.get(*entityid) {
                Ok(query_result) => query_result.1.visibility,
                Err(_) => MeshingVisibility::Invisible,
            }
        },
        Block::Generic(blockid) => {
            match registry.get_by_id(*blockid) {
                Some(result) => {
                    result.visibility()
                },
                None => MeshingVisibility::Invisible,
            }
        },
    }
}