use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};
use bevy_math::f32::Vec3;
use heron::prelude::*;

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};

use rustcraft_shared::{protocol::Protocol, shared_config, Channels};

mod resources;
mod systems;

use systems::{events, init, tick};

fn main() {
    info!("Starting game server");

    // Build App
    App::new()
        // Game essentials
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))

        // Physics
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))

        .add_startup_system(init)
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::Tick, tick)

        // Start the game
        .run();
}
