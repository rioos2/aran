// Copyright 2018 The Rio Advancement Inc

use std::path::PathBuf;
use std::collections::HashMap;

use regex::Regex;

pub const PLUGIN_PASSWORD: &'static str = "password";
pub const PLUGIN_SERVICE_ACCOUNT: &'static str = "service_account";
pub const PLUGIN_PASSTICKET: &'static str = "passticket";
pub const PLUGIN_JWT: &'static str = "jwt";
pub const PLUGIN_SERVICE_ACCOUNT_KEY_PUB: &'static str = "service_account_pub";

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct IdentityCfg {
    //  The identity handlers enabled
    //  example ["password", "serviceaccount", "otp", "jwt"]
    pub enabled: Vec<String>,

    //  A key value params hash as needed in AuthenticationFlowCfg
    pub params: HashMap<String, String>,
}

/// The default handlers turned on are
/// ["password"]
impl Default for IdentityCfg {
    fn default() -> Self {
        IdentityCfg {
            enabled: vec!["password".to_string()],
            params: HashMap::new(),
        }
    }
}

pub trait Identity {
    fn enabled(&self) -> Vec<String>;

    fn params(&self) -> HashMap<String, String>;
}

/// A trait that feeds configuration for authentication flow.
/// The modes supported are
/// 1. password (using password or token)
/// 2. service_account. Additional configuration is needed when service_account is turned on.
/// 3. jwt. An user authenticating using jwt
/// 3. passticket. OTP
pub trait AuthenticationFlowCfg {
    //A tuple of enabled identitycfg with its parameters is sent back.
    //1. Example:
    ///[identity]
    ///enabled=["password"]
    ///In the above case modes() returns
    ///(password, HashMap[()])
    ///
    //2. Example:
    ///[identity]
    ///enabled=["password", "service_account"]
    ///params={"service_account": "service_account.pub"}
    ///In the above case modes() returns
    ///([password, service_account],
    ///   HashMap[(service_account, <$RIOOS_HOME>/config/service_account.pub)])
    //
    //3. Example:
    ///[identity]
    ///enabled=["password", "service_account", "otp"]
    ///params={"service_account": "service_account.pub"}
    ///In the above case modes() returns
    ///([password, service_account,otp],
    ///   HashMap[(service_account, <$RIOOS_HOME>/config/service_account.pub)])
    fn modes(&self) -> (Vec<String>, HashMap<String, String>);
}

///A public wrapper function that converts Identity into another form of a tuple.
///Seems like a Into function actually.
pub fn flow_modes<I: Identity>(identity: &I, prefix_path: PathBuf) -> (Vec<String>, HashMap<String, String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*\.((pub|key|toml|hbs|cert.pem)).*$").unwrap();
    }

    let b = identity
        .enabled()
        .iter()
        .map(|enabz| {
            (
                enabz.to_string(),
                identity
                    .params()
                    .clone()
                    .into_iter()
                    .filter(|kv| kv.0 == *enabz.clone())
                    .map(|x| {
                        if RE.is_match(&x.1) {
                            (
                                x.0,
                                prefix_path
                                    .join(x.1.clone())
                                    .to_str()
                                    .unwrap_or(&x.1)
                                    .to_string(),
                            )
                        } else {
                            x
                        }
                    })
                    .collect::<_>(),
            )
        })
        .collect::<Vec<(String, HashMap<String, String>)>>();
    (
        b.clone().into_iter().map(|x| x.0).collect::<Vec<String>>(),
        b.into_iter()
            .map(|x| x.1)
            .flat_map(|y| y)
            .collect::<HashMap<_, String>>(),
    )
}
