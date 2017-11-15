use ansi_term::Colour;
use bodyparser;
use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use iron::prelude::*;
use iron::status;
use router::Router;
use db::data_store::Broker;
use http::deployment_handler;
use common::ui;
use db;
use http::service_account_handler;
use service::service_account_ds::ServiceAccountDS;
use protocol::servicesrv::{EndPoints, Subsets, Addesses, Ports};
use protocol::asmsrv::{TypeMeta, IdGet};
const ENDPOINT: &'static str = "Endpoint";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EndPointsReq {
    target_ref: String,
    subsets: SubsetsReq,
    object_meta: service_account_handler::ObjectMetaReq,
    type_meta: deployment_handler::TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SubsetsReq {
    addresses: Vec<AddessesReq>,
    unready_addresses: Vec<AddessesReq>,
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



pub fn endpoints_create(req: &mut Request) -> AranResult<Response> {
    let mut endpoints_create = EndPoints::new();
    {
        match req.get::<bodyparser::Struct<EndPointsReq>>() {
            Ok(Some(body)) => {
                if body.object_meta.origin.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "origin")));
                }
                if body.target_ref.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "target_ref")));
                }

                endpoints_create.set_type_meta(TypeMeta::new(ENDPOINT));

                endpoints_create.set_target_ref(body.target_ref);

                endpoints_create.set_subsets(Subsets::new(
                    body.subsets
                        .addresses
                        .iter()
                        .map(|x| Addesses::new(&x.name, &x.protocol_version, &x.ip))
                        .collect::<Vec<_>>(),
                    body.subsets
                        .unready_addresses
                        .iter()
                        .map(|x| Addesses::new(&x.name, &x.protocol_version, &x.ip))
                        .collect::<Vec<_>>(),
                    body.subsets
                        .ports
                        .iter()
                        .map(|x| Ports::new(&x.name, &x.port, &x.protocol))
                        .collect::<Vec<_>>(),
                ));
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
        format!("======= parsed {:?} ", endpoints_create),
    );

    let conn = Broker::connect().unwrap();

    match ServiceAccountDS::endpoints_create(&conn, &endpoints_create) {
        Ok(Some(endpoints)) => Ok(render_json(status::Ok, &endpoints)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}
#[allow(unused_variables)]
pub fn endpoints_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match ServiceAccountDS::endpoints_list(&conn) {
        Ok(Some(endpoints_list)) => Ok(render_json(status::Ok, &endpoints_list)),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}

pub fn endpoints_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
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
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &endpoints_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}
pub fn endpoints_list_by_origin(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut endpoints_get = IdGet::new();
    endpoints_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", endpoints_get),
    );
    match ServiceAccountDS::endpoints_list_by_origin(&conn, &endpoints_get) {
        Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &endpoints_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}
pub fn show_by_assembly(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("asmid").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut endpoints_get = IdGet::new();
    endpoints_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", endpoints_get),
    );
    match ServiceAccountDS::show_by_assembly(&conn, &endpoints_get) {
        Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}
