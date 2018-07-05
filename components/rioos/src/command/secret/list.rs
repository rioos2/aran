pub use error::{Error, Result};

use api_client::Client;
use common::ui::UI;
use config;

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of secret for you...")?;
    ui.br()?;
    let results = rio_client.list_secret(&token, &email, &get_account().to_string())?;

    let title = row!["Id", "Name", "Secret Type", "Hrs Ago"];

    pretty_table(results.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud secrets: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    ui.end(format!("{} records listed.", results.to_owned().len()))?;
    Ok(())
}

fn get_account() -> String {
    let config = config::load().unwrap();
    config.account.unwrap()
}
