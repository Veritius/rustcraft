use bevy_app::{App, Plugin};
use glob::glob;
use std::fs::File;
use std::io::{BufRead, BufReader};
use fluent::{bundle::FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use log::warn;

/// An implementation for a voxel world system
pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, app: &mut App) {
        //TODO: Allow changing locale
        let loc = Locale::new("en_US");
        app.insert_resource(loc);
    }
}

pub struct Locale<T> {
    bundle: FluentBundle<FluentResource, T>,
}

impl Locale<T> {
    pub fn new(chosen_locale: &str) -> Locale<T> {
        let langid: LanguageIdentifier = chosen_locale.parse().expect("Invalid LanguageIdentifier");
        let locale_as_string = langid.to_string();
        let mut bundle = FluentBundle::new_concurrent(vec![langid]);

        //"./resources/*/locale/{chosen_locale}/**.ftl"
        let mut locdir = String::from("./resources/*/locale/");
        locdir.push_str(&locale_as_string);
        locdir.push_str("/**.ftl");
        
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
                                        // TODO: Surely there's a better approach than this
                                        let error_message = String::from("Failed to add a Fluent string (");
                                        error_message.push_str(&file_path_str);
                                        error_message.push_str(" line ");
                                        error_message.push_str(index.to_string().as_str());
                                        error_message.push_str(")");
                                        warn!(target:"LocalePlugin", "{}", error_message) }
                                }
                            }
                            Err(error) => {
                                let formatted_error = format!("{:?}", error);
                                let error_message = String::from("Fluent error at ");
                                error_message.push_str(&file_path_str);
                                error_message.push_str(" line ");
                                error_message.push_str(index.to_string().as_str());
                                error_message.push_str(": ");
                                error_message.push_str(&formatted_error);
                                warn!(target:"LocalePlugin", "{}", error_message)
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }

        Locale { bundle }
    }
}