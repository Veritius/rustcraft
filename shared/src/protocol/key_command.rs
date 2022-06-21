use bevy_ecs::component::Component;
use naia_shared::{EntityProperty, Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct KeyCommand {
    pub entity: EntityProperty,
    pub w: Property<bool>,
    pub s: Property<bool>,
    pub a: Property<bool>,
    pub d: Property<bool>,
    pub q: Property<bool>,
    pub e: Property<bool>
}

impl KeyCommand {
    pub fn new(w: bool, s: bool, a: bool, d: bool, q: bool, e: bool) -> Self {
        KeyCommand::new_complete(w, s, a, d, q, e)
    }
}
