pub use error::{Error, Result};

use common::ui::UI;
use api_client::{Client};

use {PRODUCT, VERSION};

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String, id: String) -> Result<()> {
    ui.begin(&format!(
        "Constructing a {} datacenter for you...",
        id
    ))?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let result = rio_client.datacenter_get_by_id(&token, &email, &id)?;
    let title = row!["Id", "Name","Enabled", "Status", "Hrs Ago"];
    pretty_table(result.to_owned(), title);

    ui.para(
        "For more information on digitalclouds datacenter: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}
