use rustcraft_shared::bevy::app::{App, Plugin};

fn entry_point(app: &mut App) -> String {
    app.add_plugin(RustcraftCoreServer);
    String::from("RustcraftCoreServer")
}

pub struct RustcraftCoreServer;

impl Plugin for RustcraftCoreServer {
    fn build(&self, app: &mut App) {

    }
}