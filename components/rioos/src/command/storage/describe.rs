pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use {PRODUCT, VERSION};

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String, id: String) -> Result<()> {
    ui.begin(
        &format!("Constructing a {} storage for you...", id),
    )?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let storageconn = rio_client.get_storageconnector_by_id(&token, &email, &id)?;

    let storagepool = rio_client.get_storagepool_by_scid(&token, &email, &id)?;

    ui.br()?;
    ui.heading("StoragesConnector:")?;

    ui.para(&format!("Id: {}", storageconn.get_id()))?;
    ui.para(&format!("Name: {}", storageconn.get_name()))?;
    ui.para(&format!("Host IP : {}", storageconn.get_host_ip()))?;
    ui.para(
        &format!("Type : {}", storageconn.get_storage_type()),
    )?;
    ui.para(&format!(
        "Status: {}",
        storageconn.get_status().get_phase()
    ))?;
    ui.para(
        &format!("Hrs ago: {}", storageconn.get_created_at()),
    )?;

    ui.heading("StoragesPool list:")?;

    let title = row!["Id", "Name", "Status", "Hrs ago"];

    pretty_table(storagepool.to_owned(), title);
    ui.end(format!(
        "{} records listed.",
        storagepool.to_owned().len()
    ))?;

    ui.para(
        "For more information on storages: \
        https://www.rioos.sh/docs/reference/storages/",
    )?;

    Ok(())
}
