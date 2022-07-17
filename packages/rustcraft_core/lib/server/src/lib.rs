use rustcraft_shared::bevy::app::{App, Plugin};

fn package() -> RustcraftCoreServer {
    RustcraftCoreServer {}
}

pub struct RustcraftCoreServer;

impl Plugin for RustcraftCoreServer {
    fn build(&self, app: &mut App) {

    }
}