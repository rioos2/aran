use std::path::PathBuf;
use std::collections::BTreeMap;

use base64;
use jwt::{sign, Algorithm};

use rcore::fs::{read_from_yaml, read_from_file, append};
use rcore::crypto::{default_rioconfig_key_path, SigKeyPair};
use rcore::crypto::keys::{PairConf, PairSaverExtn};

use error::Result;

use data_store::DataStoreConn;

const AGENT_SECRET: &'static str = "agent_secret";

lazy_static! {
    static  ref BOOTSTRAP_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join("bootstrap_token.rioconfig").to_str().unwrap());
    static  ref AGENT_SECRET_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join(format!("{}.key", AGENT_SECRET)).to_str().unwrap());
    static  ref NODELET_CONFIG_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join("nodelet.rioconfig").to_str().unwrap());
}

pub struct SystemSecret {
    conn: DataStoreConn,
}

impl SystemSecret {
    pub fn new(conn: DataStoreConn) -> Self {
        SystemSecret { conn: conn }
    }
    pub fn setup(&self) -> Result<()> {
        let content = read_from_yaml(&BOOTSTRAP_FILE)?;
        self.setup_system_secret(content.clone())?;
        self.setup_agent_secret()?;
        self.setup_settings_map(content.clone())?;
        Ok(())
    }

    fn setup_system_secret(&self, content: BTreeMap<String, String>) -> Result<()> {

        let conn = self.conn.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_origin_v1($1,$2)",
            &[&("rioos_system"), &content.get("name")],
        )?;


        if rows.len() == 0 {

            &conn.query(
                "SELECT * FROM insert_secret_v1($1,$2,$3,$4,$5)",
                &[
                    &(content.get("type").unwrap_or(&"null".to_string())),
                    &(json!({
                        "token-id": format!("{}",content.get("token-id").unwrap()),
                        "token-secret": format!("{}",content.get("token-secret").unwrap())
                    })),
                    &(json!({
                        "origin": "rioos_system",
                    })),
                    &(json!({
                        "name": format!("{}",content.get("name").unwrap()),
                    })),
                    &(json!({
                        "kind": "Secret",
                        "api_version":"v1"
                    })),
                ],
            )?;
        }
        Ok(())
    }

    fn setup_agent_secret(&self) -> Result<()> {
        let conn = self.conn.pool.get_shard(0)?;

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

            append(
                &NODELET_CONFIG_FILE,
                &("\nsecret-name: ".to_string() + AGENT_SECRET),
            )?;
        }
        Ok(())
    }

    fn setup_settings_map(&self, content: BTreeMap<String, String>) -> Result<()> {
        let conn = self.conn.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_settings_maps_v1()", &[])?;

        if rows.len() == 0 {
            let nodelet_content = read_from_file(&NODELET_CONFIG_FILE)?;

            let data = format!(
                "{}.{}{}",
                content.get("token-id").unwrap_or(&"null".to_string()),
                content.get("token-secret").unwrap_or(&"null".to_string()),
                nodelet_content
            );

            let token = sign(
                &content.get("name").unwrap_or(&"null".to_string()),
                data.as_bytes(),
                Algorithm::HS256,
            )?;

            &conn.query(
                "SELECT * FROM insert_settings_map_v1($1,$2,$3,$4)",
                &[
                    &(json!({
                    "jws_token": format!("{}",token),
                    "config": format!("{}",base64::encode(&nodelet_content.as_bytes()))
                })),
                    &(json!({
                    "origin": "rioos_system",
                })),
                    &(json!({
                    "name": "nodelet_info",
                })),
                    &(json!({
                    "kind": "SettingsMap",
                    "api_version":"v1"
                })),
                ],
            )?;
        }
        Ok(())
    }
}
