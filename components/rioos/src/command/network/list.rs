pub use error::{Error, Result};

use common::ui::UI;
use rioos_core::env;

use api_client::{self, Client};

use AUTH_TOKEN_ENVVAR;
use {PRODUCT, VERSION};
use config;

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of network for you...")?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let results = rio_client.list_network(&token, &email)?;

    let title = row!["Id", "Name", "Type","Subnet IP","Netmask","Gateway","Status", "Hrs Ago"];

    pretty_table(results.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud networks: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    ui.end(
        format!("{} records listed.", results.to_owned().len()),
    )?;
    Ok(())
}
