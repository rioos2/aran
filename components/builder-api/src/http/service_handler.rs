use ansi_term::Colour;
use bodyparser;
use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use iron::prelude::*;
use iron::status;
use router::Router;
use db::data_store::Broker;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use deploy::linker_ds::LinkersDS;
use http::deployment_handler;
use http::service_account_handler;
use std::collections::BTreeMap;
use common::ui;
use db;
use protocol::servicesrv::{Services, Spec, RIO_ASM_FAC_ID};
use protocol::asmsrv::{TypeMeta, IdGet, Status, Condition};
const SERVICE: &'static str = "Service";
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServicesReq {
    spec: SpecReq,
    status: deployment_handler::StatusReq,
    object_meta: service_account_handler::ObjectMetaReq,
    type_meta: deployment_handler::TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecReq {
    selector: BTreeMap<String, String>,
    service_type: String,
    loadbalancer_ip: String,
    names: BTreeMap<String, String>,
    external_names: BTreeMap<String, String>,
}
pub fn services_create(req: &mut Request) -> AranResult<Response> {
    let mut services_create = Services::new();
    {
        match req.get::<bodyparser::Struct<ServicesReq>>() {
            Ok(Some(body)) => {
                if body.object_meta.origin.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "origin")));
                }
                let asmid = body.spec
                    .selector
                    .get(&RIO_ASM_FAC_ID.to_string())
                    .to_owned();
                if asmid.unwrap().len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "assembly id")));
                }

                services_create.set_type_meta(TypeMeta::new(SERVICE));
                services_create.set_spec(Spec::new(
                    body.spec.selector.to_owned(),
                    &body.spec.service_type,
                    &body.spec.loadbalancer_ip,
                    body.spec.names,
                    body.spec.external_names,
                ));
                services_create.set_status(Status::with_conditions(
                    &body.status.phase,
                    &body.status.message,
                    &body.status.reason,
                    body.status
                        .conditions
                        .iter()
                        .map(|x| {
                            Condition::with_type(
                                &x.message,
                                &x.reason,
                                &x.status,
                                &x.last_transition_time,
                                &x.last_probe_time,
                                &x.condition_type,
                            )
                        })
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
        format!("======= parsed {:?} ", services_create),
    );

    let conn = Broker::connect().unwrap();

    match LinkersDS::create(&conn, &services_create) {
        Ok(services) => Ok(render_json(status::Ok, &services)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}
pub fn services_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut services_get = IdGet::new();
    services_get.set_id(id.to_string());

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", services_get),
    );

    match LinkersDS::show(&conn, &services_get) {
        Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &services_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}
#[allow(unused_variables)]
pub fn services_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match LinkersDS::list(&conn) {
        Ok(Some(services_list)) => Ok(render_json(status::Ok, &services_list)),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}

pub fn services_list_by_origin(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut services_get = IdGet::new();
    services_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", services_get),
    );
    match LinkersDS::list_by_origin(&conn, &services_get) {
        Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &services_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}
