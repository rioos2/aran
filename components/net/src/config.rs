/// host url  to get the audit of the client
pub const DEFAULT_PROMETHEUS_URL: &'static str = "http://localhost:9090/api/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PrometheusCfg {
    pub url: String,
}

impl Default for PrometheusCfg {
    fn default() -> Self {
        PrometheusCfg { url: DEFAULT_PROMETHEUS_URL.to_string() }
    }
}

pub trait Prometheus {
    fn endpoint(&self) -> &str;
}
