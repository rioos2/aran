///host url to check the vulnerability of the container
pub const DEFAULT_ANCHORE_URL: &'static str = "http://localhost:8228/v1";
/// a default username for anchore or anybody else who wish to use the name admin
pub const DEFAULT_USERNAME_ADMIN: &'static str = "admin";

/// Default Influx Host url to access the log of virtual machine and container
pub const DEFAULT_LOGS_URL: &'static str = "http://localhost:8086";

///// Configuration for security vulnerability

pub trait Anchore {
    fn endpoint(&self) -> &str;
    fn username(&self) -> &str;
    fn password(&self) -> &str;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct AnchoreCfg {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Default for AnchoreCfg {
    fn default() -> Self {
        AnchoreCfg {
            url: DEFAULT_ANCHORE_URL.to_string(),
            username: DEFAULT_USERNAME_ADMIN.to_string(),
            password: DEFAULT_USERNAME_ADMIN.to_string(),
        }
    }
}



pub trait Influx {
    /// URL to Influx API
    fn endpoint(&self) -> &str;
    /// Includes the prefix of the database,table,path in influx
    fn prefix(&self) -> &str;
}


///// Configuration for Logs

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LogsCfg {
    pub url: String,
    pub prefix: String,
}

impl Default for LogsCfg {
    fn default() -> Self {
        LogsCfg {
            url: DEFAULT_LOGS_URL.to_string(),
            prefix: "rioos_logs".to_string(),
        }
    }
}


#[derive(Clone)]
pub struct InfluxClientConn {
    pub url: String,
    pub prefix: String,
}

#[allow(unused_variables)]
impl InfluxClientConn {
    pub fn new<T: Influx>(config: &T) -> Self {
        InfluxClientConn {
            url: config.endpoint().to_string(),
            prefix: config.prefix().to_string(),
        }
    }
    pub fn db(&self) -> String {
        self.prefix.clone() + "db"
    }

    pub fn table(&self) -> String {
        self.prefix.clone()
    }

    pub fn path(&self) -> String {
        self.prefix.clone() + "Path"
    }
}
