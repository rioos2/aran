use super::ledger;
use api::{Api, ApiValidator, ParmsVerifier, Validator};
use api::blockchain::audits_api::EventLog;

use api::blockchain::config::BlockchainConn;
use api::events::EventLogger;

use bodyparser;

use config::Config;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, badgateway_error, not_found_error, internal_error};
use iron::prelude::*;
use iron::status;
use protocol::api::audit::AuditEvent;
use protocol::api::base::MetaFields;
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use std::sync::Arc;
use typemap;


#[derive(Clone)]
pub struct EventsApi {
    clientcfg: Box<BlockchainConn>,
    conn: Box<DataStoreConn>,
}

/// EventsApi: EventsApi provides ability to post the events of the users
/// and manage them.
//
/// URL:
/// POST:/account/:account_id/events,
/// GET: /account/:account_id/events
impl EventsApi {
    pub fn new(datastore: Box<DataStoreConn>, clientcfg: Box<BlockchainConn>) -> Self {
        EventsApi {
            clientcfg: clientcfg,
            conn: datastore,
        }
    }

    //POST: /events
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
        let mut unmarshall_body = self.validate::<AuditEvent>(
            req.get::<bodyparser::Struct<AuditEvent>>()?,
        )?;

        debug!("{} ✓",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        //Send to the eventlogger and return.
        log_event!(req, *unmarshall_body.clone());
        push_notification!(req, *unmarshall_body.clone());

        Ok(render_json(status::Ok, &unmarshall_body))
    }

    //GET: /events
    //Input account_id
    // Returns all the events (for that account)
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_account(req)?;

        let data = ledger::from_config(&self.clientcfg)?;

        match data.retrieve_events(&params) {
            Ok(Some(envelopes)) => Ok(render_json_list(status::Ok, dispatch(req), &envelopes)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
            Err(err) => {
                if format!("{:?}", err).contains("Connection refused") {
                    return Err(badgateway_error(&format!("{}", err)));
                }
                Err(internal_error(&format!("{}", err)))
            }
        }
    }
}

impl Api for EventsApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        //secret API
        router.post("/events", XHandler::new(C { inner: create }), "events");

        router.get("/events", XHandler::new(C { inner: list }), "list_events");
    }
}

impl ApiValidator for EventsApi {}

impl ParmsVerifier for EventsApi {}
