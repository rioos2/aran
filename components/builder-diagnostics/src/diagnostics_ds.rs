// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use iron::prelude::*;
use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use std::sync::Arc;
use protocol::api::job;
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};
use rioos_http::ApiClient;
use postgres;
use db::data_store::DataStoreConn;
use telemetry::metrics::prometheus::PrometheusClient;
use serde_json;
use serde;
use serde_derive;
use StatusOutput;

const UP: &str = "up";
const DOWN: &str = "down";

pub struct Node {
    name: String,
    ip: String,
    Status: String,
}

#[derive(Serialize)]
pub struct MasterSystemStatus {
    Apiserver: String,
    Controller: String,
    Scheduler: String,
   // Nodes: Vec<Node>,  
    //blockchain_server: String,
    //log_server: String,
    //metrics_server: String,
   // database: String,
}

pub struct ExternalSystemStatus {
    marketplace_server: String,
    vault: String,
    anchore: String,
}

pub struct DiagnosticsDS;

impl DiagnosticsDS {
    pub fn status(datastore: &DataStoreConn, prom: &PrometheusClient) -> StatusOutput {
       let mstatus = MasterSystemStatus{
        Apiserver: UP.to_string(),
        Controller: controller_status(),
        Scheduler: scheduler_status(),
        //Nodes: nodes_status(),
       };
       Some(mstatus)
    }   
}

fn controller_status() -> String {
    let client = ApiClient::new("http://localhost:10252", "", "v1", None).unwrap();
    let mut res = client.get("").send();     
    match res {
        Ok(s) => UP.to_string(),
        Err(err) => DOWN.to_string()
    }
}

fn scheduler_status() -> String {
    let client = ApiClient::new("http://localhost:10251", "", "v1", None).unwrap();
    let mut res = client.get("").send();     
    match res {
        Ok(s) => UP.to_string(),
        Err(err) => DOWN.to_string()
    }
}

/*fn nodes_status() -> String {

}*/


