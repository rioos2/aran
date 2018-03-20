pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use super::super::common::pretty_table;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String, name: String) -> Result<()> {
    ui.begin(
        &format!("Constructing a {} origin for you...", name),
    )?;
    ui.br()?;

    let result = rio_client.origin_get(&token, &email, &name)?;
    let title = row!["Id", "Name", "Creator Id", "Hrs Ago"];
    pretty_table(result.to_owned(), title);

    ui.para(
        "For more information on digitalclouds origin: \
         https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}
