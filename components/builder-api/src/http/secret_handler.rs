use bodyparser;
use rio_net::http::controller::*;
use secret::secret_ds::SecretDS;
use iron::prelude::*;
use iron::status;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use protocol::secretsrv::Secret;
use protocol::asmsrv::{ObjectMeta, OwnerReferences, TypeMeta};
use std::collections::BTreeMap;


#[derive(Clone, Debug, Serialize, Deserialize)]
struct SecretCreateReq {
    data: BTreeMap<String, String>,
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
    namespace: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
    owner_references: Vec<OwnerReferencesReq>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct OwnerReferencesReq {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}

pub fn secret_create(req: &mut Request) -> IronResult<Response> {
    let mut secret_create = Secret::new();
    {
        match req.get::<bodyparser::Struct<SecretCreateReq>>() {
            Ok(Some(body)) => {
                secret_create.set_data(body.data);
                let mut object_meta = ObjectMeta::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_namespace(body.object_meta.namespace);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                let mut owner_references_collection = Vec::new();
                for data in body.object_meta.owner_references {
                    let mut owner_references = OwnerReferences::new();
                    owner_references.set_kind(data.kind);
                    owner_references.set_api_version(data.api_version);
                    owner_references.set_name(data.name);
                    owner_references.set_uid(data.uid);
                    owner_references.set_block_owner_deletion(data.block_owner_deletion);
                    owner_references_collection.push(owner_references);
                }
                object_meta.set_owner_references(owner_references_collection);
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

    match SecretDS::secret_create(&conn, &secret_create) {
        Ok(secret) => Ok(render_json(status::Ok, &secret)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
