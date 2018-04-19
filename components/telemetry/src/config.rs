
/// host url  to get the audit of the client
pub const DEFAULT_PROMETHEUS_URL: &'static str = "http://localhost:9090/api/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct TelemetryCfg {
    pub url: String,
}

impl Default for TelemetryCfg {
    fn default() -> Self {
        TelemetryCfg { url: DEFAULT_PROMETHEUS_URL.to_string() }
    }
}

pub trait Telemetry {
    fn endpoint(&self) -> &str;
}
