// Copyright (c) 2017 RioCorp Inc.
//


use std::env;
use std::fs::create_dir_all;
use std::fs::{File, canonicalize};

use std::io::Write;
use std::path::Path;
use std::collections::HashMap;

use handlebars::Handlebars;

use common::ui::{UI, Status};
use error::Result;

const DEFAULT_RIOBLU_TEMPLATE: &'static str = ""; //include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rioblu.yaml"));

pub fn start(ui: &mut UI, _token: String, maybe_name: Option<String>) -> Result<()> {
    ui.begin("Constructing a cozy digitalcloud for you...")?;
    ui.br()?;

    let (root, _name) = match maybe_name {
        Some(name) => (name.clone(), name.clone()),
        // load the yaml file and call the api.
        None => {
            // try loading the default rioblu.yaml from the current directory
            (
                "habitat".into(),
                canonicalize(".")
                    .ok()
                    .and_then(|path| {
                        path.components().last().and_then(|val| {
                            // Type gymnastics!
                            val.as_os_str().to_os_string().into_string().ok()
                        })
                    })
                    .unwrap_or("unnamed".into()),
            )
        }
    };

    // Build out the variables passed.
    let handlebars = Handlebars::new();
    let mut data = HashMap::new();
    let location = "test".to_string();
    let origin = "default".to_string();
    data.insert("location".to_string(), location);
    data.insert("origin".to_string(), origin);

    // Add all environment variables that start with "rio_" as variables in
    // the template.
    for (key, value) in env::vars() {
        if key.starts_with("rio_") {
            data.insert(key, value);
        }
    }

    let _rendered_blu_yaml = handlebars.template_render(DEFAULT_RIOBLU_TEMPLATE, &data)?;
    let rendered_plan = "";
    create_with_template(ui, &format!("{}/rioblu.yaml", root), &rendered_plan)?;

    ui.para(
        "`rioblue.yaml` is the foundation of your new digital cloud. It contains \
        declaration for your cloud os.",
    )?;

    let config_path = format!("{}/config/", root);
    match Path::new(&config_path).exists() {
        true => {
            ui.status(
                Status::Using,
                format!("existing directory: {}", config_path),
            )?
        }
        false => {
            ui.status(
                Status::Creating,
                format!("directory: {}", config_path),
            )?;
            create_dir_all(&config_path)?;
        }
    };
    ui.para(
        "`/config/` contains configuration files for your app.",
    )?;


    ui.para(
        "For more information on any of the files: \
        https://www.rioos.sh/docs/reference/blu-syntax/",
    )?;


    /*let api_client = Client::new(url, PRODUCT, VERSION, None)?;
    let ident = "";
    let abcd = "";
    ui.begin(format!("Applying {} from {}", ident, abcd))?;

    let ident ="";
    let channel ="";
    let token = "";

    match api_client.apply_blu(ident, channel, token) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to apply '{}': {:?}", ident, e);
            return Err(Error::from(e));
        }
    }

    ui.status(Status::Applied, ident)?;
    */
    Ok(())
}

fn create_with_template(ui: &mut UI, location: &str, template: &str) -> Result<()> {
    let path = Path::new(&location);
    match path.exists() {
        false => {
            ui.status(Status::Creating, format!("file: {}", location))?;
            // If the directory doesn't exist we need to make it.
            if let Some(directory) = path.parent() {
                create_dir_all(directory)?;
            }
            // Create and then render the template with Handlebars
            File::create(path).and_then(
                |mut file| file.write(template.as_bytes()),
            )?;
        }
        true => {
            // If the user has already configured a file overwriting would be impolite.
            ui.status(
                Status::Using,
                format!("existing file: {}", location),
            )?;
        }
    };
    Ok(())
}
