// Copyright 2018 The Rio Advancement Inc
//


use api_client::Client;
use common::ui::UI;
use error::Result;
use protocol::api::{deploy, devtool, scale};

use protocol::api::base::MetaFields;
use protocol::api::schema::type_meta_url;

use rioos_core::fs::open_from;
use serde_yaml;
use std::path::Path;

pub fn start(ui: &mut UI, rio_client: Client, cache_path: &str, token: &str, email: &str) -> Result<()> {
    ui.br()?;
    ui.begin("Constructing a cozy digitalcloud for you...")?;
    ui.br()?;
    let file = open_from(Path::new(cache_path))?;
    let content: DeployData = serde_yaml::from_reader(file)?;
    let assembly_fac: deploy::AssemblyFactory = rio_client.deploy_digicloud(
        content.stacks_factory.clone(),
        token,
        email,
    )?;

    if let Some(i) = content.horizontal_scaling {
        let mut hscale: scale::HorizontalScaling = i;
        let ref mut object_data = hscale.mut_meta(
            hscale.object_meta(),
            hscale.object_meta().name,
            hscale.object_meta().account,
        );

        hscale.set_owner_reference(
            object_data,
            assembly_fac.type_meta().kind,
            assembly_fac.type_meta().api_version,
            assembly_fac.object_meta().name,
            assembly_fac.get_id(),
        );
        hscale.set_meta(type_meta_url("".to_string()), object_data.clone());

        rio_client.create_horizontal_scaling(
            hscale.clone(),
            token,
            email,
        )?;
    }

    if let Some(i) = content.vertical_scaling {
        let mut vscale: scale::VerticalScaling = i;
        let ref mut object_data = vscale.mut_meta(
            vscale.object_meta(),
            vscale.object_meta().name,
            vscale.object_meta().account,
        );

        vscale.set_owner_reference(
            object_data,
            assembly_fac.type_meta().kind,
            assembly_fac.type_meta().api_version,
            assembly_fac.object_meta().name,
            assembly_fac.get_id(),
        );
        vscale.set_meta(type_meta_url("".to_string()), object_data.clone());

        rio_client.create_vertical_scaling(
            vscale.clone(),
            token,
            email,
        )?;
    }

    if let Some(i) = content.buildconfig {
        let mut build_config: devtool::BuildConfig = i;
        let ref mut object_data = build_config.mut_meta(
            build_config.object_meta(),
            build_config.object_meta().name,
            build_config.object_meta().account,
        );

        build_config.set_owner_reference(
            object_data,
            assembly_fac.type_meta().kind,
            assembly_fac.type_meta().api_version,
            assembly_fac.object_meta().name,
            assembly_fac.get_id(),
        );
        build_config.set_meta(type_meta_url("".to_string()), object_data.clone());

        rio_client.create_build_config(
            build_config.clone(),
            token,
            email,
        )?;
    }

    ui.end("Your digitalcloud is ready")?;
    ui.br()?;
    ui.para(
        "For more information on connecting to your digital \
         cloud: https://bit.ly/rioos_sh_usersguide",
    )?;
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
struct DeployData {
    stacks_factory: deploy::StacksFactory,
    horizontal_scaling: Option<scale::HorizontalScaling>,
    vertical_scaling: Option<scale::VerticalScaling>,
    buildconfig: Option<devtool::BuildConfig>,
}
