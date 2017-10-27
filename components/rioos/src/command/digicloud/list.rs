pub use error::{Error, Result};

use common::ui::UI;
use rioos_core::env;

use api_client::{self, Client};
use AUTH_TOKEN_ENVVAR;
use {PRODUCT, VERSION};
use config;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format::consts;
use prettytable::format;


pub fn start(ui: &mut UI, url: &str, token: String, email: String) -> Result<()> {
    let list = list(ui, url, &token, &email)?;
    Ok(())
}
fn list(ui: &mut UI, url: &str, token: &str, email: &str) -> Result<Vec<String>> {
    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    ui.br()?;

    ui.heading("List Deploy")?;

    ui.para("List Of AssemblyFactory")?;

    let result = rio_client.list_deploy(token, email)?;

    let mut table = Table::new();
    table.add_row(row!["id", "name", "created_at"]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
    // let table = table!(["id", "name", "created_at"], result);
    // table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    Ok(result)
}
