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
use prettytable::format;

pub fn start(ui: &mut UI, url: &str, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of digitalcloud for you...")?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let results = rio_client.list_deploy(&token, &email)?;

    let rows = results
        .clone()
        .iter()
        .map(|result| {
            let r = result
                .iter()
                .map(|col| Cell::new(&col).style_spec("Fbcl"))
                .collect::<Vec<_>>();
            Row::new(r.clone())
        })
        .collect::<Vec<_>>();

    let mut table = Table::init(rows);
    table.set_titles(row![
        "Id",
        "Name",
        "Replicas",
        "Located",
        "Origin",
        "Hrs ago"
    ]);

    table.set_format(*format::consts::FORMAT_NO_COLSEP);
    table.printstd();

    ui.para(
        "For more information on digitalclouds deployments: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    ui.end(format!("{} records listed.", results.len()))?;
    Ok(())
}
