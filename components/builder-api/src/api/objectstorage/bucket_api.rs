// Copyright 2018 The Rio Advancement Inc
//

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, ParmsVerifier, Validator};
use config::Config;
use error::Error;
use protocol::api::schema::{dispatch, type_meta};

use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

use api::objectstorage::config::ObjectStorageCfg;
use protocol::api::base::MetaFields;
use protocol::api::objectstorage::Bucket;

use super::s3;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;

#[derive(Clone)]
pub struct ObjectStorageApi {
    conn: Box<ObjectStorageCfg>,
}

impl ObjectStorageApi {
    pub fn new(conn: Box<ObjectStorageCfg>) -> Self {
        ObjectStorageApi { conn: conn }
    }

    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Bucket>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let client = s3::from_config(&self.conn)?;

        //object storage bucket create_bucket with name
        match client.create_bucket(&unmarshall_body) {
            Ok(bucket) => Ok(render_json(status::Ok, &bucket)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        let client = s3::from_config(&self.conn)?;

        match client.list_bucket() {
            Ok(Some(buckets)) => Ok(render_json_list(status::Ok, dispatch(_req), &buckets)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    fn upload(&self, req: &mut Request) -> AranResult<Response> {
        let params_id = self.verify_id_with_name(req)?;
        let params_name = self.verify_name(req)?;

        let client = s3::from_config(&self.conn)?;

        match client.upload_accessor(params_id.get_id(), params_name.get_id()) {
            Ok(bucket) => Ok(render_json(status::Ok, &bucket)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    fn download(&self, req: &mut Request) -> AranResult<Response> {
        let params_id = self.verify_id_with_name(req)?;
        let params_name = self.verify_name(req)?;

        let client = s3::from_config(&self.conn)?;

        match client.download_accessor(params_id.get_id(), params_name.get_id()) {
            Ok(bucket) => Ok(render_json(status::Ok, &bucket)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }
}

impl Api for ObjectStorageApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : storage connectors
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let upload = move |req: &mut Request| -> AranResult<Response> { _self.upload(req) };

        let _self = self.clone();
        let download = move |req: &mut Request| -> AranResult<Response> { _self.download(req) };

        router.post(
            "/buckets",
            XHandler::new(C {
                inner: create.clone(),
            }).before(basic.clone()),
            "account_buckets_create",
        );        
        router.get(
            "/buckets",
            XHandler::new(C {
                inner: list_blank.clone(),
            }).before(basic.clone()),
            "buckets_list",
        );
        router.post(
            "/bucketfiles/:id/files/:name",
            XHandler::new(C { inner: upload }).before(basic.clone()),
            "buckets_upload",
        );
        router.get(
            "/bucketfiles/:id/files/:name",
            XHandler::new(C { inner: download }).before(basic.clone()),
            "buckets_download",
        );
    }
}

impl ApiValidator for ObjectStorageApi {}

impl ParmsVerifier for ObjectStorageApi {}

impl Validator for Bucket {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
