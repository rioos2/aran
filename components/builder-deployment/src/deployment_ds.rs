// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{asmsrv, plansrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use service::service_account_ds::ServiceAccountDS;


pub struct DeploymentDS;

impl DeploymentDS {
    pub fn assembly_create(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)",
            &[
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_origin() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_selector() as Vec<String>),
                &(assembly.get_node() as String),
                &(serde_json::to_string(assembly.get_urls()).unwrap()),
                &(serde_json::to_string(assembly.get_status()).unwrap()),
                &(serde_json::to_string(assembly.get_volumes()).unwrap()),
                &(assembly.get_instance_id() as String),
                &(serde_json::to_string(assembly.get_type_meta()).unwrap()),
                &(serde_json::to_string(assembly.get_object_meta()).unwrap()),
            ],
        ).map_err(Error::AssemblyCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn assembly_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_node() as String),
                &(serde_json::to_string(assembly.get_urls()).unwrap()),
                &(serde_json::to_string(assembly.get_volumes()).unwrap()),
            ],
        ).map_err(Error::AssemblyUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn assembly_show(datastore: &DataStoreConn, get_assembly: &asmsrv::IdGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_assembly_v1($1)",
            &[&(get_assembly.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyGet)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn assembly_list(datastore: &DataStoreConn) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_v1()", &[])
            .map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                assemblys_collection.push(assembly);
            }
            response.set_assemblys(assemblys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn assemblys_show_by_origin(datastore: &DataStoreConn, assemblys_get: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblys_by_origin_v1($1)",
            &[&(assemblys_get.get_id() as String)],
        ).map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                assemblys_collection.push(assembly);
            }
            response.set_assemblys(assemblys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }


    pub fn assembly_status_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_assembly_status_v1($1, $2)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_string(assembly.get_status()).unwrap()),
            ],
        ).map_err(Error::AsmSetStatus)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn assembly_factory_create(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {

        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)",
            &[
                &(assembly_fac.get_name() as String),
                &(assembly_fac.get_uri() as String),
                &(assembly_fac.get_description() as String),
                &(assembly_fac.get_tags() as Vec<String>),
                &(assembly_fac.get_origin() as String),
                &(assembly_fac.get_plan() as String),
                &(serde_json::to_string(assembly_fac.get_properties()).unwrap()),
                &(assembly_fac.get_external_management_resource() as Vec<String>),
                &(serde_json::to_string(assembly_fac.get_component_collection()).unwrap()),
                &(serde_json::to_string(assembly_fac.get_opssettings()).unwrap()),
                &(assembly_fac.get_replicas() as i64),
                &(serde_json::to_string(assembly_fac.get_status()).unwrap()),
                &(serde_json::to_string(assembly_fac.get_object_meta()).unwrap()),
                &(serde_json::to_string(assembly_fac.get_type_meta()).unwrap()),
            ],
        ).map_err(Error::AssemblyFactoryCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly_factory = row_to_assembly_factory(&row)?;
                return Ok(Some(assembly_factory));
            }
        }
        Ok(None)
    }


    pub fn assembly_factory_show(datastore: &DataStoreConn, get_assembly_factory: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblyFactory>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assembly_factory_v1($1)",
            &[&(get_assembly_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyFactoryGet)?;
        if rows.len() > 0 {
            for row in rows {
                let mut assembly_factory = row_to_assembly_factory(&row)?;
                let data = Self::plan_show(&datastore, assembly_factory.get_plan().clone())?;
                assembly_factory.set_plan_data(data);
                return Ok(Some(assembly_factory));
            }
        }
        Ok(None)
    }

    pub fn assembly_factory_status_update(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_assembly_factorys_status_v1($1, $2)",
            &[
                &(assembly_fac.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_string(assembly_fac.get_status()).unwrap()),
            ],
        ).map_err(Error::AsmFactorySetStatus)?;
        if rows.len() > 0 {
            for row in rows {
                let mut assembly_factory = row_to_assembly_factory(&row)?;
                let data = Self::plan_show(&datastore, assembly_factory.get_plan().clone())?;
                assembly_factory.set_plan_data(data);
                return Ok(Some(assembly_factory));
            }
        }
        Ok(None)
    }


    pub fn assembly_factory_list(datastore: &DataStoreConn) -> Result<Option<asmsrv::AssemblyFactoryGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_factory_v1()", &[])
            .map_err(Error::AssemblyFactoryGet)?;

        let mut response = asmsrv::AssemblyFactoryGetResponse::new();

        let mut assembly_factorys_collection = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                assembly_factorys_collection.push(row_to_assembly_factory(&row)?)
            }
            response.set_assemblys_factory(assembly_factorys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn assemblyfactorys_show_by_origin(datastore: &DataStoreConn, assemblyfactory_get: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblyFactoryGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblyfactorys_by_origin_v1($1)",
            &[&(assemblyfactory_get.get_id() as String)],
        ).map_err(Error::AssemblyFactoryGet)?;

        let mut response = asmsrv::AssemblyFactoryGetResponse::new();

        let mut assemblyfac_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                assemblyfac_collection.push(row_to_assembly_factory(&row)?)
            }

            response.set_assemblys_factory(assemblyfac_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn assembly_factorys_describe(datastore: &DataStoreConn, assemblydes_get: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblys_by_parentid_v1($1)",
            &[&(assemblydes_get.get_id() as String)],
        ).map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                assemblys_collection.push(assembly);
            }
            response.set_assemblys(assemblys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }


    pub fn collect_spec(row: &postgres::rows::Row, datastore: &DataStoreConn) -> Result<asmsrv::Assembly> {
        let mut assembly = row_to_assembly(&row)?;
        let mut asm_fac_get = asmsrv::IdGet::new();
        asm_fac_get.set_id(assembly.get_parent_id());
        let data = Self::assembly_factory_show(&datastore, &asm_fac_get)?;
        let mut endpoint_get = asmsrv::IdGet::new();
        endpoint_get.set_id(assembly.get_id());
        let endpoints = ServiceAccountDS::endpoints_show_by_asm_id(&datastore, &endpoint_get)
            .map_err(Error::EndPoints)?;
        assembly.set_spec(data);
        assembly.set_endpoints(endpoints);
        Ok(assembly)
    }

    pub fn plan_create(datastore: &DataStoreConn, plan: &plansrv::Plan) -> Result<Option<plansrv::Plan>> {
        let conn = datastore.pool.get_shard(0)?;
        let data: Vec<String> = plan.get_services()
            .into_iter()
            .map(|plan| {
                let d = serde_json::to_string(plan).unwrap();
                d
            })
            .collect();
        let rows = &conn.query(
            "SELECT * FROM insert_plan_factory_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(plan.get_group_name() as String),
                &(plan.get_description() as String),
                &(plan.get_tags() as Vec<String>),
                &(plan.get_url() as String),
                &(plan.get_origin() as String),
                &(plan.get_artifacts() as Vec<String>),
                &(data as Vec<String>),
            ],
        ).map_err(Error::PlanCreate)?;
        if rows.len() > 0 {
            let plan = row_to_plan(&rows.get(0))?;
            return Ok(Some(plan));
        }
        Ok(None)

    }

    pub fn plan_show(datastore: &DataStoreConn, plan_url: String) -> Result<Option<plansrv::Plan>> {
        let url = plan_url.to_string();
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_plan_v1($1)", &[&url])
            .map_err(Error::PlanGet)?;
        if rows.len() > 0 {
            for row in rows {
                let plan = row_to_plan(&row)?;
                return Ok(Some(plan));
            }
        }
        Ok(None)
    }

    pub fn plan_list(datastore: &DataStoreConn) -> Result<Option<plansrv::PlanGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_plans_v1()", &[]).map_err(
            Error::PlanGetResponse,
        )?;

        let mut response = plansrv::PlanGetResponse::new();

        let mut plan_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                plan_collection.push(row_to_plan(&row)?)
            }
            response.set_plan_collection(plan_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_assembly(row: &postgres::rows::Row) -> Result<asmsrv::Assembly> {
    let mut assembly = asmsrv::Assembly::new();

    let id: i64 = row.get("id");
    let origin: i64 = row.get("origin_id");
    let urls: String = row.get("urls");
    let status: String = row.get("status");
    let node: String = row.get("node");
    let volume: String = row.get("volumes");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    assembly.set_id(id.to_string());
    assembly.set_name(row.get("name"));
    assembly.set_urls(serde_json::from_str(&urls).unwrap());
    assembly.set_uri(row.get("uri"));
    assembly.set_tags(row.get("tags"));
    assembly.set_selector(row.get("selector"));
    let mut obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    obj.set_name(id.to_string());
    assembly.set_object_meta(obj);
    assembly.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    assembly.set_description(row.get("description"));
    assembly.set_parent_id(row.get("parent_id"));
    assembly.set_origin(origin.to_string());
    assembly.set_status(serde_json::from_str(&status).unwrap());
    assembly.set_volumes(serde_json::from_str(&volume).unwrap());
    assembly.set_node(node as String);
    assembly.set_created_at(created_at.to_rfc3339());

    Ok(assembly)
}

fn row_to_assembly_factory(row: &postgres::rows::Row) -> Result<asmsrv::AssemblyFactory> {

    let mut assembly_factory = asmsrv::AssemblyFactory::new();

    let id: i64 = row.get("id");
    let origin: i64 = row.get("origin_id");
    let properties: String = row.get("properties");
    let component_collection: String = row.get("component_collection");
    let opssettings: String = row.get("opssettings");
    let status: String = row.get("status");
    let replicas: i64 = row.get("replicas");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");


    assembly_factory.set_id(id.to_string());
    assembly_factory.set_name(row.get("name"));
    assembly_factory.set_uri(row.get("uri"));
    assembly_factory.set_description(row.get("description"));
    assembly_factory.set_tags(row.get("tags"));
    assembly_factory.set_origin(origin.to_string());
    assembly_factory.set_external_management_resource(row.get("external_management_resource"));
    assembly_factory.set_created_at(created_at.to_rfc3339());
    assembly_factory.set_component_collection(serde_json::from_str(&component_collection).unwrap());
    assembly_factory.set_opssettings(serde_json::from_str(&opssettings).unwrap());
    assembly_factory.set_status(serde_json::from_str(&status).unwrap());
    assembly_factory.set_plan(row.get("plan"));
    assembly_factory.set_replicas(replicas as u32);
    assembly_factory.set_properties(serde_json::from_str(&properties).unwrap());

    let mut obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    obj.set_name(id.to_string());
    assembly_factory.set_object_meta(obj);
    assembly_factory.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    Ok(assembly_factory)
}

fn row_to_plan(row: &postgres::rows::Row) -> Result<plansrv::Plan> {
    let mut plan = plansrv::Plan::new();
    let id: i64 = row.get("id");
    let services: Vec<String> = row.get("services");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    plan.set_id(id.to_string() as String);
    plan.set_group_name(row.get("group_name"));
    plan.set_url(row.get("url"));
    plan.set_description(row.get("description"));
    plan.set_tags(row.get("tags"));
    plan.set_origin(row.get("origin"));
    plan.set_artifacts(row.get("artifacts"));
    let mut service_collection = Vec::new();
    for data in services {
        let object_service: plansrv::Service = serde_json::from_str(&data).unwrap();
        service_collection.push(object_service);
    }
    plan.set_services(service_collection);
    plan.set_created_at(created_at.to_rfc3339());

    Ok(plan)
}
