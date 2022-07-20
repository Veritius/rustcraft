use rustcraft_shared::bevy::app::{App, Plugin};

fn entry_point(app: &mut App) -> String {
    app.add_plugin(RustcraftCoreShared);
    String::from("RustcraftCoreShared")
}

pub struct RustcraftCoreShared;

impl Plugin for RustcraftCoreShared {
    fn build(&self, app: &mut App) {

    }
}