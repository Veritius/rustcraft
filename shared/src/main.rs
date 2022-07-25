pub use bevy;
pub use heron;
pub use ndarray;
pub use serde;
pub use fluent;
pub use log;
pub use yaml_rust;
pub use toml;

pub mod modloader;
pub mod localeplugin;
pub mod voxelplugin;

use log::info;
use bevy::{prelude::*};
use heron::PhysicsPlugin;
use bevy_console::ConsolePlugin;
use modloader::{ModLoaderPlugin, run_foreign_libraries};
use localeplugin::LocalePlugin;
use voxelplugin::VoxelPlugin;

fn main() {
    info!("Starting game");

    let mut app = App::new();

    // Modloader is loaded first
    app.add_plugin(ModLoaderPlugin);

    // Game essentials
    app.add_plugins(DefaultPlugins);
    app.insert_resource(WindowDescriptor {
        title: "Rustcraft vUnknown".to_string(),
        width: 500.,
        height: 300.,
        ..default()
    });
    app.add_plugin(PhysicsPlugin::default());

    // Fluent support
    app.add_plugin(LocalePlugin);

    // Voxel world
    app.add_plugin(VoxelPlugin);
        
    // Console
    app.add_plugin(ConsolePlugin);

    // Never call this more than once, and never right before app.start.
    run_foreign_libraries(&mut app, false);

    app.add_startup_system(spawn_camera);
       
    // Start the game
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-50.0, 25.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default() 
    });
    commands.spawn().insert_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });
}