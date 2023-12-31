use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::type_meta;
use config::Config;
use error::Error;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

use protocol::api::settings_map::SettingsMap;
use protocol::api::base::{IdGet, MetaFields};
use service::settings_map_ds::SettingsMapDS;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;
use bytes::Bytes;
use serde_json;

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
        let mut unmarshall_body = self.validate::<SettingsMap>(req.get::<bodyparser::Struct<SettingsMap>>()?)?;

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        match SettingsMapDS::create(&self.conn, &unmarshall_body) {
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

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?}{} ", org, name),
        );
        let mut params = IdGet::with_id(name.clone().to_string());
        params.set_name(org.clone().to_string());

        match SettingsMapDS::show(&self.conn, &params) {
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
        let res = match SettingsMapDS::show_by_id(&self.conn, &idget) {
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
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {
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
            XHandler::new(C { inner: show }),
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
