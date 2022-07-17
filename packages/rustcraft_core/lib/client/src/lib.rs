use rustcraft_shared::bevy::app::{App, Plugin};

fn package() -> RustcraftCoreClient {
    RustcraftCoreClient {}
}

pub struct RustcraftCoreClient;

impl Plugin for RustcraftCoreClient {
    fn build(&self, app: &mut App) {

    }
}