pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use protocol::api::storage::DataCenter;
use protocol::api::base::MetaFields;


use super::super::common::pretty_table;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String, id: String) -> Result<()> {
    ui.begin(
        &format!("Constructing a {} datacenter for you...", id),
    )?;
    ui.br()?;
    let dc: DataCenter = rio_client.datacenter_get_by_id(&token, &email, &id)?;
    let title = row!["Id", "Name", "Enabled", "Status", "Hrs Ago"];
    let data = vec![
        vec![
            dc.get_id(),
            dc.object_meta().name,
            dc.get_enabled().to_string(),
            dc.get_status().get_phase(),
            dc.get_created_at(),
        ],
    ];
    pretty_table(data.to_owned(), title);

    ui.para(
        "For more information on digitalclouds datacenter: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    Ok(())
}
