use bevy::{app::App, DefaultPlugins};
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_discord_presence::config::{RPCConfig, RPCPlugin};
use rustcraft_shared::{protocol::Protocol, shared_config, Channels};
use crate::systems::{events, init, input, sync, tick};

mod systems;
mod resources;

fn main() {
    App::new()
        // Game essentials
        .add_plugins(DefaultPlugins)
        
        // Networking
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        
        // Startup System
        .add_startup_system(systems::init)
        // Realtime Gameplay Loop
        .add_system_to_stage(Stage::Connection, events::connect_event)
        .add_system_to_stage(Stage::Disconnection, events::disconnect_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::spawn_entity_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::insert_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::update_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::Frame, input)
        .add_system_to_stage(Stage::PostFrame, sync)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        
        // Console
        .add_plugin(ConsolePlugin)

        // Discord Rich Presence
        .add_plugin(RPCPlugin(RPCConfig {
            app_id: 987633651728666645,
            show_time: true
        }))
        
        // Start the game
        .run();
}