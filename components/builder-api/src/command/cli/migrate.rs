use std::sync::Arc;

use common::ui::UI;
use db::{data_store, marketplace_differ, system_secret};
use error::Result;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.title("Rio/OS Migration for database")?;
    ui.para("Start migration of database")?;
    let ds = data_store::DataStoreConn::new()?;
    ds.setup()?;
    let _arc_conn = Arc::new(ds);

    system_secret::SystemSecret::new(_arc_conn.clone()).setup()?;
    marketplace_differ::MarketPlaceDiffer::new(_arc_conn).setup()?;
    ui.heading("Rio/OS Migration Complete")?;
    Ok(())
}
