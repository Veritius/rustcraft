use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bootleg_networking::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_discord_presence::config::{RPCConfig, RPCPlugin};

const MESSAGE_CHANNEL_ID: MessageChannelID = MessageChannelID::new(0);
const MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: MESSAGE_CHANNEL_ID.id,
    channel_mode: MessageChannelMode::Unreliable {
		settings: turbulence::unreliable_channel::Settings {
			bandwidth: 4096,
			burst_bandwidth: 1024,
		},
		max_message_len: 256,	
	},
    message_buffer_size: 256,
    packet_buffer_size: 256,
};

fn main() {
    App::new()
        // Game essentials
        .add_plugins(DefaultPlugins)

        // Networking
        .add_plugin(NetworkingPlugin)
        .add_startup_system(setup_network)
        .add_system(receive)
        
        // Console
        .add_plugin(ConsolePlugin)
        .add_console_command::<ConnectToRemoteServerCommand, _, _>(connect_to_remote_server_command)
        .add_console_command::<SendTestServerMessageCommand, _, _>(send_test_server_message_command)

        // Discord Rich Presence
        .add_plugin(RPCPlugin(RPCConfig {
            app_id: 987633651728666645,
            show_time: true
        }))
        
        // Start the game
        .run();
}

fn setup_network(mut commands: Commands, tokio_rt: Res<Runtime>, task_pool: Res<IoTaskPool>) {
    let mut net = NetworkResource::new_client(tokio_rt.clone(), task_pool.0.clone());

    let listen_config = ListenConfig {
        tcp_addr: "127.0.0.1:9000",
        udp_addr: "127.0.0.1:9001",
        naia_addr: "127.0.0.1:9002",
        webrtc_listen_addr: "127.0.0.1:9003",
        public_webrtc_listen_addr: "127.0.0.1:9004"
    };

    net.register_message_channel_native(MESSAGE_SETTINGS, &MESSAGE_CHANNEL_ID).unwrap();
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<String>(MESSAGE_SETTINGS)
            .unwrap();
    });

    commands.insert_resource(net);
}

fn receive(mut net: ResMut<NetworkResource>) {
    let messages = net.view_messages::<String>(&MESSAGE_CHANNEL_ID).unwrap();

    for (_handle, message) in messages.iter() {
        println!("Message: {}", message)
    }
}

#[derive(ConsoleCommand)]
#[console_command(name="join")]
struct ConnectToRemoteServerCommand {
    msg: String
}

fn connect_to_remote_server_command(mut log: ConsoleCommand<ConnectToRemoteServerCommand>) {
    if let Some(ConnectToRemoteServerCommand { msg }) = log.take() {
        println!("mfw");
    }
}

#[derive(ConsoleCommand)]
#[console_command(name="send")]
struct SendTestServerMessageCommand {
    msg: String
}

fn send_test_server_message_command(mut log: ConsoleCommand<SendTestServerMessageCommand>, mut net: ResMut<NetworkResource>) {
    if let Some(SendTestServerMessageCommand { msg }) = log.take() {
        net.broadcast_message(&msg, &MESSAGE_CHANNEL_ID).unwrap();
        println!("Trying to send message \"{}\" to remote server", msg);
    }
}