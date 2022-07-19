pub mod package;

use bevy::app::{App, Plugin};
use package::{RustcraftPackage, PackageTable};

pub struct ModLoaderPlugin;

impl Plugin for ModLoaderPlugin {
    fn build(&self, app: &mut App) {
        let table = PackageTable::new();
        app.insert_resource(table);
    }
}

/// Runs foreign libraries from packages. Never call more than once.
pub fn run_foreign_libraries(app: &mut App, is_server: bool) {
    let packagetable = &app.world.get_resource::<PackageTable>().expect("No package table found!").table;
    for package in packagetable.into_iter() {
        let server = &package.config.libentrypoint.server;
        let client = &package.config.libentrypoint.client;
        let shared = &package.config.libentrypoint.shared;

        let mut paths = Vec::<String>::new();

        // Shared
        match shared {
            Some(path) => { paths.push(format!("{}/lib/{}", package.path, path)); }
            None => {}
        }

        if is_server {
            // Server
            match server {
                Some(path) => { paths.push(format!("{}/lib/{}", package.path, path)); }
                None => {}
            }
        }
        else
        {
            // Client
            match client {
                Some(path) => { paths.push(format!("{}/lib/{}", package.path, path)); }
                None => {}
            }
        }
    }
}