use rustcraft_shared::bevy::{prelude::*, ecs::system::Commands, log::info, render::camera::OrthographicCameraBundle};

use naia_bevy_client::Client;

use rustcraft_shared::{
    protocol::{Auth, Protocol},
    Channels,
};

use crate::resources::Global;

pub fn init(mut commands: Commands, mut client: Client<Protocol, Channels>) {
    info!("Naia Bevy Client Demo started");

    client.auth(Auth::new("charlie", "12345"));
    client.connect("http://127.0.0.1:14191");

    // Point light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });

    // Setup Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Setup Colors
    commands.init_resource::<Global>();
}
