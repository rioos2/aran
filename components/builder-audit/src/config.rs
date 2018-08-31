///host url to check the vulnerability of the container
pub const DEFAULT_ANCHORE_URL: &'static str = "http://localhost:8228/v1";

/// Default Influx Host url to access the log of virtual machine and container
pub const DEFAULT_LOGS_URL: &'static str = "http://localhost:8086";

/// a default username for anchore or anybody else who wish to use the name admin
pub const DEFAULT_USERNAME_ADMIN: &'static str = "admin";

pub trait Logs {
    /// URL to Influx API
    fn influx_endpoint(&self) -> &str;
    /// Includes the prefix of the database,table,path in influx
    fn influx_prefix(&self) -> &str;
}

///// Configuration for Logs

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LogsCfg {
    pub influx_endpoint: String,
    pub influx_prefix: String,
}

impl Default for LogsCfg {
    fn default() -> Self {
        LogsCfg {
            influx_endpoint: DEFAULT_LOGS_URL.to_string(),
            influx_prefix: "rioos_logs".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct InfluxClientConn {
    pub endpoint: String,
    pub prefix: String,
}

#[allow(unused_variables)]
impl InfluxClientConn {
    pub fn new<T: Logs>(config: &T) -> Self {
        InfluxClientConn {
            endpoint: config.influx_endpoint().to_string(),
            prefix: config.influx_prefix().to_string(),
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

///// Configuration for security vulnerability

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VulnerabilityCfg {
    pub anchore_endpoint: String,
    pub anchore_username: String,
    pub anchore_password: String,
}

impl Default for VulnerabilityCfg {
    fn default() -> Self {
        VulnerabilityCfg {
            anchore_endpoint: DEFAULT_ANCHORE_URL.to_string(),
            anchore_username: DEFAULT_USERNAME_ADMIN.to_string(),
            anchore_password: DEFAULT_USERNAME_ADMIN.to_string(),
        }
    }
}

pub trait Vulnerability {
    fn anchore_endpoint(&self) -> &str;
    fn anchore_username(&self) -> &str;
    fn anchore_password(&self) -> &str;
}
