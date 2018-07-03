// Copyright 2018 The Rio Advancement Inc

//! The Status backend for ping

use db::data_store::DataStoreConn;
use rioos_http::ApiClient;

use clusters::models::ninja::DataStore;
use telemetry::metrics::prometheus::PrometheusClient;

use serde_json::Value;

use StatusOutput;

const UP: &str = "up";
const DOWN: &str = "down";
const PROBLEM: &str = "problem";

const UP_DESCRIPTION: &str = "Service is operating normally";
const DOWN_DESCRIPTION: &str = "Service is currently down";

const PING: &str = "ping";

#[derive(Serialize)]
pub struct Status {
    name: String,
    status: String,
    description: String,
}

#[derive(Serialize)]
pub struct Services {
    master: Vec<Status>,
    nodes: Vec<Status>,
}

pub struct Pinguy;

impl Pinguy {
    //collect service status and node status then return it.
    pub fn status(
        datastore: &DataStoreConn,
        _prom: &PrometheusClient,
        config: Value,
    ) -> StatusOutput {
        let mut mstatus = Vec::new();
        mstatus.push(Status {
            name: "API Server".to_string(),
            status: UP.to_string(),
            description: UP_DESCRIPTION.to_string(),
        });
        mstatus.push(Status {
            name: uppercase_first_letter("postgres"),
            status: UP.to_string(),
            description: UP_DESCRIPTION.to_string(),
        });
        mstatus.push(get_status(
            PING,
            "controller_endpoint".to_string(),
            config.clone(),
            "Controller".to_string(),
        ));
        mstatus.push(get_status(
            PING,
            "scheduler_endpoint".to_string(),
            config.clone(),
            "Scheduler".to_string(),
        ));
        mstatus.push(get_status(
            "blockchain",
            "endpoint".to_string(),
            config.clone(),
            "Blockchain".to_string(),
        ));
        mstatus.push(get_status(
            "logs",
            "influx_endpoint".to_string(),
            config.clone(),
            "Logs".to_string(),
        ));
        mstatus.push(get_status(
            "telemetry",
            "endpoint".to_string(),
            config.clone(),
            "Telemetry".to_string(),
        ));
        mstatus.push(get_status(
            PING,
            "machineconsole_endpoint".to_string(),
            config.clone(),
            "VNC Console".to_string(),
        ));
        mstatus.push(get_status(
            "appstores",
            "endpoint".to_string(),
            config.clone(),
            "Rio.AppStore".to_string(),
        ));
        mstatus.push(get_status(
            "vaults",
            "endpoint".to_string(),
            config.clone(),
            "Vaults".to_string(),
        ));
        mstatus.push(get_status(
            "vulnerability",
            "anchore_endpoint".to_string(),
            config.clone(),
            "Anchore".to_string(),
        ));
        let nodes = nodes_status(datastore);

        Some(Services {
            master: mstatus,
            nodes: nodes,
        })
    }
}

//first get all nodes from database
//then generate node status structure and return it
fn nodes_status(datastore: &DataStoreConn) -> Vec<Status> {
    let mut vec = Vec::new();
    match DataStore::new(datastore).list_blank() {
        Ok(Some(node_list)) => {
            for n in &node_list {
                let mut data = Status {
                    name: n.get_node_ip(),
                    status: UP.to_string(),
                    description: UP_DESCRIPTION.to_string(),
                };
                if n.get_status().get_phase().to_lowercase() != "running".to_string() {
                    data = Status {
                        name: n.get_node_ip(),
                        status: DOWN.to_string(),
                        description: DOWN_DESCRIPTION.to_string(),
                    }
                }
                vec.push(data);
            }
            vec
        }
        Err(err) => {
            debug!("Failed to ping node. {:?}", err);
            vec
        }
        Ok(None) => vec,
    }
}

// In this function got arguments name, search and config values
// get service information from config using name (like controller, scheduler) field
// then build client and request to service and build status struct from response
fn get_status(name: &str, search: String, config: Value, print: String) -> Status {
    match config[name][search.clone()].as_str() {
        Some(endpoint) => {
            if endpoint.is_empty() {
                return Status {
                    name: print,
                    status: PROBLEM.to_string(),
                    description: format!("Not configured (api.toml)"),
                };
            }
            let client = ApiClient::new(endpoint, "", "v1", None).unwrap();
            let mut res = client.get("").send();
            match res {
                Ok(_s) => Status {
                    name: print,
                    status: UP.to_string(),
                    description: UP_DESCRIPTION.to_string(),
                },
                Err(_err) => Status {
                    name: print,
                    status: DOWN.to_string(),
                    description: DOWN_DESCRIPTION.to_string(),
                },
            }
        }
        None => {
            return Status {
                name: print,
                status: PROBLEM.to_string(),
                description: format!("Not configured (api.toml)"),
            }
        }
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
