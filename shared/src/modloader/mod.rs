pub mod package;

use bevy::app::{App, Plugin};
use libloading::{Library, Symbol};
use log::debug;
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
    let mut packagemanager = app.world.get_resource_mut::<PackageTable>().expect("No package table found!");
    let mut paths = Vec::<String>::new();

    let packagetable = &mut packagemanager.table;

    // Read external library entry points
    for package in packagetable.into_iter() {
        let server = &package.config.libentrypoint.server;
        let client = &package.config.libentrypoint.client;
        let shared = &package.config.libentrypoint.shared;

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

    // Drop packagemanager
    let _packagemanager = "";
    let mut packagelibraries = Vec::<Library>::new();

    // Load external libraries
    for path in paths {
        debug!("Loading library {}", path);
        unsafe {
            let lib = Library::new(path).unwrap();
            packagelibraries.push(lib);
            let lib = packagelibraries.last().unwrap();
            let entrypoint: Symbol<EntryPointFunc> = lib.get(b"entry_point").unwrap();
            entrypoint(app);
            debug!("Loaded library successfully");
        }
    }

    // Get packagemanager again
    let mut packagemanager = app.world.get_resource_mut::<PackageTable>().expect("No package table found!");
    packagemanager.libraries.append(&mut packagelibraries);
}

//Entry point function, called on dynamic libraries in packages
//Function should be: entry_point(app: &mut App) -> String
//where the String is the name of the package
//
//Looks like this: (example taken from packages/rustcraft_core/lib/server/src/lib.rs)
//fn entry_point(app: &mut App) -> String {
//    app.add_plugin(RustcraftCoreServer);
//    String::from("RustcraftCoreServer")
//}
pub type EntryPointFunc = unsafe fn(app: &mut App) -> String;