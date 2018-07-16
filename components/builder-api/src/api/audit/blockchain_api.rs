use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;
use typemap;

use api::audit::config::BlockchainConn;
use api::events::EventLogger;
use api::{Api, ApiValidator, ParmsVerifier, Validator};
use common::ui;

use config::Config;
use error::Error;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, badgateway_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

use super::ledger;
use protocol::api::audit::AuditEvent;
use protocol::api::base::MetaFields;
use protocol::api::schema::{dispatch, type_meta};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;

define_event_log!();

#[derive(Clone)]
pub struct BlockChainApi {
    clientcfg: Box<BlockchainConn>,
    conn: Box<DataStoreConn>,
}

/// BlockChainApi: BlockChainApi provides ability to post the audits of the users
/// and manage them.
//
/// URL:
/// POST:/account/:account_id/audits,
/// GET: /account/:account_id/audits
impl BlockChainApi {
    pub fn new(datastore: Box<DataStoreConn>, clientcfg: Box<BlockchainConn>) -> Self {
        BlockChainApi {
            clientcfg: clientcfg,
            conn: datastore,
        }
    }

    //POST: /accounts/:account_id/audits
    //The body has the input audit::AuditEvent
    //Upon receipt of the AuditEvent with an account_id, the event
    //is sent to an asynchronous channel for processing.
    //Hence this POST operation must always return success.
    //Returns a response with the same input AuditEvent received.
    //- id
    //- ObjectMeta
    //- created_at is not available for this, as the AuditEvent is converted to
    //- an envelope which has the timestamp.
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<AuditEvent>(req.get::<bodyparser::Struct<AuditEvent>>()?)?;

        debug!("{} ✓",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            self.verify_account(req)?.get_name(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        //Send to the eventlogger and return.
        log_event!(req, *unmarshall_body.clone());
        push_notification!(req, *unmarshall_body.clone());

        Ok(render_json(status::Ok, &unmarshall_body))
    }

    //GET: /accounts/:account_id/audits
    //Input account_id
    // Returns all the audits (for that account)
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let data = ledger::from_config(&self.clientcfg)?;

        match data.retrieve_by(&params) {
            Ok(Some(envelopes)) => Ok(render_json_list(status::Ok, dispatch(req), &envelopes)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
            Err(err) => Err(badgateway_error(&format!("{}", err))),
        }
    }
}

impl Api for BlockChainApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        //secret API
        router.post(
            "/accounts/:account_id/audits",
            XHandler::new(C { inner: create }),
            "audits",
        );

        router.get(
            "/accounts/:id/audits",
            XHandler::new(C { inner: list }),
            "list_audits",
        );
    }
}

impl ApiValidator for BlockChainApi {}

impl ParmsVerifier for BlockChainApi {}

impl Validator for AuditEvent {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
