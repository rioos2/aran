// Copyright 2018 The Rio Advancement Inc

//! The startup hook is responsible for setting the ninja config (nodelet.rioconfig) in settingsmap.
// in origin: rioos_system, name: ninja
// This can be pulled like /origin/rioos_system/settings_map/ninja
use base64;
use db::data_store::DataStoreConn;
use service::models::settings_map;
use error::{Result, Error};
use hooks::BeforeHook;
use rio_core::crypto::{default_rioconfig_key_path, keys::read_key_in_bytes};
use std::path::PathBuf;
use std::collections::BTreeMap;
use protocol::api::settings_map::SettingsMap;
use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};
use protocol::api::schema::type_meta_url;

const NAME_NINJA_RIOCONFIG: &'static str = "ninja";

lazy_static! {
    static ref NODELET_CONFIG_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None).join("nodelet.rioconfig").to_str().unwrap(),);
}

pub struct Ninja {
    conn: Box<DataStoreConn>,
}

impl Ninja {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        Ninja { conn: datastore  }
    }

    fn setup_ninja_config(&self) -> Result<()> {
        let mut id = IdGet::with_id(NAME_NINJA_RIOCONFIG.to_string());
        id.set_name("rioos_system".to_string());
        
        let settings = settings_map::DataStore::new(&self.conn);

        match settings.show(&id)  {
            Ok(old_ninja) => {
             //Only if the old_ninja value doesn't exists then insert a new copy.   
             if(old_ninja.is_none()) {
               let mut s = SettingsMap::new();
               let ref mut om = s.mut_meta(s.object_meta(), NAME_NINJA_RIOCONFIG.to_string(), "rioos_system".to_string());

               // ObjectMeta and TypeMeta 
               let jackie = s.who_am_i();
               s.set_meta(type_meta_url(jackie), om.clone());
                
               // Data 
                let mut data = BTreeMap::new();
                data.insert("rioos_sh_ninja_rioconfig".to_string(), format!("{}",base64::encode(&read_key_in_bytes(&NODELET_CONFIG_FILE.as_path())?)));
                s.set_data(data);

                // Metadata 
                let mut metadata = BTreeMap::new();
                metadata.insert("origin".to_string(), "rioos_system".to_string());              
                s.set_metadata(metadata);

                settings.create(&s)?;
               }
                Ok(())
            },
            Err(e) => Err(Error::Secret(e)),
        }         
    }
}

impl BeforeHook for Ninja {
    fn before(&self) -> Result<()> {
        self.setup_ninja_config()?;
        Ok(())
    }
}
