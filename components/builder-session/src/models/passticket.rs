// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Datastore.
use error::{Result, Error};
use db::data_store::DataStoreConn;
use protocol::api::passticket::PassTicket;
use chrono::prelude::*;
use postgres;

pub struct DataStore;

impl DataStore {
    pub fn create_passticket(datastore: &DataStoreConn, passticket_id: &str) -> Result<Option<PassTicket>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_passticket_v1($1)",
            &[&(passticket_id.to_string())],
        ).map_err(Error::PassTicketCreate)?;
        if rows.len() > 0 {
            let passticket = row_to_passticket(&rows.get(0));
            return Ok(Some(passticket));
        }
        Ok(None)
    }

    pub fn get_passticket(datastore: &DataStoreConn, passticket_id: &str) -> Result<Option<String>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_passticket_v1($1)",
            &[&(passticket_id.to_string())],
        ).map_err(Error::PassTicketGet)?;
        if rows.len() > 0 {
            let passticket = rows.get(0).get("passticket");
            return Ok(Some(passticket));
        }
        Ok(None)
    }


    pub fn remove_passticket(datastore: &DataStoreConn, passticket_id: String) -> Result<()> {
        let conn = datastore.pool.get_shard(0)?;
        &conn.query(
            "SELECT * FROM remove_passticket_v1($1)",
            &[&(passticket_id)],
        ).map_err(Error::PassTicketDelete)?;
        Ok(())

    }
}

fn row_to_passticket(row: &postgres::rows::Row) -> PassTicket {
    let mut passticket = PassTicket::new();
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    passticket.set_id(id.to_string());
    passticket.set_passticket(row.get("passticket"));
    passticket.set_created_at(created_at.to_rfc3339());
    passticket
}
