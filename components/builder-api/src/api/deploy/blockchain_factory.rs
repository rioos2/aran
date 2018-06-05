// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment declaration api blockchain_factory
use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use api::{Api, ApiValidator, ExpanderSender, ParmsVerifier, Validator};
use protocol::api::schema::{dispatch, dispatch_url, type_meta};

use config::Config;
// use api::deploy::config::ServicesCfg;
use error::Error;
use error::ErrorMessage::MissingParameter;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};

use deploy::assembler::{Assembler, ServicesConfig};
use deploy::models::{blockchainfactory, blueprint, service};

use protocol::cache::{NewCacheServiceFn, CACHE_PREFIX_PLAN, CACHE_PREFIX_SERVICE};
use protocol::api::deploy::BlockchainFactory;
use protocol::api::base::{MetaFields, Status, StatusUpdate};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;

use bytes::Bytes;

#[derive(Clone)]
pub struct BlockchainFactoryApi {
    conn: Box<DataStoreConn>,
}

/// BlockchainFactory API:
/// BlockchainFactoryApi provides ability to declare the blueprints and manage them.
///
/// URL:
/// POST:/account/:account_id/blockchainfactorys,
/// GET: /account/:account_id/blockchainfactorys,
/// GET: /blockchainfactorys/:id
/// GET: /blockchainfactorys  --> list all blockchainfactorys.
/// PUT: /blockchainfactorys/status_update
impl BlockchainFactoryApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        BlockchainFactoryApi { conn: datastore }
    }

    ///GET: /blockchainfactory/:id
    ///Input: id - u64
    ///Returns BlockchainFactory
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match blockchainfactory::DataStore::new(&self.conn).show(&params) {
            Ok(Some(factory)) => Ok(render_json(status::Ok, &factory)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    ///PUT: /blockchainfactory/status
    ///Input: Status with conditions
    ///Returns BlockchainFactory with updated status
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match blockchainfactory::DataStore::new(&self.conn).status_update(&unmarshall_body) {
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
    ///GET: /accounts/:account_id/blockchainfactorys/list
    ///Input: account_id
    //Returns all the BlockchainFactorys (for that account)
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_account(req)?;

        match blockchainfactory::DataStore::new(&self.conn).list(&params) {
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
    //GET: /blockchainfactorys
    //Returns all the BlockchainFactorys (irrespective of accounts, origins)
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match blockchainfactory::DataStore::new(&self.conn).list_blank() {
            Ok(Some(blockchain_factorys)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &blockchain_factorys,
            )),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    ///GET: /blockchainfactory/:id
    ///Input: id - u64
    ///Returns BlockchainFactory
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        self.with_cache();
        let res = match blockchainfactory::DataStore::new(&self.conn).show(&idget) {
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
    ///GET: /accounts/:account_id/blockchainfactorys/list
    ///Input: account_id
    //Returns all the BlockchainFactorys (for that account)
    pub fn watch_list_by_account(&mut self, params: IdGet, dispatch: String) -> Option<String> {
        self.with_cache();
        let ident = dispatch_url(dispatch);
        match blockchainfactory::DataStore::new(&self.conn).list(&params) {
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

///The Api wirer for BlockchainFactoryApi
///Add all the api needed to be supported under `/blockchainfactory`
///To add an api refer, comments in Api trait.
impl Api for BlockchainFactoryApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : blockchainfactory
        let _config = &config;
        let _service_cfg: Box<ServicesConfig> = Box::new(_config.services.clone().into());
        self.with_cache();

        // let mut _self = self.clone();
        // let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req, &_service_cfg) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        //list everything
        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };


        router.get(
            "/accounts/:account_id/blockchainfactorys",
            XHandler::new(C { inner: list })
                .before(basic.clone())
                .before(TrustAccessed::new(
                    "rioos.blockchainfactory.get".to_string(),
                    &*config,
                )),
            "blockchainfactorys_list",
        );
        router.get(
            "/blockchainfactorys/:id",
            XHandler::new(C { inner: show })
                .before(basic.clone())
                .before(TrustAccessed::new(
                    "rioos.blockchainfactory.get".to_string(),
                    &*config,
                )),
            "blockchain_factorys_show",
        );
        router.get(
            "/blockchainfactorys",
            XHandler::new(C { inner: list_blank })
                .before(basic.clone())
                .before(TrustAccessed::new(
                    "rioos.blockchainfactory.get".to_string(),
                    &*config,
                )),
            "blockchains_factorys_list_blank",
        );
        router.put(
            "/blockchainfactorys/:id/status",
            XHandler::new(C {
                inner: status_update,
            }).before(basic.clone())
                .before(TrustAccessed::new(
                    "rioos.blockchainfactory.put".to_string(),
                    &*config,
                )),
            "blockchain_factory_status_update",
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

impl ExpanderSender for BlockchainFactoryApi {
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
                service::DataStore::list_by_blockchain_factory(&_conn, &id)
                    .ok()
                    .and_then(|v| serde_json::to_string(&v).ok())
            }),
        ));

        &self.conn.expander.with(plan_service);
        &self.conn.expander.with(services_service);
    }
}

pub struct ServicesCfg {
    pub loadbalancer_imagein: String,
    pub loadbalancer_imagename: String,
    pub loadbalancer_cpu: String,
    pub loadbalancer_mem: String,
    pub loadbalancer_disk: String,
}

impl Default for ServicesCfg {
    fn default() -> Self {
        ServicesCfg {
            loadbalancer_imagein: "container".to_string(),
            loadbalancer_imagename: "registry.rioos.xyz:5000/rioos/loadbalancer".to_string(),
            loadbalancer_cpu: "1".to_string(),
            loadbalancer_mem: "1024 MiB".to_string(),
            loadbalancer_disk: "1 GiB".to_string(),
        }
    }
}

/// Convert into ServicesConfig  from the ServiceCfg provided as defaults.
impl Into<ServicesConfig> for ServicesCfg {
    fn into(self) -> ServicesConfig {
        ServicesConfig {
            loadbalancer_imagein: self.loadbalancer_imagein,
            loadbalancer_imagename: self.loadbalancer_imagename,
            loadbalancer_cpu: self.loadbalancer_cpu,
            loadbalancer_mem: self.loadbalancer_mem,
            loadbalancer_disk: self.loadbalancer_disk,
        }
    }
}



///Convinient helpers to validating an api
impl ApiValidator for BlockchainFactoryApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for BlockchainFactoryApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct BlockchainFactory
impl Validator for BlockchainFactory {
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
