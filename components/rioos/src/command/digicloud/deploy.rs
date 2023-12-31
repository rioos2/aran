// Copyright 2018 The Rio Advancement Inc
//
use std::fs::File;
use serde_yaml;
use common::ui::UI;
use error::Result;
use api_client::Client;
use protocol::api::{deploy, scale};
use rio_net::http::schema::type_meta_url;
use protocol::api::base::MetaFields;

pub fn start(ui: &mut UI, rio_client: Client, cache_path: &str, token: &str, email: &str) -> Result<()> {
    ui.br()?;
    ui.begin("Constructing a cozy digitalcloud for you...")?;
    ui.br()?;
    let file = File::open(cache_path)?;
    let content: DeployData = serde_yaml::from_reader(file)?;
    let assembly_fac: deploy::AssemblyFactory = rio_client.deploy_digicloud(
        content.assembly_factory.clone(),
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
            hscale.object_meta().name,
            assembly_fac.get_id(),
        );
        hscale.set_meta(type_meta_url("".to_string()), object_data.clone());

        rio_client.create_horizontal_scaling(
            hscale.clone(),
            token,
            email,
        )?;
    }

    ui.end("Your digitalcloud is ready")?;
    ui.br()?;
    ui.para(
        "For more information on connecting to your digital cloud: \
        https://bit.ly/rioos_sh_usersguide",
    )?;
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
struct DeployData {
    assembly_factory: deploy::AssemblyFactory,
    horizontal_scaling: Option<scale::HorizontalScaling>,
}
