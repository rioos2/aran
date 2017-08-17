// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protocol::asmsrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;


pub struct DeploymentDS;

impl DeploymentDS {
    /// Create an assembly in the  database. If the assembly is created, we'll
    /// return the Assembly result.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the assembly cannot be created
    pub fn assembly_create(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assemby_create ");

        let status_str = serde_json::to_string(assembly.get_status()).unwrap();

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_node() as String),
                &(assembly.get_ip() as String),
                &(assembly.get_urls() as String),
                &(assembly.get_component_collection() as String),
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

    pub fn assembly_show(datastore: &DataStoreConn, get_assembly: &asmsrv::AssemblyGet) -> Result<Option<asmsrv::Assembly>> {
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
        response.set_assemblys(assemblys_collection);
        Ok(Some(response))
    }

    pub fn assembly_status_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<()> {
        let conn = datastore.pool.get_shard(0)?;
        let asm_id = assembly.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(assembly.get_status()).unwrap();
        conn.execute(
            "SELECT set_assembly_status_v1($1, $2)",
            &[&asm_id, &(status_str as String)],
        ).map_err(Error::AsmSetStatus)?;
        Ok(())
    }

    pub fn assembly_factory_create(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<Option<asmsrv::AssemblyFactory>> {
        let status_str = serde_json::to_string(assembly_fac.get_status()).unwrap();
        let properties = serde_json::to_string(assembly_fac.get_properties()).unwrap();
        let component_collection = serde_json::to_string(assembly_fac.get_component_collection()).unwrap();
        let opssettings = serde_json::to_string(assembly_fac.get_opssettings()).unwrap();

        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assembly_factory_create ");

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[
                &(assembly_fac.get_name() as String),
                &(assembly_fac.get_uri() as String),
                &(assembly_fac.get_description() as String),
                &(assembly_fac.get_tags() as Vec<String>),
                &(assembly_fac.get_plan() as String),
                &(properties as String),
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
            let assembly_factory = row_to_assembly_factory(&row)?;
            return Ok(Some(assembly_factory));
        }

        Ok(None)
    }

    pub fn assembly_factory_status_update(datastore: &DataStoreConn, assembly_fac: &asmsrv::AssemblyFactory) -> Result<()> {
        let conn = datastore.pool.get_shard(0)?;
        let asm_fac_id = assembly_fac.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(assembly_fac.get_status()).unwrap();
        conn.execute(
            "SELECT set_assembly_factorys_status_v1($1, $2)",
            &[&asm_fac_id, &(status_str as String)],
        ).map_err(Error::AsmFactorySetStatus)?;
        Ok(())
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
        response.set_assemblys_factory(assembly_factorys_collection);
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
    let parent_id: String = row.get("parent_id");
    let component_collection: String = row.get("component_collection");
    let status: String = row.get("status");
    let node: String = row.get("node");
    let ip: String = row.get("ip");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    assembly.set_id(id.to_string() as String);
    assembly.set_name(name as String);
    assembly.set_urls(urls as String);
    assembly.set_uri(uri as String);
    assembly.set_tags(tags as Vec<String>);
    assembly.set_description(description as String);
    assembly.set_parent_id(parent_id as String);
    assembly.set_component_collection(component_collection as String);
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
    let component_collection_obj: asmsrv::ComponentCollection = serde_json::from_str(&component_collection).unwrap();
    assembly_factory.set_component_collection(component_collection_obj);
    let opssettings_obj: asmsrv::OpsSettings = serde_json::from_str(&opssettings).unwrap();
    assembly_factory.set_opssettings(opssettings_obj);
    let status_obj: asmsrv::Status = serde_json::from_str(&status).unwrap();
    assembly_factory.set_status(status_obj);
    assembly_factory.set_plan(plan as String);
    assembly_factory.set_replicas(replicas as u64);
    let properties_obj: asmsrv::Properties = serde_json::from_str(&properties).unwrap();
    assembly_factory.set_properties(properties_obj);

    debug!(
        "◖☩ ASM: row_to_assemby_factory =>\n{:?}",
        assembly_factory
    );
    debug!("◖☩ DONE: row_to_assemby_factory");
    Ok(assembly_factory)
}
