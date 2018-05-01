pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use super::super::common::pretty_table;
use protocol::api::base::MetaFields;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String, id: String) -> Result<()> {
    ui.begin(
        &format!("Constructing a {} storage for you...", id),
    )?;
    ui.br()?;

    let storageconn = rio_client.get_storageconnector_by_id(&token, &email, &id)?;

    let mut storagepool = rio_client.get_storagepool_by_scid(&token, &email, &id)?;

    let result = storagepool
        .iter_mut()
        .map(|i| {
            vec![
                i.get_id(),
                i.object_meta().name,
                i.get_status().get_phase(),
                i.get_created_at(),
            ]
        })
        .collect::<Vec<_>>();
    ui.br()?;
    ui.heading("StoragesConnector:")?;

    ui.para(&format!("Id: {}", storageconn.get_id()))?;
    ui.para(
        &format!("Name: {}", storageconn.object_meta().name),
    )?;
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

    pretty_table(result.to_owned(), title);
    ui.end(format!(
        "{} records listed.",
        storagepool.to_owned().len()
    ))?;

    ui.para(
        "For more information on storages: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    Ok(())
}
