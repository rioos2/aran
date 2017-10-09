// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{asmsrv, plansrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct DeploymentDS;

impl DeploymentDS {
    pub fn assembly_create(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_node() as String),
                &(serde_json::to_string(assembly.get_ip()).unwrap()),
                &(serde_json::to_string(assembly.get_urls()).unwrap()),
                &(serde_json::to_string(assembly.get_status()).unwrap()),
                &(serde_json::to_string(assembly.get_volumes()).unwrap()),
            ],
        ).map_err(Error::AssemblyCreate)?;

        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_node() as String),
                &(serde_json::to_string(assembly.get_ip()).unwrap()),
                &(serde_json::to_string(assembly.get_urls()).unwrap()),
                &(serde_json::to_string(assembly.get_volumes()).unwrap()),
            ],
        ).map_err(Error::AssemblyUpdate)?;


        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_show(datastore: &DataStoreConn, get_assembly: &asmsrv::IdGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assembly_v1($1)",
            &[&(get_assembly.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyGet)?;

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
        let rows = &conn.query(
            "SELECT * FROM set_assembly_status_v1($1, $2)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_string(assembly.get_status()).unwrap()),
            ],
        ).map_err(Error::AsmSetStatus)?;
        for row in rows {
            let assembly = Self::collect_spec(&row, &datastore)?;
            return Ok(Some(assembly));
        }
        Ok(None)
    }

    pub fn assembly_factory_create(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {

        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[
                &(assembly_fac.get_name() as String),
                &(assembly_fac.get_uri() as String),
                &(assembly_fac.get_description() as String),
                &(assembly_fac.get_tags() as Vec<String>),
                &(assembly_fac.get_plan() as String),
                &(serde_json::to_string(assembly_fac.get_properties()).unwrap()),
                &(assembly_fac.get_external_management_resource() as Vec<String>),
                &(serde_json::to_string(assembly_fac.get_component_collection()).unwrap()),
                &(serde_json::to_string(assembly_fac.get_opssettings()).unwrap()),
                &(assembly_fac.get_replicas() as i64),
                &(serde_json::to_string(assembly_fac.get_status()).unwrap()),
            ],
        ).map_err(Error::AssemblyFactoryCreate)?;


        let assembly_factory = row_to_assembly_factory(&rows.get(0))?;

        return Ok(Some(assembly_factory.clone()));
    }

    pub fn assembly_factory_show(datastore: &DataStoreConn, get_assembly_factory: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblyFactory>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assembly_factory_v1($1)",
            &[&(get_assembly_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyFactoryGet)?;

        for row in rows {
            let mut assembly_factory = row_to_assembly_factory(&row)?;
            let data = Self::plan_show(&datastore, assembly_factory.get_plan().clone())?;
            assembly_factory.set_plan_data(data);
            return Ok(Some(assembly_factory));
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
        for row in rows {
            let mut assembly_factory = row_to_assembly_factory(&row)?;
            let data = Self::plan_show(&datastore, assembly_factory.get_plan().clone())?;
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
        let mut asm_fac_get = asmsrv::IdGet::new();
        asm_fac_get.set_id(assembly.get_parent_id());
        let data = Self::assembly_factory_show(&datastore, &asm_fac_get)?;
        assembly.set_spec(data);
        Ok(assembly)
    }

    pub fn plan_show(datastore: &DataStoreConn, plan_url: String) -> Result<Option<plansrv::Plan>> {
        let url = plan_url.to_string();
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_plan_v1($1)", &[&url])
            .map_err(Error::PlanGet)?;

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

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let urls: String = row.get("urls");
    let uri: String = row.get("uri");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let parent_id: String = row.get("parent_id");
    let status: String = row.get("status");
    let node: String = row.get("node");
    let ip: String = row.get("ip");
    let volume: String = row.get("volumes");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    assembly.set_id(id.to_string());
    assembly.set_name(name as String);
    assembly.set_urls(serde_json::from_str(&urls).unwrap());
    assembly.set_uri(uri as String);
    assembly.set_tags(tags as Vec<String>);

    let mut obj_meta = asmsrv::ObjectMeta::new();
    let mut owner_collection = Vec::new();
    let owner = asmsrv::OwnerReferences::new();
    owner_collection.push(owner);
    obj_meta.set_name(id.to_string());
    obj_meta.set_owner_references(owner_collection);
    assembly.set_object_meta(obj_meta);
    let mut type_meta = asmsrv::TypeMeta::new();
    type_meta.set_kind("Assembly".to_string());
    type_meta.set_api_version("v1".to_string());
    assembly.set_type_meta(type_meta);

    assembly.set_description(description as String);
    assembly.set_parent_id(parent_id as String);
    assembly.set_status(serde_json::from_str(&status).unwrap());
    assembly.set_volumes(serde_json::from_str(&volume).unwrap());
    assembly.set_node(node as String);
    assembly.set_ip(serde_json::from_str(&ip).unwrap());
    assembly.set_created_at(created_at.to_rfc3339());

    Ok(assembly)
}


fn row_to_assembly_factory(row: &postgres::rows::Row) -> Result<asmsrv::AssemblyFactory> {

    let mut assembly_factory = asmsrv::AssemblyFactory::new();

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let uri: String = row.get("uri");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let plan: String = row.get("plan");
    let properties: String = row.get("properties");
    let external_management_resource: Vec<String> = row.get("external_management_resource");
    let component_collection: String = row.get("component_collection");
    let opssettings: String = row.get("opssettings");
    let status: String = row.get("status");
    let replicas: i64 = row.get("replicas");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    assembly_factory.set_id(id.to_string());
    assembly_factory.set_name(name as String);
    assembly_factory.set_uri(uri as String);
    assembly_factory.set_description(description as String);
    assembly_factory.set_tags(tags as Vec<String>);
    assembly_factory.set_external_management_resource(external_management_resource as Vec<String>);
    assembly_factory.set_created_at(created_at.to_rfc3339());
    assembly_factory.set_component_collection(serde_json::from_str(&component_collection).unwrap());
    assembly_factory.set_opssettings(serde_json::from_str(&opssettings).unwrap());
    assembly_factory.set_status(serde_json::from_str(&status).unwrap());
    assembly_factory.set_plan(plan as String);
    assembly_factory.set_replicas(replicas as u64);
    assembly_factory.set_properties(serde_json::from_str(&properties).unwrap());

    let mut obj_meta = asmsrv::ObjectMeta::new();
    let mut owner_collection = Vec::new();
    let owner = asmsrv::OwnerReferences::new();
    owner_collection.push(owner);
    obj_meta.set_name(id.to_string());
    obj_meta.set_owner_references(owner_collection);
    assembly_factory.set_object_meta(obj_meta);
    let mut type_meta = asmsrv::TypeMeta::new();
    type_meta.set_kind("AssemblyFactory".to_string());
    type_meta.set_api_version("v1".to_string());
    assembly_factory.set_type_meta(type_meta);

    Ok(assembly_factory)
}


fn row_to_plan(row: &postgres::rows::Row) -> Result<plansrv::Plan> {
    let mut plan = plansrv::Plan::new();
    let id: i64 = row.get("id");
    let name: String = row.get("group_name");
    let url: String = row.get("url");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let origin: String = row.get("origin");
    let artifacts: Vec<String> = row.get("artifacts");
    let services: Vec<String> = row.get("services");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    plan.set_id(id.to_string() as String);
    plan.set_name(name as String);
    plan.set_url(url as String);
    plan.set_description(description as String);
    plan.set_tags(tags as Vec<String>);
    plan.set_origin(origin as String);
    plan.set_artifacts(artifacts as Vec<String>);
    let mut service_collection = Vec::new();
    for data in services {
        let object_service: plansrv::Service = serde_json::from_str(&data).unwrap();
        service_collection.push(object_service);
    }
    plan.set_services(service_collection);
    plan.set_created_at(created_at.to_rfc3339());

    Ok(plan)
}
