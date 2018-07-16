// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use bytes::Bytes;
use common::ui;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use deploy::assembler::{Assembler, ServicesConfig};
use deploy::models::{assemblyfactory, blueprint, service};
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::{IdGet, MetaFields};
use protocol::api::scale::{HorizontalScaling, StatusUpdate};
use protocol::api::schema::{dispatch, type_meta};
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_PLAN, CACHE_PREFIX_SERVICE};
use router::Router;
use scale::{horizontalscaling_ds, scaling};
use serde_json;
use std::sync::Arc;
use telemetry::metrics::prometheus::PrometheusClient;

#[derive(Clone)]
pub struct HorizontalScalingApi {
    conn: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
}

/// HorizontalScaling api: HorizontalScalingApi needs a Datastore mapper to manage state.
/// Hence a DataStoreConn needs to be sent in.
/// Responsible for managing the REST API /assembly
//
/// Assembly: URLs supported are.
/// POST: /origin/:originid/assemblyfactory,
/// GET: /assemblyfactory/:id
/// PUT: /assemblyfactory/status_update
impl HorizontalScalingApi {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>) -> Self {
        HorizontalScalingApi {
            conn: datastore,
            prom: prom,
        }
    }

    //POST: /horizontalscaling
    //The body has the input HorizontalScaling structure.
    //Returns a mutated HorizontalSclaing with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate(req.get::<bodyparser::Struct<HorizontalScaling>>()?)?;
        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match horizontalscaling_ds::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(response)) => Ok(render_json(status::Ok, &response)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: /horizontalscaling/:id/scale
    //Input: Body of structure deploy::Scaling
    //Returns an updated AssemblyFactory with id, ObjectMeta. created_at
    fn scale(&self, req: &mut Request, _cfg: &ServicesConfig) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        match horizontalscaling_ds::DataStore::new(&self.conn).show(&params) {
            Ok(Some(hs)) => {
                let af_id: Vec<IdGet> = hs.get_owner_references()
                    .iter()
                    .map(|x| IdGet::with_id(x.uid.to_string()))
                    .collect::<Vec<_>>();
                match assemblyfactory::DataStore::new(&self.conn).show(&af_id[0]) {
                    Ok(Some(factory)) => match Assembler::new(&self.conn, _cfg).reassemble(
                        hs.get_status().get_desired_replicas(),
                        hs.get_status().get_current_replicas(),
                        &factory,
                    ) {
                        Ok(factory) => Ok(render_json(status::Ok, &factory)),
                        Err(err) => Err(internal_error(&format!("{}\n", err))),
                    },
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

    //GET: /horizontalscaling/assemblyfactory/:id

    //Input assembly factory id Returns horizontal_scaling
    //Will need roles/permission to access others horizontal_scaling
    fn show_by_assembly_factory(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match horizontalscaling_ds::DataStore::new(&self.conn).show_by_assembly_factory(&params) {
            Ok(Some(scale)) => Ok(render_json(status::Ok, &scale)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /horizontalscaling/:id
    //Input id - u64 as input
    //Returns an horizontalscaling
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match horizontalscaling_ds::DataStore::new(&self.conn).show(&idget) {
            Ok(Some(hs)) => {
                let data = json!({
                            "type": typ,
                            "data": hs,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
    //GET: /horizontalscaling/:assembly_factory_id/metrics
    //get metrics for the list of assemblys
    fn metrics(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        let query_pairs = self.default_validate(req)?;
        match scaling::metrics::Client::new(&self.prom).metrics(&params.get_id(), query_pairs) {
            Ok(Some(hs_metrics)) => Ok(render_json_list(status::Ok, dispatch(req), &hs_metrics)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //PUT: /horizontalscaling/status
    //Input status  as input and returns an HorizontalScaling
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match horizontalscaling_ds::DataStore::new(&self.conn).status_update(&unmarshall_body) {
            Ok(Some(hs_update)) => Ok(render_json(status::Ok, &hs_update)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //PUT: /horizontalscaling/:id
    //Input horizontalscaling id and returns updated horizontalscaling
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body =
            self.validate(req.get::<bodyparser::Struct<HorizontalScaling>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match horizontalscaling_ds::DataStore::new(&self.conn).update(&unmarshall_body) {
            Ok(Some(hs)) => Ok(render_json(status::Ok, &hs)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //horizontalscaling
    //Global: Returns all the HorizontalScalings (irrespective of origins)
    //Will need roles/permission to access this.
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match horizontalscaling_ds::DataStore::new(&self.conn).list_blank() {
            Ok(Some(hss)) => Ok(render_json_list(status::Ok, dispatch(req), &hss)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for HorizontalScalingApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _config = &config;
        let _service_cfg: Box<ServicesConfig> = Box::new(_config.services.clone().into());

        self.with_cache();

        //closures : scaling
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let metrics = move |req: &mut Request| -> AranResult<Response> { _self.metrics(req) };

        let _self = self.clone();
        let status_update =
            move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let show_by_assembly_factory = move |req: &mut Request| -> AranResult<Response> {
            _self.show_by_assembly_factory(req)
        };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let scale =
            move |req: &mut Request| -> AranResult<Response> { _self.scale(req, &_service_cfg) };

        router.post(
            "/horizontalscaling",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "horizontal_scalings",
        );

        router.put(
            "/horizontalscaling/:id/status",
            XHandler::new(C {
                inner: status_update,
            }).before(basic.clone()),
            "horizontal_scaling_status_update",
        );
        router.put(
            "/horizontalscaling/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "horizontal_scaling_update",
        );
        router.get(
            "/horizontalscaling/:id/metrics",
            XHandler::new(C { inner: metrics }).before(basic.clone()),
            "horizontal_scaling_metrics",
        );

        router.get(
            "/horizontalscaling/:id/scale",
            XHandler::new(C { inner: scale }).before(basic.clone()),
            "horizontal_scaling",
        );

        router.get(
            "/horizontalscaling/assemblyfactory/:id",
            XHandler::new(C {
                inner: show_by_assembly_factory,
            }).before(basic.clone()),
            "horizontal_scaling_show_by_assembly_factory",
        );

        router.get(
            "/horizontalscaling",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "horizontal_scaling_list_blank",
        );
    }
}

impl ExpanderSender for HorizontalScalingApi {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();

        let plan_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_PLAN.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                blueprint::DataStore::show(&_conn, &id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        let _conn = self.conn.clone();

        let services_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_SERVICE.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                service::DataStore::list_by_assembly_factory(&_conn, &id)
                    .ok()
                    .and_then(|v| serde_json::to_string(&v).ok())
            }),
        ));

        &self.conn.expander.with(plan_service);
        &self.conn.expander.with(services_service);
    }
}
///We say HorizontalScalingAPI resource needs to be validated.
impl ApiValidator for HorizontalScalingApi {}

///We say HorizontalScalingAPI needs its parameters to be verified
impl ParmsVerifier for HorizontalScalingApi {}

///verify the quer params
impl QueryValidator for HorizontalScalingApi {}

///Plugin in a Validator for HorizontalScaling structure
///For now we don't validate anything.
impl Validator for HorizontalScaling {
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
impl Validator for StatusUpdate {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
