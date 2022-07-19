use std::{io::BufReader, fs};
use bevy::utils::HashSet;
use semver::{Version, VersionReq};
use serde::Deserialize;
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
                Ok(package_file) => {
                    let contents = fs::read_to_string(package_file.as_os_str());
                    match contents {
                        Ok(ver_contents) => {
                            let unverified_package_data = toml::from_str::<RustcraftPackageConfigDeser>(ver_contents.as_str());
                            match unverified_package_data {
                                Ok(deser_package_data) => {
                                    let mut folder = String::from(package_file.as_os_str().to_str().unwrap());
                                    folder.truncate(folder.len() - 13);

                                    // TODO: Rewrite this to be less shit.
                                    let id = deser_package_data.id;
                                    let name = deser_package_data.name;
                                    let desc = deser_package_data.desc;
                                    let mut authors = Vec::new();
                                    for item in deser_package_data.authors {
                                        authors.push(item.to_string());
                                    }
                                    let packageversion = Version::parse(&deser_package_data.packageversion).expect(format!("Invalid version field in {:#?}", &package_file).as_str());
                                    let gameversionreq = VersionReq::parse(&deser_package_data.gameversionreq).expect(format!("Invalid version field in {:#?}", &package_file).as_str());
                                    let mut libentrypoint = EntryPoints { client: None, server: None, shared: None };
                                    match &deser_package_data.libentrypoint {
                                        Some(lib_ep) => {
                                            let svr = lib_ep.get("server");
                                            let cli = lib_ep.get("client");
                                            let shr = lib_ep.get("shared");
                                            match svr {
                                                Some(val) => { libentrypoint.server = Some(val.to_string())}
                                                None => {}
                                            }
                                            match cli {
                                                Some(val) => { libentrypoint.client = Some(val.to_string())}
                                                None => {}
                                            }
                                            match shr {
                                                Some(val) => { libentrypoint.shared = Some(val.to_string())}
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                    let mut dependencies: Option<Vec<String>> = None;
                                    match deser_package_data.dependencies {
                                        Some(v) => {
                                            let mut array = Vec::<String>::new();
                                            for item in v {
                                                array.push(item.to_string());
                                            }
                                            dependencies = Some(array);
                                        }
                                        None => {}
                                    }
                                    let mut incompatibilities: Option<Vec<String>> = None;
                                    match deser_package_data.incompatibilities {
                                        Some(v) => {
                                            let mut array = Vec::<String>::new();
                                            for item in v {
                                                array.push(item.to_string());
                                            }
                                            incompatibilities = Some(array);
                                        }
                                        None => {}
                                    }

                                    // Done
                                    table.push(RustcraftPackage { path: folder, config: RustcraftPackageConfig { id, name, desc, authors, packageversion, gameversionreq, libentrypoint, dependencies, incompatibilities }});
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
    pub config: RustcraftPackageConfig
}

/// Config information for a package
pub struct RustcraftPackageConfig {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub authors: Vec<String>,
    pub packageversion: Version,
    pub gameversionreq: VersionReq,
    pub libentrypoint: EntryPoints,
    pub dependencies: Option<Vec<String>>,
    pub incompatibilities: Option<Vec<String>>,
}

#[derive(Deserialize)]
/// Deserialised information from TOML file
/// Not actually used in code outside here
struct RustcraftPackageConfigDeser {
    id: String,
    name: String,
    desc: String,
    authors: Array,
    packageversion: String,
    gameversionreq: String,
    libentrypoint: Option<Table>,
    dependencies: Option<Array>,
    incompatibilities: Option<Array>,
}

pub struct EntryPoints {
    pub client: Option<String>,
    pub server: Option<String>,
    pub shared: Option<String>
}