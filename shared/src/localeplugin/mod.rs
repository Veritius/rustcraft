use bevy::app::{App, Plugin};
use glob::glob;
use std::fs::File;
use std::io::{BufRead, BufReader};
use fluent::{bundle::FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use super::modloader::package::PackageTable;
use log::{warn, error};

/// An implementation for a voxel world system
pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, app: &mut App) {
        //TODO: Allow changing locale
        let loc = Locale::new(app, "en_US");
        app.insert_resource(loc);
    }
}

pub struct Locale {
    bundle: FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>,
}

impl Locale {
    pub fn new(app: &App, chosen_locale: &str) -> Locale {
        let packagetable = app.world.get_resource::<PackageTable>().expect("No package table found!");
        let langid: LanguageIdentifier = chosen_locale.parse().expect("Invalid LanguageIdentifier");
        let locale_as_string = langid.to_string();
        let mut bundle = FluentBundle::new_concurrent(vec![langid]);

        for package in &packagetable.table {
            let locdir = format!("{}/locale/{}/**/*.ftl", package.path.to_owned(), &locale_as_string);

            // Loop over files
            for path in glob(&locdir).expect("Error in glob pattern") {
                match path {
                    Ok(ftl_file) => {
                        // Open file
                        let file = File::open(ftl_file);
                        match file {
                            Ok(_) => {}
                            Err(_) => { continue; }
                        }
                        
                        let validated_file = file.unwrap();
                        let file_path = format!("{:?}", validated_file);
                        let file_path_str = file_path.as_str();
                        let reader = BufReader::new(validated_file);

                        // Loop over every line
                        for (index, line) in reader.lines().enumerate() {
                            let line = line.unwrap();
                            let res = FluentResource::try_new(line);
                            match res {
                                Ok(resource) => { 
                                    // Try to make FluentResource
                                    let result = bundle.add_resource(resource);
                                    match result {
                                        Ok(_) => {}
                                        Err(_) => {
                                            warn!(target:"LocalePlugin", "Failed to add a Fluent string ({} line {})", &file_path_str, index.to_string().as_str())
                                        }
                                    }
                                }
                                Err(error) => {
                                    error!(target:"LocalePlugin", "Fluent error at {} line {}: {:?}", &file_path_str, index.to_string().as_str(), error)
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }

        Locale { bundle }
    }
}