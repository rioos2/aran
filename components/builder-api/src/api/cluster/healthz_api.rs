// Copyright 2018 The Rio Advancement Inc

use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use bytes::Bytes;
use clusters::models::healthz::DataStore;
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
pub struct HealthzApi {
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
impl HealthzApi {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>) -> Self {
        HealthzApi {
            prom: prom,
            conn: datastore,
        }
    }    

    //metrics of the overall node from prometheus
    fn healthz_all(&self, _req: &mut Request) -> AranResult<Response> {

        match DataStore::new(&self.conn).healthz_all(&self.prom) {
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

impl Api for HealthzApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let healthz_all =
            move |req: &mut Request| -> AranResult<Response> { _self.healthz_all(req) };

        let _self = self.clone();
        let healthz = move |req: &mut Request| -> AranResult<Response> { _self.status(req) };

        router.get(
            "/healthz/overall",
            XHandler::new(C { inner: healthz_all }).before(basic.clone()),
            "healthz_all",
        );

        router.get("/healthz", XHandler::new(C { inner: healthz }), "healthz");
       
    }
}

impl ApiValidator for HealthzApi {}

impl ParmsVerifier for HealthzApi {}

impl QueryValidator for HealthzApi {}

