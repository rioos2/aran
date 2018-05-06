// Copyright 2018 The Rio Advancement Inc

use std::sync::Arc;
use std::str;
use std::path::PathBuf;

use chrono::Local;

use iron::prelude::*;
use iron::headers::ContentType;
use iron::mime::{Mime, SubLevel, TopLevel};
use iron::modifiers::Header;
use iron::status;

use api::Api;
use super::super::super::VERSION;

use config::Config;

use router::Router;
use serde::Serialize;
use serde_json;
use serde_json::value::Map;

use http_gateway::http::controller::*;
use http_gateway::util::errors::AranResult;
use http_gateway::util::errors::internal_error;

use db::data_store::DataStoreConn;
use telemetry::metrics::prometheus::PrometheusClient;
use rio_diago::models::diagnostics::Pinguy;
use rio_core::fs::{read_from_file, rioconfig_config_path};

use handlebars::{to_json, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

lazy_static! {
    static  ref RIOSTATUS_TEMPLATE: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("template/riostatus.hbs").to_str().unwrap());
}

#[derive(Clone)]
pub struct DiagnosticsApi {
    prom: Box<PrometheusClient>,
    conn: Arc<DataStoreConn>,
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
    pub fn new(datastore: Arc<DataStoreConn>, prom: Box<PrometheusClient>, config: Arc<Config>) -> Self {
        DiagnosticsApi {
            prom: prom,
            conn: datastore,
            config: config,
        }
    }

    fn ping(&self, req: &mut Request) -> AranResult<Response> {
        let conf = serde_json::to_value(&*self.config).unwrap();
        debug!("finding raw flag");
        let raw_flag = req.headers
            .get_raw("Accept")
            .map(|res| {
                str::from_utf8(&res[0])
                    .unwrap_or("")
                    .split(",")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .find(|&x| x == "text/html")
            })
            .unwrap_or(None);
        debug!("-- raw flag is {:?}", raw_flag);

        match Pinguy::status(&self.conn, &self.prom, conf) {
            Some(data) => {
                if raw_flag.is_some() {
                    return Ok(Self::render_ping_html(
                        status::Ok,
                        &data,
                        "Status".to_string(),
                    ));
                }

                Ok(render_json(status::Ok, &data))
            }
            None => Err(internal_error(&format!("Pong - no balls."))),
        }
    }

    fn render_ping_html<T: Serialize>(_status: status::Status, response: &T, _title: String) -> Response {
        let value = serde_json::to_value(response).unwrap();
        let mut data = Map::new();
        let headers = Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("status_symbol", Box::new(Self::status_symbol_helper));

        let date = Local::now();
        data.insert("time".to_string(), to_json(&date.to_rfc2822()));

        match read_from_file(&RIOSTATUS_TEMPLATE) {
            Ok(res) => {
                data.insert(
                    "version".to_string(),
                    serde_json::Value::String(VERSION.to_string()),
                );
                data.insert("packages".to_string(), to_json(&value["master"]));
                data.insert("nodes".to_string(), to_json(&value["nodes"]));
                let r = handlebars.render_template(&res, &data).unwrap();

                Response::with((status::Ok, r, headers))
            }
            Err(_err) => {
                let r = "<h3>Pong - no balls returned. Missing config/template/riostatus.hbs. Have you configured correctly ?</h3>";
                Response::with((status::Ok, r, headers))
            }
        }
    }

    // define a custom helper
    fn status_symbol_helper(h: &Helper, _: &Handlebars, _: &mut RenderContext, out: &mut Output) -> Result<(), RenderError> {
        // get parameter from helper or throw an error
        let param = try!(h.param(0,).ok_or(RenderError::new(
            "status symbol helper is required for displaying the ping status symbol.",
        ),));

        let mut rendered = format!(
            "<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\" width=\"90.594px\" height=\"59.714px\" viewBox=\"0 0 90.594 59.714\" enable-background=\"new 0 0 90.594 59.714\" xml:space=\"preserve\" class=\"icon\">
        <polyline class=\"up\" fill=\"none\" stroke=\"#666\" stroke-width=\"5\" stroke-miterlimit=\"10\" points=\"1.768,23.532 34.415,56.179 88.826,1.768\"/>
         </svg>"
        );

        if param.value().render() != "up".to_string() {
            rendered = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 414.298 414.299\" class=\"icon\"><path d=\"M3.663 410.637c2.44 2.44 5.64 3.66 8.84 3.66 3.198 0 6.397-1.22 8.838-3.66l185.81-185.81 185.81 185.81c2.44 2.44 5.64 3.662 8.84 3.662 3.198 0 6.397-1.222 8.84-3.662 4.88-4.88 4.88-12.796 0-17.68L224.827 207.15l185.81-185.81c4.882-4.88 4.882-12.795 0-17.677-4.88-4.88-12.795-4.88-17.678 0L207.15 189.47 21.34 3.664c-4.882-4.882-12.796-4.882-17.678 0-4.882 4.88-4.882 12.796 0 17.678l185.81 185.81L3.662 392.96c-4.88 4.88-4.88 12.796 0 17.677z\"/></svg>");
        }

        out.write(rendered.as_ref())?;
        Ok(())
    }
}

impl Api for DiagnosticsApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {
        let _self = self.clone();
        let ping = move |req: &mut Request| -> AranResult<Response> { _self.ping(req) };

        router.get("/ping", XHandler::new(C { inner: ping }), "ping");
    }
}
