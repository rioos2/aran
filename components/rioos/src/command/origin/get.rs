pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use {PRODUCT, VERSION};
use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String, name: String) -> Result<()> {
    ui.begin(
        &format!("Constructing a {} origin for you...", name),
    )?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let result = rio_client.origin_get(&token, &email, &name)?;
    let title = row!["Id", "Name", "Creator Id", "Hrs Ago"];
    pretty_table(result.to_owned(), title);

    ui.para(
        "For more information on digitalclouds origin: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}
