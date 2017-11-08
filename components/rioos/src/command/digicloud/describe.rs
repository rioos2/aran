pub use error::{Error, Result};

use common::ui::UI;

use api_client::Client;
use {PRODUCT, VERSION};

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String, name: String) -> Result<()> {
    ui.begin(&format!(
        "Constructing a {} digitalcloud for you...",
        name
    ))?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let result = rio_client.describe_deploy(&token, &email, &name)?;

    ui.heading("OverView")?;
    ui.para(&format!("Id: {}", result.get_id()))?;
    ui.para(&format!("Name: {}", result.get_name()))?;
    ui.para(&format!(
        "Replicas: {}",
        result.get_replicas().to_string()
    ))?;
    ui.para(
        &format!("Status: {}", result.get_status().get_phase()),
    )?;
    ui.para(&format!(
        "Located: {}",
        result.get_properties().clone().get_region()
    ))?;
    ui.para(&format!("Origin: {}", result.get_origin()))?;
    ui.para(&format!("Hrs ago: {}", result.get_created_at()))?;

    let replicas = rio_client.get_assembly_by_id(
        &token,
        &email,
        &result.get_id(),
    )?;

    ui.heading("Replicas")?;
    let title = row!["Id", "Name", "Status", "Origin", "Hrs ago"];

    pretty_table(replicas.to_owned(), title);

    ui.br()?;

    ui.end(
        format!("{} records listed.", replicas.to_owned().len()),
    )?;

    ui.para(
        "For more information on digitalclouds deployments: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}
