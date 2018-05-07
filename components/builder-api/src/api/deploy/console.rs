use std::sync::Arc;
use iron::prelude::*;
use router::Router;

use config::Config;
use error::Error;

use http_gateway::http::controller::*;
use rioos_http::ApiClient;

use http_gateway::util::errors::AranResult;
use http_gateway::util::errors::{internal_error, not_found_error};

use protocol::api::base::IdGet;
use deploy::models::assembly;
use iron::status;
use protocol::api::deploy::ExecURL;
use api::Api;
use db::error::Error::RecordsNotFound;
use db::data_store::DataStoreConn;

#[derive(Clone)]
pub struct Containers {
    conn: Arc<DataStoreConn>,
    config: Arc<Config>,
}

/// URL:
/// GET: /account/:account_id/assemblys/:id/exec
impl Containers {
    pub fn new(datastore: Arc<DataStoreConn>, config: Arc<Config>) -> Self {
        Containers {
            conn: datastore,
            config: config,
        }
    }

    fn get(&self, req: &mut Request) -> AranResult<Response> {
        let (acc, asm_id) = {
            let params = req.extensions.get::<Router>().unwrap();
            let acc = params.find("account_id").unwrap().to_owned();
            let asm_id = params.find("id").unwrap().to_owned();
            (acc, asm_id)
        };
        let id_get = IdGet::with_id(asm_id.to_string());
        match assembly::DataStore::new(&self.conn).show(&id_get) {
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                asm_id
            ))),
            Ok(Some(assembly)) => {
                if !assembly.get_metadata().contains_key("rioos_sh_vnc_host") || !assembly.get_metadata().contains_key("rioos_sh_vnc_port") {
                    return Err(not_found_error(&format!(
                        "Still deploying. Must have console host and port: for {} ",
                        asm_id
                    )));
                }
                let vnc = &"".to_string();
                let host = assembly
                    .get_metadata()
                    .get("rioos_sh_vnc_host")
                    .unwrap_or(vnc);
                let port = assembly
                    .get_metadata()
                    .get("rioos_sh_vnc_port")
                    .unwrap_or(vnc);
                let url = format!(
                    "http://{}:{}/exec/accounts/{}/assemblys/{}?tty=1&input=1&stdout=1&stdin=1&stderr=1",
                    host,
                    port,
                    acc,
                    asm_id
                );

                let client = ApiClient::new(&url, "", "v1", None)?;
                let res = client.get("").send();

                match res {
                    Ok(mut data) => {
                        let x: ExecURL = data.json()?;
                        Ok(render_json(
                            status::Ok,
                            &ExecURL {
                                url: format!("{}", x.url),
                                target: format!("{}:{}", host, port),
                            },
                        ))
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        Err(internal_error(&format!("{}", err)))
                    }
                }
            }
        }
    }
}

impl Api for Containers {

    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);


        let _self = self.clone();
        let get_url = move |req: &mut Request| -> AranResult<Response> { _self.get(req) };

        router.get(
            "/accounts/:account_id/assemblys/:id/exec",
            XHandler::new(C { inner: get_url })
            .before(basic.clone()),
            "get_console_url",
        );
    }
}
