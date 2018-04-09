pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String, id: String) -> Result<()> {
    ui.begin(
        &format!("Constructing a {} datacenter for you...", id),
    )?;
    ui.br()?;
    let result = rio_client.datacenter_get_by_id(&token, &email, &id)?;
    let title = row!["Id", "Name", "Enabled", "Status", "Hrs Ago"];
    pretty_table(result.to_owned(), title);

    ui.para(
        "For more information on digitalclouds datacenter: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    Ok(())
}
