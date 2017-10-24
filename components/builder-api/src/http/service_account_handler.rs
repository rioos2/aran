use ansi_term::Colour;
use bodyparser;
use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, DBError};

use service::service_account_ds::ServiceAccountDS;
use iron::prelude::*;
use iron::status;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use protocol::servicesrv::{Secret, ObjectReference, ServiceAccount, ObjectMetaData, EndPoints,Subsets, Addesses,Ports};
use protocol::asmsrv::{TypeMeta, IdGet};
use std::collections::BTreeMap;
use http::deployment_handler;
use common::ui;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EndPointsReq {
    subsets: SubsetsReq,
    object_meta: ObjectMetaReq,
    type_meta: deployment_handler::TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SubsetsReq {
    target_ref: String,
    addresses: Vec<AddessesReq>,
    not_ready_addresses: Vec<AddessesReq>,
    ports: Vec<PortsReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AddessesReq {
    name: String,
    protocol_version: String,
    ip: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PortsReq {
    name: String,
    port: String,
    protocol: String,
}

pub fn secret_create(req: &mut Request) -> AranResult<Response> {
    let mut secret_create = Secret::new();
    {
        match req.get::<bodyparser::Struct<SecretCreateReq>>() {
            Ok(Some(body)) => {
                if body.object_meta.origin.len() <= 0 {
                    return Err(bad_request("Missing value for field: `origin`"));
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

                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                secret_create.set_type_meta(type_meta);

                secret_create.set_secret_type(body.secret_type);
            }
            Err(err) => {
                return Err(malformed_body(&err.detail));
            }
            _ => return Err(malformed_body(&"nothing found in body")),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", secret_create),
    );

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::secret_create(&conn, &secret_create) {
        Ok(secret) => Ok(render_json(status::Ok, &secret)),
        Err(err) => Err(internal_error(&format!("{}", err), &"errred.")),

    }
}

pub fn secret_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request("id must be a number")),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut secret_get = IdGet::new();
    secret_get.set_id(id.to_string());

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", secret_get),
    );

    match ServiceAccountDS::secret_show(&conn, &secret_get) {
        Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
        Ok(None) => {
            let err = "NotFound";
            Ok(render_net_error(
                &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
            ))
        }
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

#[allow(unused_variables)]
pub fn secret_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::secret_list(&conn) {
        Ok(Some(service_list)) => Ok(render_json(status::Ok, &service_list)),
        Ok(None) => {
            let err = "NotFound";
            Ok(render_net_error(
                &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
            ))
        }
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn secret_show_by_origin(req: &mut Request) -> IronResult<Response> {
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
            let err = "NotFound";
            Ok(render_net_error(
                &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
            ))
        }
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
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", service_create),
    );

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::service_account_create(&conn, &service_create) {
        Ok(service) => Ok(render_json(status::Ok, &service)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

#[allow(unused_variables)]
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
        Ok(origin) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn endpoints_create(req: &mut Request) -> IronResult<Response> {
    let mut endpoints_create = EndPoints::new();
    {
        match req.get::<bodyparser::Struct<EndPointsReq>>() {
            Ok(Some(body)) => {
                if body.object_meta.origin.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `origin`",
                    )));

                }


                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                endpoints_create.set_object_meta(object_meta);

                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                endpoints_create.set_type_meta(type_meta);

                let mut subsets = Subsets::new();
                subsets.set_target_ref(body.subsets.target_ref);

                let mut address_collection = Vec::new();
                for address in body.subsets.addresses {
                    let mut addesses = Addesses::new();
                    addesses.set_name(address.name);
                    addesses.set_protocol_version(address.protocol_version);
                    addesses.set_ip(address.ip);
                    address_collection.push(addesses);
                }
                subsets.set_addresses(address_collection);

                let mut not_ready_address_collection = Vec::new();
                for nr_address in body.subsets.not_ready_addresses {
                    let mut addesses = Addesses::new();
                    addesses.set_name(nr_address.name);
                    addesses.set_protocol_version(nr_address.protocol_version);
                    addesses.set_ip(nr_address.ip);
                    not_ready_address_collection.push(addesses);
                }
                subsets.set_not_ready_addresses(not_ready_address_collection);

                let mut ports_collection = Vec::new();
                for port in body.subsets.ports {
                    let mut ports = Ports::new();
                    ports.set_name(port.name);
                    ports.set_port(port.port);
                    ports.set_protocol(port.protocol);
                    ports_collection.push(ports);
                }
                subsets.set_ports(ports_collection);
                endpoints_create.set_subsets(subsets);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", endpoints_create),
    );

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::endpoints_create(&conn, &endpoints_create) {
        Ok(endpoints) => Ok(render_json(status::Ok, &endpoints)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn endpoints_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::endpoints_list(&conn) {
        Ok(Some(endpoints_list)) => Ok(render_json(status::Ok, &endpoints_list)),
        Ok(None) => {
            let err = "NotFound";
            Ok(render_net_error(
                &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
            ))
        }
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn endpoints_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut endpoints_get = IdGet::new();
    endpoints_get.set_id(id.to_string());

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", endpoints_get),
    );

    match ServiceAccountDS::endpoints_show(&conn, &endpoints_get) {
        Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
        Ok(None) => {
            let err = "NotFound";
            Ok(render_net_error(
                &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
            ))
        }
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}
