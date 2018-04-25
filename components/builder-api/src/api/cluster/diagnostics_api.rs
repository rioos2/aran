// Copyright 2018 The Rio Advancement Inc

use std::sync::Arc;
use std::str;
use iron::prelude::*;
use router::Router;
use api::Api;
use iron::status;
use config::Config;
use serde_json;
use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use db::data_store::DataStoreConn;
use telemetry::metrics::prometheus::PrometheusClient;
use diagnostics::diagnostics_ds::DiagnosticsDS;
use rio_net::util::errors::internal_error;
use rio_net::http::rendering::render_html;

#[derive(Clone)]
pub struct DiagnosticsApi {
    prom: Box<PrometheusClient>,
    conn: Box<DataStoreConn>,
    config: Arc<Config>,
}

/// Diagnostics api: DiagnosticsApi provides all system informations
/// 1. Version of Rio/OS  software
/// 2. Are all the software running
/// 3. Read the *.toml files and see the interconnecting softwares are up.
/// 4. Read the docker logs, and native logs (queryParms [tail=1000]) 
//
/// Diagnostics: URLs supported are.
/// GET: /ping,
/// GET: /diagnostics

impl DiagnosticsApi {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>, config: Arc<Config>) -> Self {
        DiagnosticsApi {
            prom: prom,
            conn: datastore,
            config: config,
        }
    }

    fn ping(&self, req: &mut Request) -> AranResult<Response> {       
        let mut flag = false;
        let conf = serde_json::to_value(&*self.config).unwrap();
        match req.headers.get_raw("Accept") {
            Some(res) => {
                let s = match str::from_utf8(&res[0]) {
                            Ok(v) => v,
                            Err(_e) => "",
                };              
                let splitted: Vec<&str> = s.split(",").collect();
                match splitted.iter().find(|&&x| x == "text/html") {
                    Some(_res) => {
                        flag = true;
                    }
                    None => {}
                }
            }
            None => {}
        }       
        match DiagnosticsDS::status(&self.conn, &self.prom, conf) {
            Some(node) => {                
                if flag {
                    Ok(render_html(status::Ok, &node, "Status".to_string()))
                } else {
                    Ok(render_json(status::Ok, &node))
                }
            }            
            None => Err(internal_error(&format!("error"))),
        }
    }

}

impl Api for DiagnosticsApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {

        let _self = self.clone();
        let ping = move |req: &mut Request| -> AranResult<Response> { _self.ping(req) };
    
        router.get("/ping", XHandler::new(C { inner: ping }), "ping");

      
    }
}
