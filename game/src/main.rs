pub use rustcraft_modlib::engine;
pub use rustcraft_modlib::engine::bevy;

use bevy::prelude::*;
use rustcraft_modlib::ModPackageData;

#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct ModWrapper {
    metadata: extern fn() -> ModPackageData,
    entry_point: extern fn(app: &mut App),
}

fn main() {
    let mut packages = vec![];

    #[cfg(target_os="windows")]
    let extension = "dll";
    #[cfg(target_os="linux")]
    let extension = "so";
    #[cfg(target_os="macos")]
    let extension = "dylib";

    for package in glob::glob(&format!("mods/*.{extension}")).unwrap() {
        let package = package.unwrap();
        let package_wrapper: Container<ModWrapper> = unsafe { Container::load(package).unwrap() };
        let metadata = package_wrapper.metadata();
        packages.push(package_wrapper);
    }

    let mut app = App::new();
    for package in packages {
        package.entry_point(&mut app);
    }

    app.run();
}
