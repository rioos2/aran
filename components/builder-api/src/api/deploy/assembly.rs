// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory] for the HTTP server
use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use api::{Api, ApiValidator, Validator, ParmsVerifier, ExpanderSender};
use rio_net::http::schema::{dispatch, type_meta, dispatch_url};
use config::Config;
use error::Error;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};
use rio_net::metrics::prometheus::PrometheusClient;

use deploy::models::{assembly, assemblyfactory, endpoint, volume, blueprint};

use protocol::cache::{CACHE_PREFIX_PLAN, CACHE_PREFIX_FACTORY, CACHE_PREFIX_ENDPOINT, CACHE_PREFIX_VOLUME, CACHE_PREFIX_METRIC};
use protocol::cache::NewCacheServiceFn;
use protocol::api::deploy::Assembly;
use protocol::api::base::StatusUpdate;
use protocol::api::base::MetaFields;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;

use bytes::Bytes;

#[derive(Clone)]
pub struct AssemblyApi {
    conn: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
}

/// Assembly API:
/// AssemblyApi provides ability to manage the instances produced by the declaration in the blueprint AssemblyFactory.
///
/// URL:
/// POST:/account/:account_id/assemblys,
/// GET: /account/:account_id/assemblys,
/// GET: /assemblys/:id
/// PUT: /assemblys/update
/// PUT: /assemblys/status_update
/// GET: /assemblys  --> list all assemblys.
impl AssemblyApi {
    pub fn new(datastore: Box<DataStoreConn>, promconn: Box<PrometheusClient>) -> Self {
        AssemblyApi { conn: datastore, prom: promconn }
    }

    //POST: /accounts/:account_id/assemblys
    //Input: Body of structure deploy::Assembly
    //Returns an updated Assembly with id, ObjectMeta. created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate::<Assembly>(req.get::<bodyparser::Struct<Assembly>>()?)?;

        let m = unmarshall_body.mut_meta(unmarshall_body.object_meta(), unmarshall_body.get_name(), self.verify_account(req)?.get_name());

        unmarshall_body.set_meta(type_meta(req), m);

        ui::rawdumpln(Colour::White, '✓', format!("======= parsed {:?} ", unmarshall_body));

        match assembly::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    ///Wide describe, that provides the full information of an assemblyfactory.
    ///GET: /assemblyfactory/describe/:id
    ///Input: id = u64
    ///Returns AssemblyFactory with all information (general, plans, scaling)
    fn describe(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match assembly::DataStore::new(&self.conn).show_by_assemblyfactory(&params) {
            Ok(Some(factory)) => Ok(render_json_list(status::Ok, dispatch(req), &factory)),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound), &params.get_id()))),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /assembly/:id
    //Input id - u64 as input
    //Returns an Assembly
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match assembly::DataStore::new(&self.conn).show(&params) {
            Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound), params.get_id()))),
        }
    }

    ///Every user will be able to list their own account_id.
    ///Will need roles/permission to access others account_id.
    ///GET: /accounts/:account_id/assemblys/list
    ///Input account_id
    ///Returns all the Assemblys (for that account)
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_account(req)?;

        match assembly::DataStore::new(&self.conn).list(&params) {
            Ok(Some(assemblys)) => Ok(render_json_list(status::Ok, dispatch(req), &assemblys)),
            Ok(None) => Err(not_found_error(&format!("{} for account {}", Error::Db(RecordsNotFound), &params.get_id()))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    ///Every user will be able to list their own account_id.
    ///Will need roles/permission to access others account_id.
    ///GET: /accounts/:account_id/assemblys/list
    ///Input account_id
    ///Returns all the Assemblys (for that account)
    pub fn list_by_account_direct(&self, params: IdGet, dispatch: String) -> Option<String> {
        let ident = dispatch_url(dispatch);
        match assembly::DataStore::new(&self.conn).list(&params) {
            Ok(Some(assemblys)) => {
                let data = json!({
                                "api_version": ident.version,
                                "kind": ident.kind,
                                "items": assemblys,
                });
                Some(serde_json::to_string(&data).unwrap())
            }
            Ok(None) => None,
            Err(_err) => None,
        }
    }

    ///PUT: /assembly/:id
    ///Input assembly id
    ///Returns updated assemblyfactory
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Assembly>>()?)?;
        let m = unmarshall_body.mut_meta(unmarshall_body.object_meta(), unmarshall_body.get_name(), unmarshall_body.get_account());

        unmarshall_body.set_meta(type_meta(req), m);
        unmarshall_body.set_id(params.get_id());
        match assembly::DataStore::new(&self.conn).update(&unmarshall_body) {
            Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound), params.get_id()))),
        }
    }

    ///PUT: /assemblyfactory/status
    ///Input Status  as input
    ///Returns an AssemblyFactory
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match assembly::DataStore::new(&self.conn).status_update(&unmarshall_body) {
            Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound), params.get_id()))),
        }
    }

    //Global: Returns all the AssemblyFactorys (irrespective of origins)
    //GET: /assembly
    //Will need roles/permission to access this.
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match assembly::DataStore::new(&self.conn).list_blank() {
            Ok(Some(assemblys)) => Ok(render_json_list(status::Ok, dispatch(req), &assemblys)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /assembly/:id
    //Input id - u64 as input
    //Returns an Assembly
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        self.with_cache();
        let res = match assembly::DataStore::new(&self.conn).show(&idget) {
            Ok(Some(assembly)) => {
                let data = json!({
                            "type": typ,
                            "data": assembly,      
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

///The Api wirer for AssemblyFactoryApi
///Add all the api needed to be supported under `/assemblys`
///To add an api refer, comments in Api trait.
impl Api for AssemblyApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures: assembly
        self.with_cache();
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let describe = move |req: &mut Request| -> AranResult<Response> { _self.describe(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        //routes: assemblys
        router.post("/accounts/:account_id/assemblys", XHandler::new(C { inner: create }).before(basic.clone()), "assemblys");
        router.get("/accounts/:account_id/assemblys", XHandler::new(C { inner: list }).before(basic.clone()), "assembly_list");
        router.get("/assemblys/:id", XHandler::new(C { inner: show }).before(basic.clone()), "assembly_show");
        //Special move here from assemblyfactory code. We have  moved it here since
        //the expanders for endpoints, volume are missing assembly factory,
        router.get("/assemblyfactorys/:id/describe", XHandler::new(C { inner: describe }).before(basic.clone()), "assemblyfactorys_describe");
        router.get("/assemblys", XHandler::new(C { inner: list_blank }).before(basic.clone()), "assembly_list_blank");

        router.put("/assemblys/:id/status", XHandler::new(C { inner: status_update }).before(basic.clone()), "assembly_status");
        router.put("/assemblys/:id", XHandler::new(C { inner: update }).before(basic.clone()), "assembly_update");
    }
}

///Setup the cache sender for this api.
///Essentially hookup all the computation intensive strategry
///that will reload the cache using closures.
///Setup the cache sender for this api.
///Essentially hookup all the computation intensive strategry
///that will reload the cache using closures.
use protocol::api::base::IdGet;
use serde_json;

impl ExpanderSender for AssemblyApi {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let plan_service = Box::new(NewCacheServiceFn::new(CACHE_PREFIX_PLAN.to_string(), Box::new(move |id: IdGet| -> Option<String> { blueprint::DataStore::show(&_conn, &id).ok().and_then(|p| serde_json::to_string(&p).ok()) })));

        let mut _conn = self.conn.clone();
        _conn.expander.with(plan_service);

        let factory_service = Box::new(NewCacheServiceFn::new(CACHE_PREFIX_FACTORY.to_string(), Box::new(move |id: IdGet| -> Option<String> { assemblyfactory::DataStore::new(&_conn).show(&id).ok().and_then(|f| serde_json::to_string(&f).ok()) })));

        let _conn = self.conn.clone();

        let endpoint_service = Box::new(NewCacheServiceFn::new(CACHE_PREFIX_ENDPOINT.to_string(), Box::new(move |id: IdGet| -> Option<String> { endpoint::DataStore::show_by_assembly(&_conn, &id).ok().and_then(|e| serde_json::to_string(&e).ok()) })));

        let _conn = self.conn.clone();

        let volume_service = Box::new(NewCacheServiceFn::new(CACHE_PREFIX_VOLUME.to_string(), Box::new(move |id: IdGet| -> Option<String> { volume::DataStore::show_by_assembly(&_conn, &id).ok().and_then(|v| serde_json::to_string(&v).ok()) })));
        let _conn = self.conn.clone();
        let _prom = self.prom.clone();

        let metric_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_METRIC.to_string(),
            Box::new(move |id: IdGet| -> Option<String> { assembly::DataStore::new(&_conn).show_metrics(&id, &_prom).ok().and_then(|m| serde_json::to_string(&m).ok()) }),
        ));

        &self.conn.expander.with(factory_service);
        &self.conn.expander.with(endpoint_service);
        &self.conn.expander.with(volume_service);
        &self.conn.expander.with(metric_service);
    }
}

///Convinient helpers to validating an api
impl ApiValidator for AssemblyApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for AssemblyApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct Assembly
impl Validator for Assembly {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }
        if self.object_meta().cluster_name.len() <= 0 {
            s.push("cluster_name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }
        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
