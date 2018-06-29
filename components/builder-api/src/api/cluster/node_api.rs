// Copyright 2018 The Rio Advancement Inc

use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use bytes::Bytes;
use clusters::models::ninja::Nodes;
use common::ui;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, badgateway_error, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::api::node::{Node, NodeFilter, NodeStatusUpdate};
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use serde_json;
use std::sync::Arc;
use telemetry::metrics::prometheus::PrometheusClient;

#[derive(Clone)]
pub struct NodeApi {
    prom: Box<PrometheusClient>,
    conn: Box<DataStoreConn>,
}

/// Network api: NodeApi provides ability to declare the node
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Node: URLs supported are.
/// POST: /nodes,
/// GET: /node
/// PUT: /nodes/status
/// GET: /node/:ip
impl NodeApi {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>) -> Self {
        NodeApi {
            prom: prom,
            conn: datastore,
        }
    }

    //POST: /nodes
    //The body has the input cluster::Nodes
    //Returns a mutated Network  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Node>>()?)?;

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

        match Nodes::create(&self.conn, &unmarshall_body) {
            Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
    // GET  / //GET: /nodes
    //Blank origin: Returns all the Nodes (irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match Nodes::list_blank(&self.conn) {
            Ok(Some(node_list)) => Ok(render_json_list(status::Ok, dispatch(_req), &node_list)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /nodes/:id
    //Input id - u64 as input and returns a Nodes
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match Nodes::show(&self.conn, &params) {
            Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /nodes/:id
    //Input id - u64 as input
    //Returns an nodes
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match Nodes::show(&self.conn, &idget) {
            Ok(Some(node)) => {
                let data = json!({
                            "type": typ,
                            "data": node,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //PUT: /nodes/status
    //Input status  as input and returns an Nodes
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body =
            self.validate(req.get::<bodyparser::Struct<NodeStatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match Nodes::status_update(&self.conn, &unmarshall_body) {
            Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //PUT: /nodes/:id
    //Input node  as input and returns an Nodes
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Node>>()?)?;
        unmarshall_body.set_id(params.get_id());

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match Nodes::update(&self.conn, &unmarshall_body) {
            Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //List the node fitler with node ip
    //GET: /discovery/:ip
    //Input node ip returns the  node
    fn discovery(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate(req.get::<bodyparser::Struct<NodeFilter>>()?)?;

        match Nodes::discovery(&self.conn, &unmarshall_body) {
            Ok(Some(node_get)) => Ok(render_json_list(status::Ok, dispatch(req), &node_get)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //List the node fitler with node ip
    //GET: /node/:ip
    //Input node ip returns the  node
    fn show_by_address(&self, req: &mut Request) -> AranResult<Response> {
        let query_pairs = self.default_validate(req)?;
        match Nodes::show_by_node_ip(&self.conn, &IdGet::with_id(query_pairs.get("ipaddress"))) {
            Ok(Some(node_get)) => Ok(render_json(status::Ok, &node_get)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //metrics of the overall node from prometheus
    fn healthz_all(&self, _req: &mut Request) -> AranResult<Response> {
        match Nodes::healthz_all(&self.prom) {
            Ok(Some(health_all)) => Ok(render_json(status::Ok, &health_all)),
            Err(err) => Err(badgateway_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    /// Endpoint for determining availability of builder-api components.
    /// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
    fn status(&self, _req: &mut Request) -> AranResult<Response> {
        Ok(render_json(
            status::Ok,
            &format!("code:{},version:{}", "200", "rioos-2.0"),
        ))
    }
}

impl Api for NodeApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let healthz_all =
            move |req: &mut Request| -> AranResult<Response> { _self.healthz_all(req) };

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let show_by_address =
            move |req: &mut Request| -> AranResult<Response> { _self.show_by_address(req) };

        let _self = self.clone();
        let status_update =
            move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let healthz = move |req: &mut Request| -> AranResult<Response> { _self.status(req) };

        let _self = self.clone();
        let discovery = move |req: &mut Request| -> AranResult<Response> { _self.discovery(req) };

        router.get(
            "/healthz/overall",
            XHandler::new(C { inner: healthz_all }).before(basic.clone()),
            "healthz_all",
        );

        router.get("/healthz", XHandler::new(C { inner: healthz }), "healthz");

        router.post(
            "/nodes",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "nodes",
        );
        router.get(
            "/nodes",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "nodes_list",
        );
        router.get(
            "/nodes/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "node_show",
        );
        router.put(
            "/nodes/:id/status",
            XHandler::new(C {
                inner: status_update,
            }).before(basic.clone()),
            "node_status_update",
        );

        router.put(
            "/nodes/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "node_update",
        );

        router.get(
            "/nodes/ip",
            XHandler::new(C {
                inner: show_by_address,
            }).before(basic.clone()),
            "node_show_by_address",
        );

        router.post(
            "/nodes/discover",
            XHandler::new(C { inner: discovery }).before(basic.clone()),
            "node_discovery",
        );
    }
}

impl ApiValidator for NodeApi {}

impl ParmsVerifier for NodeApi {}

impl QueryValidator for NodeApi {}

impl Validator for Node {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.get_node_ip().len() <= 0 {
            s.push("node_ip".to_string());
        }

        if self.get_status().get_phase().len() <= 0 {
            s.push("phase".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for NodeFilter {
    fn valid(self) -> AranValidResult<Self> {
        return Ok(Box::new(self));
    }
}

impl Validator for NodeStatusUpdate {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
