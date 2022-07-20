use rustcraft_shared::bevy::app::{App, Plugin};

fn entry_point(app: &mut App) -> String {
    app.add_plugin(RustcraftCoreClient);
    String::from("RustcraftCoreClient")
}

pub struct RustcraftCoreClient;

impl Plugin for RustcraftCoreClient {
    fn build(&self, app: &mut App) {

    }
}