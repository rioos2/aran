use common::ui::UI;
use db::appstore::DataStoreConn;
use error::Result;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.title("Rio AppStore Migration for database")?;
    ui.para("Start migration of database")?;
    let ds = DataStoreConn::new()?;
    ds.setup()?;
    ui.heading("Rio/OS Migration Complete")?;
    Ok(())
}
