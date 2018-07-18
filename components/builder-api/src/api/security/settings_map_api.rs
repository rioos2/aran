// Copyright 2018 The Rio Advancement Inc
//

use api::{Api, ApiValidator, ParmsVerifier, Validator};
use bodyparser;
use bytes::Bytes;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::{IdGet, MetaFields};
use protocol::api::schema::type_meta;
use protocol::api::settings_map::SettingsMap;
use router::Router;
use serde_json;
use service::models::settings_map;
use std::sync::Arc;

/// Securer api: SecurerApi provides ability to declare the node
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Secret: URLs supported are.
/// POST: /accounts/:account_id/secrets,,
/// GET: /accounts/:account_id/secrets,
/// GET: /secrets,
#[derive(Clone)]
pub struct SettingsMapApi {
    conn: Box<DataStoreConn>,
}

impl SettingsMapApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        SettingsMapApi { conn: datastore }
    }
    //POST: /settingsmap
    //The body has the input cluster::secrets
    //Returns a mutated Secret  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<SettingsMap>(req.get::<bodyparser::Struct<SettingsMap>>()?)?;

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        match settings_map::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(settings)) => Ok(render_json(status::Ok, &settings)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /origin/:origin_name/settingsmap/:settingsmap_name
    //Input id - u64 as input and returns a secrets
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let (org, name) = {
            let params = req.extensions.get::<Router>().unwrap();
            let org_name = params.find("origin").unwrap().to_owned();
            let set_name = params.find("name").unwrap().to_owned();
            (org_name, set_name)
        };

        debug!("✓ {}",
            format!("======= parsed {:?}{} ", org, name),
        );
        let mut params = IdGet::with_id(name.clone().to_string());
        params.set_name(org.clone().to_string());

        match settings_map::DataStore::new(&self.conn).show(&params) {
            Ok(Some(settings)) => Ok(render_json(status::Ok, &settings)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                name
            ))),
        }
    }

    //GET: /settingsmap/:id
    //Input id - u64 as input
    //Returns an secrets
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match settings_map::DataStore::new(&self.conn).show_by_id(&idget) {
            Ok(Some(settings)) => {
                let data = json!({
                            "type": typ,
                            "data": settings,      
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for SettingsMapApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);
        //closures : secrets
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        //settingsmap API
        router.post(
            "/settingsmap",
            XHandler::new(C { inner: create }),
            "settingsmap",
        );
        router.get(
            "/origins/:origin/settingsmap/:name",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "settingsmap_show",
        );
    }
}

impl ApiValidator for SettingsMapApi {}

impl ParmsVerifier for SettingsMapApi {}

impl Validator for SettingsMap {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];
        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if !self.get_metadata().contains_key("origin") {
            s.push("origin".to_string());
        }
        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
