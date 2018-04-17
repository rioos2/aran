use db::data_store::DataStoreConn;
use common::ui::UI;
use error::Result;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.title("Rio/OS Migration for database")?;
    ui.para("Start migration of database")?;
    let ds = DataStoreConn::new()?;
    ds.setup()?;
    ui.heading("Rio/OS Migration Complete")?;
    Ok(())
}
