pub use error::{Error, Result};

use common::ui::UI;

use api_client::Client;

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of nodes for you...")?;
    ui.br()?;

    let results = rio_client.list_node(&token, &email)?;

    let title = row!["Id", "Name", "Status", "Unschedulable", "Hrs Ago"];

    pretty_table(results.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud nodes: \
         https://www.rioos.sh/docs/reference/deployment/",
    )?;

    ui.end(
        format!("{} records listed.", results.to_owned().len()),
    )?;
    Ok(())
}
