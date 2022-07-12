use bevy::{prelude::*, app::App, DefaultPlugins, window::PresentMode, ecs::system::Command};
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};
use heron::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_discord_presence::config::{RPCConfig, RPCPlugin};
use rustcraft_shared::{protocol::Protocol, shared_config, Channels};
use crate::systems::{events, init, input, sync, tick};

mod command_history; //hack
mod systems;
mod resources;

fn main() {
    info!("Starting client");

    let mut app = App::new();
    // Game essentials
    app.add_plugins(DefaultPlugins);
    app.insert_resource(WindowDescriptor {
        title: "Rustcraft vUnknown".to_string(),
        width: 500.,
        height: 300.,
        present_mode: PresentMode::Fifo,
        ..default()
    });
    app.add_plugin(PhysicsPlugin::default());
    
    // Networking
    app.add_plugin(ClientPlugin::<Protocol, Channels>::new(
        ClientConfig::default(),
        shared_config(),
    ));
    
    // Startup System
    app.add_startup_system(systems::init);
    // Realtime Gameplay Loop
    app.add_system_to_stage(Stage::Connection, events::connect_event);
    app.add_system_to_stage(Stage::Disconnection, events::disconnect_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::spawn_entity_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::insert_component_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::update_component_event);
    app.add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event);
    app.add_system_to_stage(Stage::Frame, input);
    app.add_system_to_stage(Stage::PostFrame, sync);
    // Gameplay Loop on Tick
    app.add_system_to_stage(Stage::Tick, tick);
        
    // Console
    app.add_plugin(ConsolePlugin);

    // Discord Rich Presence
    app.add_plugin(RPCPlugin(RPCConfig {
        app_id: 987633651728666645,
        show_time: true
    }));
       
    // Start the game
    app.run();
}