use ansi_term::Colour;
use bodyparser;
use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
use super::super::*;
use iron::prelude::*;
use iron::status;
use router::Router;
use db::data_store::Broker;
use service::service_account_ds::ServiceAccountDS;
use protocol::servicesrv::{Secret, ObjectReference, ServiceAccount, ObjectMetaData};
use protocol::asmsrv::{TypeMeta, IdGet};
use std::collections::BTreeMap;
use http::deployment_handler;
use common::ui;
use db;
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
const SECRET: &'static str = "Secret";
const SERVICE_ACCOUNT: &'static str = "ServiceAccount";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SecretCreateReq {
    data: BTreeMap<String, String>,
    secret_type: String,
    object_meta: ObjectMetaReq,
    type_meta: deployment_handler::TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServiceAccountCreateReq {
    secrets: ObjectReferenceReq,
    object_meta: ObjectMetaReq,
    type_meta: deployment_handler::TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectMetaReq {
    pub name: String,
    pub origin: String,
    pub uid: String,
    pub created_at: String,
    pub cluster_name: String,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjectReferenceReq {
    kind: String,
    name: String,
    origin: String,
    uid: String,
}

pub fn secret_create(req: &mut Request) -> AranResult<Response> {
    let mut secret_create = Secret::new();
    {
        match req.get::<bodyparser::Struct<SecretCreateReq>>() {
            Ok(Some(body)) => {
                if body.object_meta.origin.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "origin")));
                }

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

                secret_create.set_type_meta(TypeMeta::new(SECRET));


                secret_create.set_secret_type(body.secret_type);
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", secret_create),
    );

    let data = securer::from_config(
        &req.get::<persistent::Read<SecurerBroker>>().unwrap(),
        &Broker::connect().unwrap(),
    )?;

    match data.secure(&secret_create) {
        Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn secret_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(IDMUSTNUMBER)),
        }
    };

    let mut secret_get = IdGet::new();
    secret_get.set_id(id.to_string());

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", secret_get),
    );

    let conn = Broker::connect().unwrap();


    match ServiceAccountDS::secret_show(&conn, &secret_get) {
        Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &secret_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}

#[allow(unused_variables)]
pub fn secret_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();

    let data = securer::from_config(
        &req.get::<persistent::Read<SecurerBroker>>().unwrap(),
        &Broker::connect().unwrap(),
    )?;

    match data.retrieve() {
        Ok(service_list) => Ok(render_json(status::Ok, &service_list)),
        // Ok(None) => {
        //     Err(not_found_error(
        //         &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
        //     ))
        // }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}

pub fn secret_show_by_origin(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut secret_get = IdGet::new();
    secret_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", secret_get),
    );
    match ServiceAccountDS::secret_show_by_origin(&conn, &secret_get) {
        Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &secret_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}


pub fn service_account_create(req: &mut Request) -> AranResult<Response> {
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
                service_create.set_secrets(ObjectReference::new(
                    &body.secrets.name,
                    &body.secrets.kind,
                    &body.secrets.origin,
                    &body.secrets.uid,
                ));
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(ser_name);
                object_meta.set_origin(org_name);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                service_create.set_object_meta(object_meta);
                service_create.set_type_meta(TypeMeta::new(SERVICE_ACCOUNT));

            }

            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", service_create),
    );

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::service_account_create(&conn, &service_create) {
        Ok(Some(service)) => Ok(render_json(status::Ok, &service)),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),

    }
}

#[allow(unused_variables)]
pub fn service_account_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::service_account_list(&conn) {
        Ok(Some(service_list)) => Ok(render_json(status::Ok, &service_list)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn service_account_show(req: &mut Request) -> AranResult<Response> {
    let (org_name, ser_name) = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        let ser_name = params.find("serviceaccount").unwrap().to_owned();
        (org_name, ser_name)
    };
    let mut serv_get = IdGet::new();
    serv_get.set_id(ser_name);
    serv_get.set_name(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", serv_get),
    );
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::service_account_show(&conn, &serv_get) {
        Ok(Some(origin)) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &serv_get.get_id()
            )))
        }
    }
}
