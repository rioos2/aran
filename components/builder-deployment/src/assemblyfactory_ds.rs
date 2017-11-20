// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::asmsrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use planfactory_ds::PlanFactoryDS;


pub struct AssemblyFactoryDS;

impl AssemblyFactoryDS {
    ////////// AF STARTS
    pub fn create(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {

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


    pub fn show(datastore: &DataStoreConn, get_assembly_factory: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblyFactory>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assembly_factory_v1($1)",
            &[&(get_assembly_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyFactoryGet)?;
        if rows.len() > 0 {
            for row in rows {
                let mut assembly_factory = row_to_assembly_factory(&row)?;
                let data = PlanFactoryDS::show(&datastore, assembly_factory.get_plan().clone())?;
                assembly_factory.set_plan_data(data.unwrap());
                return Ok(Some(assembly_factory));
            }
        }
        Ok(None)
    }

    pub fn status_update(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {
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
                let data = PlanFactoryDS::show(&datastore, assembly_factory.get_plan().clone())?;
                assembly_factory.set_plan_data(data.unwrap());
                return Ok(Some(assembly_factory));
            }
        }
        Ok(None)
    }


    pub fn list(datastore: &DataStoreConn) -> Result<Option<asmsrv::AssemblyFactoryGetResponse>> {
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
    pub fn show_by_origin(datastore: &DataStoreConn, assemblyfactory_get: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblyFactoryGetResponse>> {
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
    ////////// AF ENDS
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
