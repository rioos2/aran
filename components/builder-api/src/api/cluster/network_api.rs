// Copyright 2018 The Rio Advancement Inc

//! A collection of network functions for the HTTP server

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;
use ansi_term::Colour;
use common::ui;

use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::{dispatch, type_meta};

use config::Config;
use error::Error;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};
use error::ErrorMessage::MissingParameter;

use network::network_ds::NetworkDS;
use protocol::api::network::Network;

use db::error::Error::RecordsNotFound;
use db::data_store::DataStoreConn;
use protocol::api::base::MetaFields;
use bytes::Bytes;
use serde_json;
use protocol::api::base::IdGet;

#[derive(Clone)]
pub struct NetworkApi {
    conn: Box<DataStoreConn>,
}

/// Network api: NetworkApi provides ability to declare the network
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Network: URLs supported are.
/// POST: /network,
/// GET: /networks
impl NetworkApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        NetworkApi { conn: datastore }
    }

    //POST: /networks
    //The body has the input cluster::Networks
    //Returns a mutated Network  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Network>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match NetworkDS::create(&self.conn, &unmarshall_body) {
            Ok(Some(network)) => Ok(render_json(status::Ok, &network)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    // GET  / //GET: /networks
    //Blank origin: Returns all the Networks (irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match NetworkDS::list_blank(&self.conn) {
            Ok(Some(network)) => Ok(render_json_list(status::Ok, dispatch(_req), &network)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
    // PUT  / //PUT: /networks/:id
    //Update network data
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Network>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match NetworkDS::update(&self.conn, &unmarshall_body) {
            Ok(Some(update)) => Ok(render_json(status::Ok, &update)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
    //GET: /networks/:id
    //Input id - u64 as input and returns a Nodes
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match NetworkDS::show(&self.conn, &params) {
            Ok(Some(network)) => Ok(render_json(status::Ok, &network)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /networks/:id
    //Input id - u64 as input
    //Returns an networks
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match NetworkDS::show(&self.conn, &idget) {
            Ok(Some(network)) => {
                let data = json!({
                            "type": typ,
                            "data": network,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for NetworkApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        //origin less,
        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        router.post(
            "/networks",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "networks",
        );

        router.get(
            "/networks",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "networks_list_blank",
        );

        router.get(
            "/networks/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "networks_get",
        );

        router.put(
            "/networks/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "networks_update",
        );
    }
}

impl ApiValidator for NetworkApi {}

impl ParmsVerifier for NetworkApi {}

//Validates parsed Networks from the body of the request.
//Checks for `...` in .....
impl Validator for Network {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_network_type().len() <= 0 {
            s.push("network_type".to_string());
        }

        if self.get_subnet_ip().len() <= 0 {
            s.push("subnet_ip".to_string());
        }

        if self.get_netmask().len() <= 0 {
            s.push("netmask".to_string());
        }

        if self.get_gateway().len() <= 0 {
            s.push("gateway".to_string());
        }

        if self.get_bridge_hosts().len() <= 0 {
            s.push("bridge_hosts".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
