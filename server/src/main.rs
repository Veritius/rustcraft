use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bootleg_networking::*;

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
        
        // Start the game
        .run();
}

fn setup_network(mut commands: Commands, tokio_rt: Res<Runtime>, task_pool: Res<IoTaskPool>) {
    let mut net = NetworkResource::new_server(tokio_rt.clone(), task_pool.0.clone());

    let listen_config = ListenConfig {
        tcp_addr: "127.0.0.1:9000",
        udp_addr: "127.0.0.1:9001",
        naia_addr: "127.0.0.1:9002",
        webrtc_listen_addr: "127.0.0.1:9003",
        public_webrtc_listen_addr: "127.0.0.1:9004"
    };

    net.listen(listen_config, Some(2048));

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