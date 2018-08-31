// Copyright 2018 The Rio Advancement Inc

//! The startup hook is responsible for sticking an agent secret under secrets.
// in origin: rioos_system, name: agent_secret
// This can be pulled like /origin/rioos_system/secrets/agent_secret
//TO-DO: This seems like the same like setting_map. We need to discuss.
use base64;
use db::data_store::DataStoreConn;
use error::Result;
use hooks::BeforeHook;
use rio_core::crypto::keys::{PairConf, PairSaverExtn};
use rio_core::crypto::{default_rioconfig_key_path, SigKeyPair};
use rio_core::fs::{append, open_from};
use std::path::PathBuf;

const AGENT_SECRET: &'static str = "agent_secret";

lazy_static! {
    static ref AGENT_SECRET_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None).join(format!("{}.key", AGENT_SECRET)).to_str().unwrap());
    static ref NODELET_CONFIG_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None).join("nodelet.rioconfig").to_str().unwrap());
}

pub struct ForGulpd {
    conn: Box<DataStoreConn>,
}

impl ForGulpd {
    pub fn new(conn: Box<DataStoreConn>) -> Self {
        ForGulpd { conn: conn }
    }

    fn setup_agent_secret(&self) -> Result<()> {
        let conn = self.conn.pool.get_shard(0)?;
        open_from(&NODELET_CONFIG_FILE.as_path())?;

        if !(AGENT_SECRET_FILE.exists()) {
            SigKeyPair::mk_signed(
                AGENT_SECRET,
                PairConf::with_extn(PairSaverExtn::PubRSA),
                &default_rioconfig_key_path(None),
            )?;
        }
        let rows = &conn.query("SELECT * FROM get_secrets_by_origin_v1($1,$2)", &[&("rioos_system"), &AGENT_SECRET])?;
        if rows.len() == 0 {
            let pairs = SigKeyPair::get_pair_for(AGENT_SECRET, &default_rioconfig_key_path(None))?;
            let rsa_pub_key = SigKeyPair::get_rsa_public_key(AGENT_SECRET, &default_rioconfig_key_path(None))?;
            &conn.query(
                "SELECT * FROM insert_secret_v1($1,$2,$3,$4,$5)",
                &[
                    &("rioos_sh/token"),
                    &(json!({
                        "rioos_sh/ssh_privatekey": format!("{}",base64::encode(&pairs.secret()?)),
                        "rioos_sh/ssh_pubkey": format!("{}",base64::encode(&rsa_pub_key))
                    })),
                    &(json!({
                        "origin": "rioos_system",
                    })),
                    &(json!({
                        "name": format!("{}",AGENT_SECRET),
                    })),
                    &(json!({
                        "kind": "Secret",
                        "api_version":"v1"
                    })),
                ],
            )?;
            append(&NODELET_CONFIG_FILE, &("\nsecret_name: ".to_string() + AGENT_SECRET))?;
        }
        Ok(())
    }
}

impl BeforeHook for ForGulpd {
    fn before(&self) -> Result<()> {
        self.setup_agent_secret()?;
        Ok(())
    }
}
