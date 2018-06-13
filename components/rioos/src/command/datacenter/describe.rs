pub use error::{Error, Result};

use super::super::common::pretty_table;
use api_client::Client;
use common::ui::UI;
use protocol::api::base::{hours_ago, MetaFields};

pub fn start(
    ui: &mut UI,
    rio_client: Client,
    token: String,
    email: String,
    id: String,
) -> Result<()> {
    ui.begin(&format!("Constructing a {} datacenter for you...", id))?;
    ui.br()?;

    let result = rio_client.describe_datacenter(&token, &email, &id)?;

    ui.heading("OverView")?;
    ui.para(&format!("Id: {}", result.get_id()))?;
    ui.para(&format!("Name: {}", result.object_meta().name))?;
    ui.para(&format!("Status: {}", result.get_status().get_phase()))?;
    ui.para(&format!("Enabled : {}", result.get_enabled()))?;
    ui.para(&format!("Hrs ago: {}", hours_ago(result.get_created_at())))?;

    let storageconn = rio_client.get_storageconnector_by_id(&token, &email, &result.get_storage())?;

    ui.br()?;
    ui.heading("StoragesConnector:")?;

    ui.para(&format!("Id: {}", storageconn.get_id()))?;
    ui.para(&format!("Name: {}", storageconn.object_meta().name))?;
    ui.para(&format!("Host IP : {}", storageconn.get_host_ip()))?;
    ui.para(&format!("Type : {}", storageconn.get_storage_type()))?;
    ui.para(&format!("Status: {}", storageconn.get_status().get_phase()))?;

    ui.para(&format!(
        "Hrs ago: {}",
        hours_ago(storageconn.get_created_at())
    ))?;

    let mut storagepool =
        rio_client.get_storagepool_by_scid(&token, &email, &storageconn.get_id())?;
    let result = storagepool
        .iter_mut()
        .map(|i| {
            vec![
                i.get_id(),
                i.object_meta().name,
                i.get_status().get_phase(),
                hours_ago(i.get_created_at()),
            ]
        })
        .collect::<Vec<_>>();

    ui.heading("StoragesPool list:")?;

    let title = row!["Id", "Name", "Status", "Hrs ago"];

    pretty_table(result.to_owned(), title);
    ui.end(format!("{} records listed.", storagepool.to_owned().len()))?;

    ui.para(
        "For more information on datacenter: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    Ok(())
}
