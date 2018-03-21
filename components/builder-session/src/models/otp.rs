// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the SessionDS.
use error::{Result, Error};
use db::data_store::DataStoreConn;

pub struct OtpDS;

impl OtpDS {
    pub fn get_otp(datastore: &DataStoreConn, token: &str) -> Result<Option<String>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_otp_v1($1)", &[&(token.to_string())])
            .map_err(Error::OTPGet)?;
        if rows.len() > 0 {
            let otp = rows.get(0).get("otp");
            return Ok(Some(otp));
        }
        Ok(None)
    }


    pub fn remove_otp(datastore: &DataStoreConn, token: String) -> Result<()> {
        let conn = datastore.pool.get_shard(0)?;
        &conn.query("SELECT * FROM remove_otp_v1($1)", &[&(token)])
            .map_err(Error::OTPDelete)?;
        Ok(())

    }
}
