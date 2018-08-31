// use protocol::api::base::IdGet;
// use db::data_store::DataStoreConn;
// use error::{Result, Error};
// use super::{marketplace_ds, package_ds};
// use db::error::Error::RecordsNotFound;
// use protocol::api::base::MetaFields;
//
// pub struct PackageAttacher<'a> {
//     mid: &'a IdGet,
//     conn: &'a DataStoreConn,
// }
//
// #[derive(Default)]
// pub struct PackageParam {
//     pub category: String,
//     pub origin: String,
//     pub extension: String,
//     pub name: String,
//     pub version: String,
// }
//
// impl PackageParam {
//     pub fn get_url(&self) -> String {
//         self.origin.clone() + "/" + &self.category + "/" + &self.version + "_" + &self.name + "." + &self.extension
//     }
// }
//
// impl<'a> PackageAttacher<'a> {
//     pub fn new(conn: &'a DataStoreConn, mid: &'a IdGet) -> Self {
//         PackageAttacher {
//             conn: &*conn,
//             mid: &*mid,
//         }
//     }
//
//     pub fn with(&self, category: String, origin: String, extension: String, version: String, name: String) -> PackageParam {
//         PackageParam {
//             category: category,
//             origin: origin,
//             extension: extension,
//             version: version,
//             name: name,
//             ..Default::default()
//         }
//     }
//
//     ///get the marketplace and package information to form the directory
//     pub fn get_package(&self) -> Result<Option<PackageParam>> {
//         match marketplace_ds::DataStore::new(&self.conn).show(&self.mid) {
//             Ok(Some(marketplace)) => {
//                 let params: Vec<IdGet> = marketplace
//                     .get_owner_references()
//                     .iter()
//                     .map(|x| IdGet::with_id(x.uid.to_string()))
//                     .collect::<Vec<_>>();
//
//                 match package_ds::DataStore::new(&self.conn).show(&params[0]) {
//                     Ok(Some(package)) => Ok(Some(
//                         self.with(
//                             marketplace.get_category(),
//                             marketplace
//                                 .get_metadata()
//                                 .get("origin")
//                                 .unwrap_or(&"rioos-system".to_string())
//                                 .to_string(),
//                             package.get_extension(),
//                             package.get_version_number(),
//                             marketplace.get_name(),
//                         ),
//                     )),
//                     Err(err) => Err(err),
//                     Ok(None) => Err(Error::Db(RecordsNotFound)),
//                 }
//             }
//             Err(err) => Err(err),
//             Ok(None) => Err(Error::Db(RecordsNotFound)),
//         }
//     }
// }
