use bevy::ecs::component::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GridFixture {
    pub x: Property<i64>,
    pub y: Property<i64>,
    pub z: Property<i64>,
}

impl GridFixture {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        GridFixture::new_complete(x, y, z)
    }
}
