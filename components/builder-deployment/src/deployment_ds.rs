// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{asmsrv, plansrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use std::collections::BTreeMap;

pub struct DeploymentDS;

impl DeploymentDS {
    pub fn assembly_create(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assemby_create ");

        let status_str = serde_json::to_string(assembly.get_status()).unwrap();
        let type_meta = serde_json::to_string(assembly.get_type_meta()).unwrap();
        let object_meta = serde_json::to_string(assembly.get_object_meta()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(object_meta as String),
                &(type_meta as String),
                &(assembly.get_node() as String),
                &(assembly.get_ip() as String),
                &(assembly.get_urls() as String),
                &(status_str as String),
            ],
        ).map_err(Error::AssemblyCreate)?;

        debug!(">● ROWS: assemby_create =>\n{:?}", &rows);
        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assemby_create ");
        let asm_id = assembly.get_id().parse::<i64>().unwrap();
        let type_meta = serde_json::to_string(assembly.get_type_meta()).unwrap();
        let object_meta = serde_json::to_string(assembly.get_object_meta()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM update_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[
                &asm_id,
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(object_meta as String),
                &(type_meta as String),
                &(assembly.get_node() as String),
                &(assembly.get_ip() as String),
                &(assembly.get_urls() as String),
            ],
        ).map_err(Error::AssemblyUpdate)?;

        debug!(">● ROWS: assemby_create =>\n{:?}", &rows);
        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_show(datastore: &DataStoreConn, get_assembly: &asmsrv::IdGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assemby_show {:?}", get_assembly.get_id());
        let asm_id = get_assembly.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_assembly_v1($1)", &[&asm_id])
            .map_err(Error::AssemblyGet)?;

        debug!(">● ROWS: assemby_show =>\n{:?}", &rows);

        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_list(datastore: &DataStoreConn) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_v1()", &[])
            .map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();

        debug!(">● ROWS: assemby_list =>\n{:?}", &rows);
        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            assemblys_collection.push(assembly);
        }
        response.set_assemblys(
            assemblys_collection,
            "AssemblyList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn assembly_status_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let asm_id = assembly.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(assembly.get_status()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM set_assembly_status_v1($1, $2)",
            &[&asm_id, &(status_str as String)],
        ).map_err(Error::AsmSetStatus)?;
        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_factory_create(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {
        let status_str = serde_json::to_string(assembly_fac.get_status()).unwrap();
        let properties = serde_json::to_string(assembly_fac.get_properties()).unwrap();
        let type_meta = serde_json::to_string(assembly_fac.get_type_meta()).unwrap();
        let object_meta = serde_json::to_string(assembly_fac.get_object_meta()).unwrap();
        let component_collection = serde_json::to_string(assembly_fac.get_component_collection()).unwrap();
        let opssettings = serde_json::to_string(assembly_fac.get_opssettings()).unwrap();

        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assembly_factory_create ");

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
            &[
                &(assembly_fac.get_name() as String),
                &(assembly_fac.get_uri() as String),
                &(assembly_fac.get_description() as String),
                &(assembly_fac.get_tags() as Vec<String>),
                &(assembly_fac.get_plan() as String),
                &(properties as String),
                &(type_meta as String),
                &(object_meta as String),
                &(assembly_fac.get_external_management_resource() as Vec<String>),
                &(component_collection as String),
                &(opssettings as String),
                &(assembly_fac.get_replicas() as i64),
                &(status_str as String),
            ],
        ).map_err(Error::AssemblyFactoryCreate)?;

        debug!(">● ROWS: assembly_factory_create =>\n{:?}", &rows);
        let assembly_factory = row_to_assembly_factory(&rows.get(0))?;
        debug!("◖☩ DONE: assembly_factory_create ");
        return Ok(Some(assembly_factory.clone()));
    }

    pub fn assembly_factory_show(datastore: &DataStoreConn, get_assembly_factory: &asmsrv::AssemblyFactoryGet) -> Result<Option<asmsrv::AssemblyFactory>> {
        let conn = datastore.pool.get_shard(0)?;
        let asm_id = get_assembly_factory.get_id().parse::<i64>().unwrap();

        debug!(
            "◖☩ START: assemby_factory_show {:?}",
            get_assembly_factory.get_id()
        );

        let rows = &conn.query("SELECT * FROM get_assembly_factory_v1($1)", &[&asm_id])
            .map_err(Error::AssemblyFactoryGet)?;

        debug!(">● ROWS: assemby_factory_show =>\n{:?}", &rows);

        for row in rows {
            let mut assembly_factory = row_to_assembly_factory(&row)?;
            let plan_url = assembly_factory.get_plan();
            let data = Self::plan_show(&datastore, plan_url.clone())?;
            assembly_factory.set_plan_data(data);
            return Ok(Some(assembly_factory));
        }

        Ok(None)
    }

    pub fn assembly_factory_status_update(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {
        let conn = datastore.pool.get_shard(0)?;
        let asm_fac_id = assembly_fac.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(assembly_fac.get_status()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM set_assembly_factorys_status_v1($1, $2)",
            &[&asm_fac_id, &(status_str as String)],
        ).map_err(Error::AsmFactorySetStatus)?;
        for row in rows {
            let mut assembly_factory = row_to_assembly_factory(&row)?;
            let plan_url = assembly_factory.get_plan();
            let data = Self::plan_show(&datastore, plan_url.clone())?;
            assembly_factory.set_plan_data(data);
            return Ok(Some(assembly_factory));
        }
        Ok(None)
    }


    pub fn assembly_factory_list(datastore: &DataStoreConn) -> Result<Option<asmsrv::AssemblyFactoryGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_factory_v1()", &[])
            .map_err(Error::AssemblyFactoryGet)?;

        let mut response = asmsrv::AssemblyFactoryGetResponse::new();

        let mut assembly_factorys_collection = Vec::new();

        debug!(">● ROWS: assembly_factory_list =>\n{:?}", &rows);
        for row in rows {
            assembly_factorys_collection.push(row_to_assembly_factory(&row)?)
        }
        response.set_assemblys_factory(
            assembly_factorys_collection,
            "AssemblyFactoryList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn collect_spec(row: &postgres::rows::Row, datastore: &DataStoreConn) -> Result<asmsrv::Assembly> {
        let mut assembly = row_to_assembly(&row)?;
        let mut asm_fac_get = asmsrv::AssemblyFactoryGet::new();
        asm_fac_get.set_id(assembly.get_parent_id());
        let data = Self::assembly_factory_show(&datastore, &asm_fac_get)?;
        assembly.set_spec(data);
        Ok(assembly)
    }

    pub fn plan_show(datastore: &DataStoreConn, plan_url: String) -> Result<Option<plansrv::Plan>> {
        let url = plan_url.to_string();
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: plan_show {:?}", plan_url);

        let rows = &conn.query("SELECT * FROM get_plan_v1($1)", &[&url])
            .map_err(Error::PlanGet)?;

        debug!(">● ROWS: plan_show =>\n{:?}", &rows);

        for row in rows {
            let plan = row_to_plan(&row)?;
            return Ok(Some(plan));
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
        for row in rows {
            plan_collection.push(row_to_plan(&row)?)
        }
        response.set_plan_collection(plan_collection, "PlanList".to_string(), "v1".to_string());
        Ok(Some(response))
    }
}

fn row_to_assembly(row: &postgres::rows::Row) -> Result<asmsrv::Assembly> {
    let mut assembly = asmsrv::Assembly::new();
    debug!("◖☩ START: row_to_assemby");

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let urls: String = row.get("urls");
    let uri: String = row.get("uri");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");
    let parent_id: String = row.get("parent_id");
    let status: String = row.get("status");
    let node: String = row.get("node");
    let ip: String = row.get("ip");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    assembly.set_id(id.to_string() as String);
    assembly.set_name(name as String);
    assembly.set_urls(urls as String);
    assembly.set_uri(uri as String);
    assembly.set_tags(tags as Vec<String>);
    let mut object_meta_obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    object_meta_obj.set_name(id.to_string() as String);
    assembly.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    assembly.set_type_meta(type_meta_obj);
    assembly.set_description(description as String);
    assembly.set_parent_id(parent_id as String);
    let status_obj: asmsrv::Status = serde_json::from_str(&status).unwrap();
    assembly.set_status(status_obj);
    assembly.set_node(node as String);
    assembly.set_ip(ip as String);
    assembly.set_created_at(created_at.to_rfc3339());

    debug!("◖☩ ASM: row_to_assemby =>\n{:?}", assembly);
    debug!("◖☩ DONE: row_to_assemby");
    Ok(assembly)
}


fn row_to_assembly_factory(row: &postgres::rows::Row) -> Result<asmsrv::AssemblyFactory> {
    let mut assembly_factory = asmsrv::AssemblyFactory::new();
    debug!("◖☩ START: row_to_assemby_factory");
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let uri: String = row.get("uri");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let plan: String = row.get("plan");
    let properties: String = row.get("properties");
    let type_meta: String = row.get("type_meta");
    let object_meta: String = row.get("object_meta");
    let external_management_resource: Vec<String> = row.get("external_management_resource");
    let component_collection: String = row.get("component_collection");
    let opssettings: String = row.get("opssettings");
    let status: String = row.get("status");
    let replicas: i64 = row.get("replicas");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    assembly_factory.set_id(id.to_string() as String);
    assembly_factory.set_name(name as String);
    assembly_factory.set_uri(uri as String);
    assembly_factory.set_description(description as String);
    assembly_factory.set_tags(tags as Vec<String>);
    assembly_factory.set_external_management_resource(external_management_resource as Vec<String>);
    assembly_factory.set_created_at(created_at.to_rfc3339());
    let com_obj: BTreeMap<String, String> = serde_json::from_str(&component_collection).unwrap();
    assembly_factory.set_component_collection(com_obj);
    let opssettings_obj: asmsrv::OpsSettings = serde_json::from_str(&opssettings).unwrap();
    assembly_factory.set_opssettings(opssettings_obj);
    let status_obj: asmsrv::Status = serde_json::from_str(&status).unwrap();
    assembly_factory.set_status(status_obj);
    assembly_factory.set_plan(plan as String);
    assembly_factory.set_replicas(replicas as u64);
    let mut object_meta_obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    object_meta_obj.set_name(id.to_string() as String);
    assembly_factory.set_object_meta(object_meta_obj);
    let properties_obj: asmsrv::Properties = serde_json::from_str(&properties).unwrap();
    assembly_factory.set_properties(properties_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    assembly_factory.set_type_meta(type_meta_obj);

    debug!(
        "◖☩ ASM: row_to_assemby_factory =>\n{:?}",
        assembly_factory
    );
    debug!("◖☩ DONE: row_to_assemby_factory");
    Ok(assembly_factory)
}


fn row_to_plan(row: &postgres::rows::Row) -> Result<plansrv::Plan> {
    let mut plan = plansrv::Plan::new();
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let url: String = row.get("url");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let camp_version: String = row.get("camp_version");
    let origin: String = row.get("origin");
    let artifacts: Vec<String> = row.get("artifacts");
    let services: Vec<String> = row.get("services");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    plan.set_id(id.to_string() as String);
    plan.set_name(name as String);
    plan.set_url(url as String);
    plan.set_description(description as String);
    plan.set_tags(tags as Vec<String>);
    plan.set_camp_version(camp_version as String);
    plan.set_origin(origin as String);
    plan.set_artifacts(artifacts as Vec<String>);
    let mut service_collection = Vec::new();
    for data in services {
        let object_service: plansrv::Service = serde_json::from_str(&data).unwrap();
        service_collection.push(object_service);
    }
    plan.set_services(service_collection);
    plan.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ PLAN: row_to_plan =>\n{:?}", plan);
    debug!("◖☩ DONE: row_to_assemby_factory");
    Ok(plan)
}
