use std::{io::BufReader, fs};
use bevy::utils::HashSet;
use serde::{Serialize, Deserialize};
use glob::glob;
use toml::value::{Table, Array};
use log::{warn, info, error};

// A array of the currently installed packages
pub struct PackageTable {
    pub table: Vec<RustcraftPackage>
}

impl PackageTable {
    pub fn new() -> PackageTable {
        // Load all packages
        let mut table = Vec::new();

        let pattern = "packages/*/package.toml";
        for path in glob(&pattern).expect("Error in glob pattern") {
            match path {
                Ok(ftl_file) => {
                    let contents = fs::read_to_string(ftl_file.as_os_str());
                    match contents {
                        Ok(ver_contents) => {
                            let unverified_package_data = toml::from_str(ver_contents.as_str());
                            match unverified_package_data {
                                Ok(package_data) => {
                                    let mut folder = String::from(ftl_file.as_os_str().to_str().unwrap());
                                    folder.truncate(folder.len() - 13);
                                    table.push(RustcraftPackage { path: folder, config: package_data });
                                }
                                Err(error_message) => {
                                    let mut message_contents = String::from("Invalid package.toml ");
                                    message_contents.push_str(format!("{}", error_message).as_str());
                                    error!("{}", message_contents)
                                }
                            }
                        }
                        Err(error_message) => {
                            error!("Package path is inaccessible {}", error_message)
                        }
                    }
                }
                Err(error_message) => {
                    error!("Glob error: {}", error_message);
                }
            }
        }

        // TODO: Check dependencies/incompatibilities

        let packagetable = PackageTable { table };
        return packagetable;
    }
}

pub struct RustcraftPackage {
    pub path: String,
    config: RustcraftPackageConfig
}

#[derive(Serialize, Deserialize)]
pub struct RustcraftPackageConfig {
    id: String,
    name: String,
    desc: String,
    authors: Array,
    packageversion: String,
    gameversionrange: Table,
    dependencies: Option<Array>,
    incompatibilities: Option<Array>,
}