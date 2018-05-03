use std::sync::Arc;
use telemetry::metrics::prometheus::PrometheusClient;
use iron::prelude::*;
use router::Router;


use config::Config;
use error::Error;

use http_gateway::http::controller::*;
use rioos_http::ApiClient;

use http_gateway::util::errors::{AranResult};
use http_gateway::util::errors::{not_found_error, internal_error};

use protocol::api::base::IdGet;
use deploy::models::{assembly};
use iron::status;
use protocol::api::deploy::VncResponseUrl;
use api::Api;
use db::error::Error::RecordsNotFound;
use db::data_store::DataStoreConn;

#[derive(Clone)]
pub struct VncConsoleApi {
    prom: Box<PrometheusClient>,
    conn: Box<DataStoreConn>,
    config: Arc<Config>,
}

/// BlockChainApi: BlockChainApi provides ability to post the audits of the users
/// and manage them.
//
/// URL:
/// POST:/account/:account_id/audits,
/// GET: /account/:account_id/audits
impl VncConsoleApi {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>, config: Arc<Config>) -> Self {
        VncConsoleApi { prom: prom,
                        conn: datastore,
                        config: config }
    }


    fn get_url(&self, req: &mut Request) -> AranResult<Response> {
        let (acc, asm_id) = {
            let params = req.extensions.get::<Router>().unwrap();
            let acc = params.find("account_id").unwrap().to_owned();
            let asm_id = params.find("id").unwrap().to_owned();
            (acc, asm_id)
        };
        let id_get = IdGet::with_id(asm_id.to_string());
        match assembly::DataStore::new(&self.conn).show(&id_get) {
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound),asm_id))),
            Ok(Some(assembly)) => {
                if assembly.get_metadata().contains_key("rioos_sh_vnc_host") && assembly.get_metadata().contains_key("rioos_sh_vnc_port") {
                    let host = assembly.get_metadata().get("rioos_sh_vnc_host").unwrap();
                    let port = assembly.get_metadata().get("rioos_sh_vnc_port").unwrap();
                    let url = format!("http://{}:{}/exec/accounts/{}/assemblys/{}?tty=1&input=1", host, port,acc,asm_id);

                    let client = ApiClient::new(&url, "", "v1", None).unwrap();
                    let res = client.get("").send();
                    match res {
                        Ok(mut data) => {
                            let x: VncResponseUrl = data.json()?;
                            Ok(render_json(status::Ok, &VncResponseUrl {
                            url: format!("{}:{}/{:?}",host,port,x)
                            }))
                            }
                        Err(err) => Err(internal_error(&format!("{}", err))),
                    }
                    } else {
                        return Err(not_found_error(&format!("No host or Port fount for {} ",asm_id)))
                    }
        }
    }
}
}

impl Api for VncConsoleApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {

        let _self = self.clone();
        let get_url = move |req: &mut Request| -> AranResult<Response> { _self.get_url(req) };

        router.get("/accounts/:account_id/assemblys/:id/exec", XHandler::new(C { inner: get_url }), "get_console_url");

    }
}
