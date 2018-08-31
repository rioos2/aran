//softwarekey/saver.rs

//! Saver persists the license updates performed in SoftwareKey which is in memory to a datastore. The need to persists arises from the systems around Rio/OS wanting to know about the license state. The license API directly uses the persisted license data.

use db::data_store::DataStoreConn;
use entitlement::models::license;
use protocol::api::licenses::Licenses;

pub struct Saver<'a> {
    conn: &'a Box<DataStoreConn>,
}
impl<'a> Saver<'a> {
    pub fn new(conn: &'a Box<DataStoreConn>) -> Saver {
        Saver { conn: &conn }
    }

    //Saves the commencement of trial in the datastore
    pub fn create(&self, license: Licenses) {
        license::DataStore::new(&self.conn).create_or_update(&license);
    }

    //Saves the status of the license periodically in the datastore/
    //This is called once per every hour.
    pub fn update(&self, license: Licenses) {
        license::DataStore::new(&self.conn).update(&license);
    }

    pub fn update_status(&self, license: Licenses) {
        license::DataStore::new(&self.conn).update_status(&license);
    }
}
