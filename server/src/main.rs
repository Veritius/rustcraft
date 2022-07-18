pub use naia_bevy_server;

use rustcraft_shared::bevy::core::CorePlugin;
use rustcraft_shared::bevy::app::{App, ScheduleRunnerPlugin};
use rustcraft_shared::bevy::ecs::{system::Command, prelude::Commands};
use rustcraft_shared::bevy::log::{info, LogPlugin};
use rustcraft_shared::bevy::math::f32::Vec3;
use rustcraft_shared::{voxelplugin::VoxelPlugin, localeplugin::LocalePlugin};
use rustcraft_shared::heron::prelude::*;

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use rustcraft_shared::naia_shared::Property;

use rustcraft_shared::{protocol::GridFixture};
use rustcraft_shared::{protocol::Protocol, protocol::Position, shared_config, Channels};

mod resources;
mod systems;

use systems::{events, init, tick};

fn main() {
    info!("Starting game server");

    // Make app
    let mut app = App::new();

    // Game essentials
    app.add_plugin(CorePlugin::default());
    app.add_plugin(ScheduleRunnerPlugin::default());
    app.add_plugin(LogPlugin::default());

    // Physics
    app.add_plugin(PhysicsPlugin::default());
    app.insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)));

    // Voxel world
    app.add_plugin(VoxelPlugin);

    // Fluent support
    app.add_plugin(LocalePlugin);

    // Networking
    app.add_plugin(ServerPlugin::<Protocol, Channels>::new(
        ServerConfig::default(),
        shared_config(),
    ));
    app.add_startup_system(init);
    app.add_system_to_stage(Stage::ReceiveEvents, events::authorization_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::connection_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event);
    app.add_system_to_stage(Stage::Tick, tick);

    // Start the game
    app.run();
}
