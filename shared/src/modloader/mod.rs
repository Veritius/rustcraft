pub mod package;

use bevy::app::{App, Plugin};
use package::PackageTable;

pub struct ModLoaderPlugin;

impl Plugin for ModLoaderPlugin {
    fn build(&self, app: &mut App) {
        let table = PackageTable::new();
        app.insert_resource(table);
    }
}