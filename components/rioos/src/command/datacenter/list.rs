pub use error::{Error, Result};

use common::ui::UI;
use api_client::{Client};
use {PRODUCT, VERSION};

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of datacenter for you...")?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let results = rio_client.list_datacenters(&token, &email)?;

    let title = row!["Id", "Name","Enabled", "Status", "Hrs Ago"];

    pretty_table(results.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud datacenter: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    ui.end(
        format!("{} records listed.", results.to_owned().len()),
    )?;
    Ok(())
}
