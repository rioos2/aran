use common::ui::UI;
use db::data_store;
use error::Result;
use hooks::before::{AHooks, HookServiceFn, DIFFER_HOOK, NINJA_HOOK, SECRET_HOOK, SENSEI_HOOK};
use hooks::differ::AppStore;
use hooks::register::Sensei;
use hooks::secrets::ForGulpd;
use hooks::settings::Ninja;
use hooks::BeforeHook;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.title("Rio/OS Migration for database")?;
    ui.para("Start migration of database")?;
    let ds = data_store::DataStoreConn::new()?;
    ds.setup()?;

    let box_ds = Box::new(ds.clone());

    ui.para("Before Hooks")?;
    let mut ah = AHooks::new();

    let box_ds1 = box_ds.clone();
    let fndiffer = Box::new(HookServiceFn::new(
        DIFFER_HOOK.to_string(),
        Box::new(move || -> Option<()> { AppStore::new(box_ds1.clone()).before().ok() }),
    ));

    let box_ds2 = box_ds.clone();
    let fnsecret = Box::new(HookServiceFn::new(
        SECRET_HOOK.to_string(),
        Box::new(move || -> Option<()> { ForGulpd::new(box_ds2.clone()).before().ok() }),
    ));

    let box_ds3 = box_ds.clone();
    let fnninja = Box::new(HookServiceFn::new(
        NINJA_HOOK.to_string(),
        Box::new(move || -> Option<()> { Ninja::new(box_ds3.clone()).before().ok() }),
    ));

    let box_ds4 = box_ds.clone();
    let fnsensei = Box::new(HookServiceFn::new(
        SENSEI_HOOK.to_string(),
        Box::new(move || -> Option<()> { Sensei::new(box_ds4.clone()).before().ok() }),
    ));

    ah.register(fndiffer);
    ah.register(fnsecret);
    ah.register(fnninja);
    ah.register(fnsensei);

    ah.setup();

    ui.heading("Rio/OS Migration Complete")?;
    Ok(())
}
