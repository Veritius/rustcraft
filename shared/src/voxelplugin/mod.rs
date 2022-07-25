use bevy::{prelude::*, app::{App, Plugin}};

mod chunk;
mod voxel;
mod events;
mod mesh;

use voxel::VoxelDataTable;
use chunk::ChunkManager;
use events::*;
use mesh::generate_visual_mesh;

use self::voxel::{Voxel, SimpleVoxel};

/// An implementation for a voxel world system
pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        //No multi-world/dimensions support (yet).
        app.init_resource::<VoxelDataTable>();
        app.init_resource::<ChunkManager>();
        
        //TODO: Put this in events.rs
        app.add_event::<BlockUpdateEvent>();
        app.add_event::<BlockRemovalAttemptEvent>();
        app.add_event::<BlockRemovalEvent>();
        app.add_event::<BlockPlacementAttemptEvent>();
        app.add_event::<BlockPlacementEvent>();
        app.add_event::<BlockReplacementAttemptEvent>();
        app.add_event::<BlockReplacementEvent>();

        app.add_startup_system(make_a_chunk);
        app.add_startup_system(do_a_thing);
    }
}

fn make_a_chunk(mut commands: Commands, mut cm: ResMut<ChunkManager>) {
    cm.chunks.add_chunk(commands, 0, 0, 0);
    let mut chunk = cm.chunks.get_chunk_mut(0, 0, 0).unwrap().unwrap();
    chunk.set_voxel(0, 0, 0, Some(Voxel::SimpleVoxel({SimpleVoxel {id: 0}})));
    chunk.set_voxel(8, 8, 8, Some(Voxel::SimpleVoxel({SimpleVoxel {id: 0}})));
    chunk.set_voxel(16, 16, 16, Some(Voxel::SimpleVoxel({SimpleVoxel {id: 0}})));
    for cx in chunk.voxels {
        for cy in cx {
            for cz in cy {
                match cz {
                    Some(vox) => {
                        println!("{:?}", vox);
                    }
                    None => {}
                }
            }
        }
    }
}

fn do_a_thing(mut commands: Commands, tbl: Res<VoxelDataTable>, mut cm: ResMut<ChunkManager>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let table = &mut cm.chunks.tbl;
    for (ai, at) in table.into_iter() {
        for (bi, bt) in at.into_iter() {
            for (ci, bt) in bt.into_iter() {
                let mesh = generate_visual_mesh(&tbl, &bt.unwrap());
                let entity = bt.unwrap().entity;
                commands.get_or_spawn(entity).insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(mesh)),
                    material: materials.add(Color::RED.into()),
                    ..default()
                });
            }
        }
    }
}