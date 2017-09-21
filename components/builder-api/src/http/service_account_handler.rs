use bodyparser;
use rio_net::http::controller::*;
use service::service_account_ds::ServiceAccountDS;
use iron::prelude::*;
use iron::status;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use protocol::servicesrv::{Secret, SecretGet, ObjectReference, ServiceAccount, ObjectMetaData, ServiceAccountGet};
use protocol::asmsrv::TypeMeta;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SecretCreateReq {
    data: BTreeMap<String, String>,
    object_meta: ObjectMetaReq,
    type_meta: TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServiceAccountCreateReq {
    secrets: ObjectReferenceReq,
    object_meta: ObjectMetaReq,
    type_meta: TypeMetaReq,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct TypeMetaReq {
    kind: String,
    api_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjectMetaReq {
    name: String,
    origin: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjectReferenceReq {
    kind: String,
    name: String,
    origin: String,
    uid: String,
}

pub fn secret_create(req: &mut Request) -> IronResult<Response> {
    let mut secret_create = Secret::new();
    {
        match req.get::<bodyparser::Struct<SecretCreateReq>>() {
            Ok(Some(body)) => {
                secret_create.set_data(body.data);
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                secret_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                secret_create.set_type_meta(type_meta);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::secret_create(&conn, &secret_create) {
        Ok(secret) => Ok(render_json(status::Ok, &secret)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn secret_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut secret_get = SecretGet::new();
    secret_get.set_id(id.to_string());

    match ServiceAccountDS::secret_show(&conn, &secret_get) {
        Ok(secret) => Ok(render_json(status::Ok, &secret)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn service_account_create(req: &mut Request) -> IronResult<Response> {
    let (org_name, ser_name) = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        let ser_name = params.find("serviceaccount").unwrap().to_owned();

        (org_name, ser_name)
    };

    let mut service_create = ServiceAccount::new();
    {
        match req.get::<bodyparser::Struct<ServiceAccountCreateReq>>() {
            Ok(Some(body)) => {
                let mut obj_ref = ObjectReference::new();
                obj_ref.set_name(body.secrets.name);
                obj_ref.set_kind(body.secrets.kind);
                obj_ref.set_origin(body.secrets.origin);
                obj_ref.set_uid(body.secrets.uid);
                service_create.set_secrets(obj_ref);
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(ser_name);
                object_meta.set_origin(org_name);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                service_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                service_create.set_type_meta(type_meta);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::service_account_create(&conn, &service_create) {
        Ok(service) => Ok(render_json(status::Ok, &service)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn service_account_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::service_account_list(&conn) {
        Ok(service_list) => Ok(render_json(status::Ok, &service_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}


pub fn service_account_show(req: &mut Request) -> IronResult<Response> {
    let (org_name, ser_name) = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        let ser_name = params.find("serviceaccount").unwrap().to_owned();
        (org_name, ser_name)
    };
    let mut serv_get = ServiceAccountGet::new();
    serv_get.set_name(ser_name);
    serv_get.set_origin(org_name);
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::service_account_show(&conn, &serv_get) {
        Ok(origin) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}
