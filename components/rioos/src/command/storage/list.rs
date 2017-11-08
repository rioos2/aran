pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use {PRODUCT, VERSION};
use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String) -> Result<()> {
    ui.begin("Constructing a list of storages for you...")?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;
    let results = rio_client.get_storageconnector(&token, &email)?;
    let value = results
        .get_items()
        .iter_mut()
        .map(|c| {
            rio_client
                .get_storagepool_by_id(&token, &email, &(c.get_id()))
                .map(|p| {
                    p.get_items()
                        .into_iter()
                        .map(|x| {
                            vec![
                                c.get_id(),
                                c.get_storage_type(),
                                c.get_host_ip(),
                                c.get_disks_str(),
                                x.get_id(),
                                x.get_disks_str(),
                            ]
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap()
            // println!("--- {:?}", pool);
        })
        .flat_map(|s| s)
        .collect::<Vec<_>>();


    let title = row![
        "Id",
        "Type",
        "Stored At",
        "Available Disks",
        "Pool Id",
        "Pool Usage"
    ];

    pretty_table(value.to_owned(), title);

    ui.br()?;

    ui.para(
        "For more information on storages: \
        https://www.rioos.sh/docs/reference/storages/",
    )?;

    ui.end(
        format!("{} records listed.", value.to_owned().len()),
    )?;
    Ok(())
}
