use db::{system_secret, data_store, marketplace_differ};
use common::ui::UI;
use error::Result;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.title("Rio/OS DB Migration")?;
    ui.para("DB is Migrating")?;
    let ds = data_store::DataStoreConn::new()?;
    ds.setup()?;
    system_secret::SystemSecret::new(ds.clone()).setup()?;
    marketplace_differ::MarketPlaceDiffer::new(ds.clone())
        .setup()?;
    ui.heading("Rio/OS Migration Complete")?;
    Ok(())
}
