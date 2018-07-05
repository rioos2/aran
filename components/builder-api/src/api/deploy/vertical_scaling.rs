// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use bytes::Bytes;
use clusters::models::healthz::DataStore;
use common::ui;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use deploy::models::assembly;
use deploy::replicas_expander::ReplicasExpander;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::{IdGet, MetaFields};
use protocol::api::scale::{VerticalScaling, VerticalScalingStatusUpdate};
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use scale::{scaling, verticalscaling_ds};
use serde_json;
use std::sync::Arc;
use telemetry::metrics::prometheus::PrometheusClient;

#[derive(Clone)]
pub struct VerticalScalingApi {
    conn: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
}

/// verticalscaling api: VerticalScalingApi needs a Datastore mapper to manage state.
/// Hence a DataStoreConn needs to be sent in.
/// Responsible for managing the REST API /assembly
//
/// Assembly: URLs supported are.
impl VerticalScalingApi {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>) -> Self {
        VerticalScalingApi {
            conn: datastore,
            prom: prom,
        }
    }

    //POST: /verticalscaling
    //The body has the input verticalscaling structure.
    //Returns a mutated verticalscaling with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<VerticalScaling>>()?)?;
        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match verticalscaling_ds::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(response)) => Ok(render_json(status::Ok, &response)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //PUT: /verticalscaling/:id/status
    //Input status  as input and returns an VerticalScaling
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body =
            self.validate(req.get::<bodyparser::Struct<VerticalScalingStatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match verticalscaling_ds::DataStore::new(&self.conn).status_update(&unmarshall_body) {
            Ok(Some(vs_update)) => Ok(render_json(status::Ok, &vs_update)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //PUT: /verticalscaling/:id
    //Input verticalscaling id and returns updated verticalscaling
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<VerticalScaling>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match verticalscaling_ds::DataStore::new(&self.conn).update(&unmarshall_body) {
            Ok(Some(vertical)) => Ok(render_json(status::Ok, &vertical)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: /verticalscaling/:id
    //Input id - u64 as input
    //Returns an verticalscaling
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match verticalscaling_ds::DataStore::new(&self.conn).show(&idget) {
            Ok(Some(vertical)) => {
                let data = json!({
                            "type": typ,
                            "data": vertical,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //verticalscaling
    //Global: Returns all the verticalScalings (irrespective of origins)
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match verticalscaling_ds::DataStore::new(&self.conn).list_blank() {
            Ok(Some(vertical)) => Ok(render_json_list(status::Ok, dispatch(req), &vertical)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /verticalscaling/:assembly_factory_id/metrics
    //get metrics for the list of assemblys
    fn metrics(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        let query_pairs = self.default_validate(req)?;
        match scaling::metrics::Client::new(&self.prom).metrics(&params.get_id(), query_pairs) {
            Ok(Some(vs_metrics)) => Ok(render_json_list(status::Ok, dispatch(req), &vs_metrics)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //POST: /verticalscaling/scale/:id
    //Input: Body of structure deploy::Scaling
    //Returns an updated AssemblyFactory with id, ObjectMeta. created_at
    fn scale(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        match verticalscaling_ds::DataStore::new(&self.conn).show(&params) {
            Ok(Some(vs)) => {
                let af_id: Vec<IdGet> = vs.get_owner_references()
                    .iter()
                    .map(|x| IdGet::with_id(x.uid.to_string()))
                    .collect::<Vec<_>>();
                match assembly::DataStore::new(&self.conn).show_by_assemblyfactory(&af_id[0]) {
                    Ok(Some(assemblys)) => {
                        let metrics = DataStore::healthz_all(&self.prom)?;
                        match ReplicasExpander::new(&self.conn, assemblys, metrics, &vs).expand() {
                            Ok(Some(job)) => Ok(render_json(status::Ok, &job)),
                            Err(err) => Err(internal_error(&format!("{}\n", err))),
                            Ok(None) => Err(not_found_error(&format!(
                                "{} for {}",
                                Error::Db(RecordsNotFound),
                                params.get_id()
                            ))),
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
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
}

impl Api for VerticalScalingApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : scaling
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let status_update =
            move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let scale = move |req: &mut Request| -> AranResult<Response> { _self.scale(req) };

        let _self = self.clone();
        let metrics = move |req: &mut Request| -> AranResult<Response> { _self.metrics(req) };

        router.post(
            "/verticalscaling",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "verticalscalings",
        );

        router.put(
            "/verticalscaling/:id/status",
            XHandler::new(C {
                inner: status_update,
            }).before(basic.clone()),
            "vertical_scaling_status_update",
        );
        router.put(
            "/verticalscaling/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "vertical_scaling_update",
        );
        router.get(
            "/verticalscaling",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "vertical_scaling_list_blank",
        );

        router.get(
            "/verticalscaling/scale/:id",
            XHandler::new(C { inner: scale }).before(basic.clone()),
            "verticalscaling_scale",
        );
        router.get(
            "/verticalscaling/:id/metrics",
            XHandler::new(C { inner: metrics }).before(basic.clone()),
            "vertical_scaling_metrics",
        );
    }
}

///We say verticalscalingAPI resource needs to be validated.
impl ApiValidator for VerticalScalingApi {}

///We say verticalscalingAPI needs its parameters to be verified
impl ParmsVerifier for VerticalScalingApi {}

///verify the quer params
impl QueryValidator for VerticalScalingApi {}

///Plugin in a Validator for verticalscaling structure
///For now we don't validate anything.
impl Validator for VerticalScaling {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }
        if self.object_meta().owner_references.len() < 1 {
            s.push("owner_references".to_string());
        } else {
            self.object_meta()
                .owner_references
                .iter()
                .map(|x| {
                    if x.uid.len() <= 0 {
                        s.push("uid".to_string());
                    }
                })
                .collect::<Vec<_>>();
        }

        if self.get_scale_type().len() <= 0 {
            s.push("scale_type".to_string());
        }
        if self.get_state().len() <= 0 {
            s.push("state".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
impl Validator for VerticalScalingStatusUpdate {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
