

use api::{Api, ParmsVerifier, QueryValidator};

use audit::config::InfluxClientConn;
use audit::models::log;

use config::Config;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use deploy::models::assembly;
use error::Error;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{internal_error, not_found_error, badgateway_error};
use http_gateway::util::errors::AranResult;

use iron::prelude::*;
use iron::status;

use protocol::api::base::MetaFields;
use protocol::api::log::LogQueryBuilder;
use protocol::api::schema::dispatch;
use router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct LogApi {
    logconn: Box<InfluxClientConn>,
    conn: Box<DataStoreConn>,
}

/// LogApi: LogApi provides ability to get the log for machine and container
/// and manage them.
//
/// LOG: URLs supported are.
/// GET: /list,
/// GET: /list_blank
impl LogApi {
    pub fn new(datastore: Box<DataStoreConn>, logconn: Box<InfluxClientConn>) -> Self {
        LogApi {
            logconn: logconn,
            conn: datastore,
        }
    }

    /// list the log for all (container and machine)
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        let query_pairs = self.optional_validate(req)?;
        match log::DataStore::list_blank(&self.logconn, &LogQueryBuilder::with(query_pairs)) {
            Ok(Some(log_list)) => Ok(render_json_list(status::Ok, dispatch(req), &log_list)),
            Err(err) => {
                if format!("{:?}", err).contains("Connection refused") {
                    return Err(badgateway_error(&format!("{}", err)));
                }
                Err(internal_error(&format!("{}\n", err)))
            }
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    /// list the log Based on the assembly_id
    ///input: assembly_id
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        let mut query_pairs = self.optional_validate(req)?;
        match assembly::DataStore::new(&self.conn).show(&params) {
            Ok(Some(assembly)) => {
                query_pairs.labels.insert(
                    "name".to_string(),
                    assembly.object_meta().name,
                );
                match log::DataStore::list(&self.logconn, &LogQueryBuilder::with(query_pairs)) {
                    Ok(Some(log_list)) => Ok(render_json_list(status::Ok, dispatch(req), &log_list)),
                    Err(err) => {
                        if format!("{:?}", err).contains("Connection refused") {
                            return Err(badgateway_error(&format!("{}", err)));
                        }
                        Err(internal_error(&format!("{}\n", err)))
                    }
                    Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
                }
            }
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
}

impl Api for LogApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {
        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        router.get(
            "/logs",
            XHandler::new(C { inner: list_blank }),
            "logs_list_blank",
        );

        router.get("/logs/:id", XHandler::new(C { inner: list }), "list");
    }
}

impl ParmsVerifier for LogApi {}

impl QueryValidator for LogApi {}
