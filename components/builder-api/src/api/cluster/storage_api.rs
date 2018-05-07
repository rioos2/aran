// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [origin] for the HTTP server

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, Validator, ParmsVerifier};
use protocol::api::schema::{dispatch, type_meta};
use config::Config;
use error::Error;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};

use storage::storage_ds::StorageDS;
use protocol::api::base::StatusUpdate;
use protocol::api::storage::{Storage, DataCenter, StoragePool};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use protocol::api::base::MetaFields;
use error::ErrorMessage::MissingParameter;
use ansi_term::Colour;
use common::ui;
use bytes::Bytes;
use serde_json;
use protocol::api::base::IdGet;

#[derive(Clone)]
pub struct StorageApi {
    conn: Arc<DataStoreConn>,
}

/// Storage api
/// - every instance of StoragesApi needs a DataStoreConn
/// StorageConnector
/// POST: storageconnectors, GET: storageconnectors/:id, GET: storageconnectors PUT: storageconnectors/status_update
/// StoragePool
/// POST: storagepools, GET: storagepools/:id, GET: storagepools PUT: storagepools/status_update
/// Datacenters
/// POST: datacenters, GET: datacenters/:id, GET: datacenters PUT: datacenters/status_update
impl StorageApi {
    pub fn new(datastore: Arc<DataStoreConn>) -> Self {
        StorageApi { conn: datastore }
    }
    //POST: /storageconnectors
    //The body has the input cluster::strorage
    //Returns a mutated Storage  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Storage>>()?)?;

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
        match StorageDS::storage_create(&self.conn, &unmarshall_body) {
            Ok(storage) => Ok(render_json(status::Ok, &storage)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /storagesconnectors
    //Blank origin: Returns all the Storage(irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list(&self, _req: &mut Request) -> AranResult<Response> {
        match StorageDS::storage_list(&self.conn) {
            Ok(Some(storage_list)) => Ok(render_json_list(status::Ok, dispatch(_req), &storage_list)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /storageconnectors/:id
    //Input id - u64 as input and returns a storage
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match StorageDS::storage_show(&self.conn, &params) {
            Ok(Some(storage)) => Ok(render_json(status::Ok, &storage)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /storageconnectors/:id
    //Input id - u64 as input
    //Returns an secrets
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match StorageDS::storage_show(&self.conn, &idget) {
            Ok(Some(storage)) => {
                let data = json!({
                            "type": typ,
                            "data": storage,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //PUT: /storagesconnectors/:id
    //Input storageconnector id and returns updated storageconnector
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Storage>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match StorageDS::storage_update(&self.conn, &unmarshall_body) {
            Ok(Some(storage_create)) => Ok(render_json(status::Ok, &storage_create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
    //PUT: /storageconnectors/:id/status
    //Input storageconnector id and returns updated storageconnector
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<StatusUpdate>>()?,
        )?;
        unmarshall_body.set_id(params.get_id());

        match StorageDS::storage_status_update(&self.conn, &unmarshall_body) {
            Ok(Some(storage_create)) => Ok(render_json(status::Ok, &storage_create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //POST: /datacenters
    //The body has the input cluster::datacenters
    //Returns a mutated Datacenter  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn data_center_create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<DataCenter>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        match StorageDS::data_center_create(&self.conn, &unmarshall_body) {
            Ok(dc_create) => Ok(render_json(status::Ok, &dc_create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /datacenters
    //Blank origin: Returns all the Datacenters(irrespective of namespaces)
    //Will need roles/permission to access this.

    fn data_center_list(&self, _req: &mut Request) -> AranResult<Response> {
        match StorageDS::data_center_list(&self.conn) {
            Ok(Some(data_center_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &data_center_list,
            )),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /datacenters/:id
    //Input id - u64 as input and returns a datacenter
    fn data_center_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        match StorageDS::data_center_show(&self.conn, &params) {
            Ok(Some(dc)) => Ok(render_json(status::Ok, &dc)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    // PUT : /datacenters/:id
    //Update datacenter data
    fn datacenter_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<DataCenter>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match StorageDS::datacenter_update(&self.conn, &unmarshall_body) {
            Ok(Some(update)) => Ok(render_json(status::Ok, &update)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /datacenters/:id
    //Input id - u64 as input
    //Returns an datacenters
    pub fn data_center_watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match StorageDS::data_center_show(&self.conn, &idget) {
            Ok(Some(dc)) => {
                let data = json!({
                            "type": typ,
                            "data": dc,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //POST: /storagepool
    //The body has the input cluster::stroragepool
    //Returns a mutated Storagepool  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn storage_pool_create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StoragePool>>()?)?;

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

        match StorageDS::storage_pool_create(&self.conn, &unmarshall_body) {
            Ok(Some(storage)) => Ok(render_json(status::Ok, &storage)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //PUT: /storagepool/:id/status
    //Input storagepool id and returns updated storagepool
    fn storage_pool_status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<StatusUpdate>>()?,
        )?;

        unmarshall_body.set_id(params.get_id());

        match StorageDS::storage_pool_status_update(&self.conn, &unmarshall_body) {
            Ok(Some(storage_pool_update)) => Ok(render_json(status::Ok, &storage_pool_update)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /storageconnectors/:id/storagespool
    //Blank origin: Returns all the Storagepool(irrespective of namespaces)
    //Will need roles/permission to access this.
    fn storage_pool_by_connector(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match StorageDS::storage_pool_list(&self.conn, &params) {
            Ok(Some(storage)) => Ok(render_json_list(status::Ok, dispatch(req), &storage)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /storagespool/:id
    //Input id - u64 as input and returns a storagespool
    fn storagepool_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        match StorageDS::storage_pool_show(&self.conn, &params) {
            Ok(Some(storagepool)) => Ok(render_json(status::Ok, &storagepool)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }


    //GET: /storagespool
    //Blank origin: Returns all the Storagepool(irrespective of namespaces)
    //Will need roles/permission to access this.
    fn storage_pool_list(&self, _req: &mut Request) -> AranResult<Response> {
        match StorageDS::storage_pool_list_all(&self.conn) {
            Ok(Some(storage_pool_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &storage_pool_list,
            )),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /storagespool/:id
    //Input id - u64 as input
    //Returns an storagespool
    pub fn storage_pool_watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match StorageDS::storage_pool_show(&self.conn, &idget) {
            Ok(Some(pool)) => {
                let data = json!({
                            "type": typ,
                            "data": pool,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for StorageApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : storage connectors
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        //closures : storagepools
        let _self = self.clone();
        let storage_pool_create = move |req: &mut Request| -> AranResult<Response> { _self.storage_pool_create(req) };

        let _self = self.clone();
        let storage_pool_list = move |req: &mut Request| -> AranResult<Response> { _self.storage_pool_list(req) };

        let _self = self.clone();
        let storage_pool_status_update = move |req: &mut Request| -> AranResult<Response> { _self.storage_pool_status_update(req) };

        let _self = self.clone();
        let storage_pool_by_connector = move |req: &mut Request| -> AranResult<Response> { _self.storage_pool_by_connector(req) };

        let _self = self.clone();
        let storagepool_show = move |req: &mut Request| -> AranResult<Response> { _self.storagepool_show(req) };


        //closures : datacenters
        let _self = self.clone();
        let data_center_create = move |req: &mut Request| -> AranResult<Response> { _self.data_center_create(req) };

        let _self = self.clone();
        let data_center_list = move |req: &mut Request| -> AranResult<Response> { _self.data_center_list(req) };

        let _self = self.clone();
        let data_center_show = move |req: &mut Request| -> AranResult<Response> { _self.data_center_show(req) };

        let _self = self.clone();
        let datacenter_update = move |req: &mut Request| -> AranResult<Response> { _self.datacenter_update(req) };

        router.post(
            "/storageconnectors",
            XHandler::new(C { inner: create })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storageconnector.post".to_string(),&*config)),
            "storages",
        );
        router.get(
            "/storageconnectors",
            XHandler::new(C { inner: list })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storageconnector.get".to_string(),&*config)),
            "storages_list",
        );
        router.get(
            "/storageconnectors/:id",
            XHandler::new(C { inner: show })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storageconnector.get".to_string(),&*config)),
            "storages_show",
        );
        router.put(
            "storageconnectors/:id/status",
            XHandler::new(C { inner: status_update })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storageconnector.put".to_string(),&*config)),
            "storages_status_update",
        );
        router.put(
            "storageconnectors/:id",
            XHandler::new(C { inner: update })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storageconnector.put".to_string(),&*config)),
            "storages_update",
        );

        //StoragePool API
        router.post(
            "/storagespool",
            XHandler::new(C { inner: storage_pool_create })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storagepool.post".to_string(),&*config)),
            "storages_pool",
        );
        router.get(
            "/storageconnectors/:id/storagespool",
            XHandler::new(C { inner: storage_pool_by_connector })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storagepool.get".to_string(),&*config)),
            "storages_pool_show_by_connector",
        );
        router.get(
            "/storagespool",
            XHandler::new(C { inner: storage_pool_list })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storagepool.get".to_string(),&*config)),
            "storages_pool_list",
        );
        router.put(
            "/storagespool/:id/status",
            XHandler::new(C { inner: storage_pool_status_update })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storagepool.put".to_string(),&*config)),
            "storages_pool_status_update",
        );
        router.get(
            "/storagespool/:id",
            XHandler::new(C { inner: storagepool_show })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.storagepool.get".to_string(),&*config)),
            "storagepool_show",
        );



        //DataCenter API
        router.post(
            "/datacenters",
            XHandler::new(C { inner: data_center_create })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.datacenter.post".to_string(),&*config)),
            "data_center",
        );
        router.get(
            "/datacenters",
            XHandler::new(C { inner: data_center_list })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.datacenter.get".to_string(),&*config)),
            "data_center_list",
        );
        router.get(
            "/datacenters/:id",
            XHandler::new(C { inner: data_center_show })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.datacenter.get".to_string(),&*config)),
            "data_center_show",
        );

        router.put(
            "/datacenters/:id",
            XHandler::new(C { inner: datacenter_update })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.datacenter.put".to_string(),&*config)),
            "data_center_update",
        );
    }
}

impl ApiValidator for StorageApi {}

impl ParmsVerifier for StorageApi {}

impl Validator for Storage {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_host_ip().len() <= 0 {
            s.push("host_ip".to_string());
        }

        if self.get_storage_type().len() <= 0 {
            s.push("storage_type".to_string());
        }

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for StoragePool {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_connector_id().len() <= 0 {
            s.push("connector_id".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for DataCenter {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.get_storage().len() <= 0 {
            s.push("storage".to_string());
        }

        if self.get_networks().len() <= 0 {
            s.push("networks".to_string());
        }
        if self.get_nodes().len() <= 0 {
            s.push("nodes".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
