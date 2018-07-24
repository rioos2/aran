use deploy::assembler::ServicesConfig as AssemblerServicesConfig;

// Returns the stub services config
//
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServicesCfg {
    pub dns: String,
}

impl Default for ServicesCfg {
    fn default() -> Self {
        ServicesCfg {
            dns: "107.152.143.242".to_string(),
        }
    }
}

/// Convert into ServicesConfig  from the ServiceCfg provided as defaults.
impl Into<AssemblerServicesConfig> for ServicesCfg {
    fn into(self) -> AssemblerServicesConfig {
        AssemblerServicesConfig {
            dns: self.dns,
        }
    }
}
