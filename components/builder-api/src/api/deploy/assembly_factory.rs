// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment declaration api assembly_factory
use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, Validator};
use bodyparser;
use bytes::Bytes;
use common::ui;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use deploy::assembler::{Assembler, ServicesConfig};
use deploy::models::{assemblyfactory, blueprint, service, stacksfactory};
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::{MetaFields, Status, StatusUpdate};
use protocol::api::deploy::AssemblyFactory;
use protocol::api::schema::{dispatch, dispatch_url, type_meta};
use protocol::cache::{
    ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_PLAN, CACHE_PREFIX_SERVICE,
    CACHE_PREFIX_STACKS_FACTORY,
};
use router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct AssemblyFactoryApi {
    conn: Box<DataStoreConn>,
}

/// AssemblyFactory API:
/// AssemblyFactoryApi provides ability to declare the blueprints and manage them.
///
/// URL:
/// POST:/account/:account_id/assemblyfactorys,
/// GET: /account/:account_id/assemblyfactorys,
/// GET: /assemblyfactorys/:id
/// GET: /assemblyfactorys  --> list all assemblyfactorys.
/// PUT: /assemblyfactorys/status_update
impl AssemblyFactoryApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        AssemblyFactoryApi { conn: datastore }
    }

    //POST: /accounts/:account_id/assemblyfactory
    //Input: Body of structure deploy::AssemblyFactory
    //Returns an updated AssemblyFactory with id, ObjectMeta. created_at
    fn create(&self, req: &mut Request, _cfg: &ServicesConfig) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<AssemblyFactory>(req.get::<bodyparser::Struct<AssemblyFactory>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            self.verify_account(req)?.get_name(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        unmarshall_body.set_status(Status::pending());

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match Assembler::new(&self.conn, _cfg).assemble(&unmarshall_body) {
            Ok(factory) => Ok(render_json(status::Ok, &factory)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    ///GET: /assemblyfactory/:id
    ///Input: id - u64
    ///Returns AssemblyFactory
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match assemblyfactory::DataStore::new(&self.conn).show(&params) {
            Ok(Some(factory)) => Ok(render_json(status::Ok, &factory)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    ///Wide describe, that provides the full information of an assemblyfactory.
    ///GET: /stacksfactorys/describe/:id
    ///Input: id = u64
    ///Returns StacksFactory with all information (plans)
    ///This doesn't send back spec.assembly_factory
    fn describe(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match assemblyfactory::DataStore::new(&self.conn).show_by_stacksfactory(&params) {
            Ok(Some(factory)) => Ok(render_json_list(status::Ok, dispatch(req), &factory)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    ///PUT: /assemblyfactory/status
    ///Input: Status with conditions
    ///Returns AssemblyFactory with updated status
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match assemblyfactory::DataStore::new(&self.conn).status_update(&unmarshall_body) {
            Ok(Some(factory)) => Ok(render_json(status::Ok, &factory)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    ///Every user will be able to list their own account_id.
    ///GET: /accounts/:account_id/assemblyfactorys/list
    ///Input: account_id
    //Returns all the AssemblyFactorys (for that account)
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_account(req)?;

        match assemblyfactory::DataStore::new(&self.conn).list(&params) {
            Ok(Some(factorys)) => Ok(render_json_list(status::Ok, dispatch(req), &factorys)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for account {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //Will need roles/permission to access this.
    //GET: /assemblyfactorys
    //Returns all the AssemblyFactorys (irrespective of accounts, origins)
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match assemblyfactory::DataStore::new(&self.conn).list_blank() {
            Ok(Some(assembly_factorys)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &assembly_factorys,
            )),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    ///GET: /assemblyfactory/:id
    ///Input: id - u64
    ///Returns AssemblyFactory
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        self.with_cache();
        let res = match assemblyfactory::DataStore::new(&self.conn).show(&idget) {
            Ok(Some(factory)) => {
                let data = json!({
                            "type": typ,
                            "data": factory,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    ///Every user will be able to list their own account_id.
    ///GET: /accounts/:account_id/assemblyfactorys/list
    ///Input: account_id
    //Returns all the AssemblyFactorys (for that account)
    pub fn watch_list_by_account(&mut self, params: IdGet, dispatch: String) -> Option<String> {
        self.with_cache();
        let ident = dispatch_url(dispatch);
        match assemblyfactory::DataStore::new(&self.conn).list(&params) {
            Ok(Some(factorys)) => {
                let data = json!({
                                "api_version": ident.version,
                                "kind": ident.kind,
                                "items": factorys,
                });
                Some(serde_json::to_string(&data).unwrap())
            }
            Ok(None) => None,
            Err(_err) => None,
        }
    }
}

///The Api wirer for AssemblyFactoryApi
///Add all the api needed to be supported under `/assemblyfactory`
///To add an api refer, comments in Api trait.
impl Api for AssemblyFactoryApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : assemblyfactory
        let _config = &config;
        let _service_cfg: Box<ServicesConfig> = Box::new(_config.services.clone().into());
        self.with_cache();

        let mut _self = self.clone();
        let create =
            move |req: &mut Request| -> AranResult<Response> { _self.create(req, &_service_cfg) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let describe = move |req: &mut Request| -> AranResult<Response> { _self.describe(req) };

        let _self = self.clone();
        let status_update =
            move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        //list everything
        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        router.post(
            "/accounts/:account_id/assemblyfactorys",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "assembly_factorys",
        );

        router.get(
            "/accounts/:account_id/assemblyfactorys",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "assemblyfactorys_list",
        );
        router.get(
            "/assemblyfactorys/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "assembly_factorys_show",
        );
        router.get(
            "/stacksfactorys/:id/describe",
            XHandler::new(C { inner: describe }).before(basic.clone()),
            "stacksfactorys_describe",
        );
        router.get(
            "/assemblyfactorys",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "assemblys_factorys_list_blank",
        );
        router.put(
            "/assemblyfactorys/:id/status",
            XHandler::new(C {
                inner: status_update,
            }).before(basic.clone()),
            "assembly_factory_status_update",
        );
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

impl ExpanderSender for AssemblyFactoryApi {
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

        let mut _conn = self.conn.clone();
        _conn.expander.with(plan_service);
        let stacks_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_STACKS_FACTORY.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                debug!("» Stacksfactory live load for ≈ {}", id);
                stacksfactory::DataStore::new(&_conn)
                    .show(&id)
                    .ok()
                    .and_then(|f| serde_json::to_string(&f).ok())
            }),
        ));

        &self.conn.expander.with(stacks_service);
        &self.conn.expander.with(services_service);
    }
}

///Convinient helpers to validating an api
impl ApiValidator for AssemblyFactoryApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for AssemblyFactoryApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct AssemblyFactory
impl Validator for AssemblyFactory {
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

        if self.get_secret().id.len() <= 0 {
            s.push("secret_id".to_string());
        }

        if self.get_replicas() <= 0 {
            s.push("replicas".to_string());
        }
        if self.get_plan().len() <= 0 {
            s.push("plan".to_string());
        }

        if self.object_meta().owner_references.len() <= 0 {
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

        if !self.get_resources().contains_key("compute_type") {
            s.push("compute_type".to_string());
        }
        if !self.get_resources().contains_key("storage_type") {
            s.push("storage_type".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!(
            "{:?} -> {}",
            s, "must have => "
        ))))
    }
}
