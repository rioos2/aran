use std::path::PathBuf;
use std::sync::Arc;

use base64;

use rcore::crypto::keys::{PairConf, PairSaverExtn};
use rcore::crypto::{default_rioconfig_key_path, SigKeyPair};
use rcore::fs::{append, open_from};

use error::Result;

use data_store::DataStoreConn;

const AGENT_SECRET: &'static str = "agent_secret";

lazy_static! {
    static ref AGENT_SECRET_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None)
        .join(format!("{}.key", AGENT_SECRET))
        .to_str()
        .unwrap());
    static ref NODELET_CONFIG_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None)
        .join("nodelet.rioconfig")
        .to_str()
        .unwrap());
}

pub struct SystemSecret {
    conn: Arc<DataStoreConn>,
}

impl SystemSecret {
    pub fn new(conn: Arc<DataStoreConn>) -> Self {
        SystemSecret { conn: conn }
    }
    pub fn setup(&self) -> Result<()> {
        self.setup_agent_secret()?;
        Ok(())
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
        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_origin_v1($1,$2)",
            &[&("rioos_system"), &AGENT_SECRET],
        )?;
        if rows.len() == 0 {
            let pairs = SigKeyPair::get_pair_for(AGENT_SECRET, &default_rioconfig_key_path(None))?;
            let rsa_pub_key =
                SigKeyPair::get_rsa_public_key(AGENT_SECRET, &default_rioconfig_key_path(None))?;
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
            append(
                &NODELET_CONFIG_FILE,
                &("\nsecret_name: ".to_string() + AGENT_SECRET),
            )?;
        }
        Ok(())
    }
}
