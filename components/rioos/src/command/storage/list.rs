pub use error::{Error, Result};

use common::ui::UI;
use rioos_core::env;

use api_client::{self, Client};

use AUTH_TOKEN_ENVVAR;
use {PRODUCT, VERSION};
use config;

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of storages for you...")?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let mut  results = rio_client.get_storageconnector(&token, &email)?;
    let mut value = results.get_items()
            .iter_mut()
            .map(|i| {

                vec![
                i.get_id(),
                i.get_storage_type(),
                i.get_host_ip(),
                i.get_storage_info().get_disks().iter().map(|d|{ format!("{}  ",d.get_size())}).collect(),
                // rio_client.get_storagepool_by_id(&token,&email,&(i.get_id())).unwrap().get_items().iter().map(|f| { f.get_storage_info().get_disks().iter().map(|d| {format!("{}  ",d.get_size())}).collect()}).collect()
                rio_client.get_storagepool_by_id(&token,&email,&(i.get_id())).unwrap().get_items().iter_mut().map(|f|{format!("{}  ",f.get_id())}).collect()
                 ]

            }).collect::<Vec<_>>();

    let title = row!["Id", "Type", "Stored At Server","Available Disk","Pool Id"];

    pretty_table(value, title);

    ui.br()?;

    ui.para(
        "For more information on Digitalcloud storages: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    // ui.end(
    //     format!("{} records listed.", value.to_owned().len()),
    // )?;
    Ok(())
}
