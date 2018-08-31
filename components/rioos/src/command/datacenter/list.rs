pub use error::{Error, Result};

use super::super::common::pretty_table;
use api_client::Client;
use common::ui::UI;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of datacenter for you...")?;
    ui.br()?;

    let results = rio_client.list_datacenters(&token, &email)?;

    let title = row!["Id", "Name", "Enabled", "Status", "Hrs Ago"];

    pretty_table(results.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud datacenter: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    ui.end(format!("{} records listed.", results.to_owned().len()))?;
    Ok(())
}
