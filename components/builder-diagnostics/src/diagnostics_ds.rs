// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use rioos_http::ApiClient;
use db::data_store::DataStoreConn;
use telemetry::metrics::prometheus::PrometheusClient;
use nodesrv::node_ds::NodeDS;
use serde_json::Value;
use StatusOutput;

const UP: &str = "up";
const DOWN: &str = "down";

#[derive(Serialize)]
pub struct Node {
    //Name: String,
    ip: String,
    status: String,
}

#[derive(Serialize)]
pub struct MasterSystems {
    apiserver: String,
    controller: String,
    scheduler: String,
    nodes: Vec<Node>,  
    postgres: String,
}

#[derive(Serialize)]
pub struct SupportSystems {   
    blockchainserver: String,
    logserver: String,
    metricsserver: String,
    vncserver: String,
}

#[derive(Serialize)]
pub struct ExternalSystems {
    marketplaceserver: String,
    vault: String,
    anchore: String,
}

#[derive(Serialize)]
pub struct Status {
	master: MasterSystems,
	support: SupportSystems,
	external: ExternalSystems,
}

pub struct DiagnosticsDS;

impl DiagnosticsDS {
    pub fn status(datastore: &DataStoreConn, _prom: &PrometheusClient, config: Value) -> StatusOutput {
       let mstatus = Status {
       		master: MasterSystems {
        			apiserver: UP.to_string(),
        			controller: get_status("controller".to_string(), "url".to_string(), config.clone()),
        			scheduler: get_status("scheduler".to_string(), "url".to_string(), config.clone()),        			
        			nodes: nodes_status(datastore),
        			postgres: UP.to_string(),
       		},
       		support: SupportSystems {        			
        			blockchainserver: get_status("blockchain".to_string(), "endpoint".to_string(), config.clone()),
        			logserver: get_status("logs".to_string(), "url".to_string(), config.clone()),
        			metricsserver: get_status("prometheus".to_string(), "url".to_string(), config.clone()),
        			vncserver: get_status("vnc".to_string(), "url".to_string(), config.clone()),
       		},
       		external: ExternalSystems {
       				marketplaceserver: get_status("marketplaces".to_string(), "endpoint".to_string(), config.clone()),
       				vault: get_status("vaults".to_string(), "endpoint".to_string(), config.clone()),
       				anchore: get_status("anchore".to_string(), "url".to_string(), config.clone()),
       		},
       	};
       Some(mstatus)
    }   
}

fn nodes_status(datastore: &DataStoreConn) -> Vec<Node> {
	let mut vec = Vec::new();
	match NodeDS::list_blank(datastore) {
        Ok(Some(node_list)) => {
        	for n in &node_list {
    			let data = Node {
                    ip: n.get_node_ip(),
                    status: n.get_status().get_phase(),
                };
                vec.push(data);
			}
			vec
        }
        Err(err) => {
        	println!("{:?}", err);
        	vec
        }
        Ok(None) => vec,
    }
}

fn get_status(name: String, search: String, config: Value) -> String {
   match config[name.clone()][search].as_str() {
		Some(url) => {
			let client = ApiClient::new(url, "", "v1", None).unwrap();
    		let mut res = client.get("").send();     
    		match res {
        		Ok(_s) => UP.to_string(),
        		Err(_err) => DOWN.to_string()
    		}
		}
		None => return format!("{} doesn't configure api.toml file.", name.clone())
	}	
}



