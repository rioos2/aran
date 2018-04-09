pub use error::{Error, Result};

use common::ui::UI;

use api_client::Client;

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of origins for you...")?;
    ui.br()?;
    let results = rio_client.list_origins(&token, &email)?;

    let title = row!["Id", "Name", "Creator Id", "Hrs Ago"];

    pretty_table(results.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud origins: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    ui.end(
        format!("{} records listed.", results.to_owned().len()),
    )?;
    Ok(())
}
