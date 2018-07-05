use deploy::assembler::ServicesConfig as AssemblerServicesConfig;

// Returns the stub services config
//
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServicesCfg {
    pub loadbalancer_imagein: String,
    pub loadbalancer_imagename: String,
    pub loadbalancer_cpu: String,
    pub loadbalancer_mem: String,
    pub loadbalancer_disk: String,
    pub dns: String,
}

impl Default for ServicesCfg {
    fn default() -> Self {
        ServicesCfg {
            loadbalancer_imagein: "container".to_string(),
            loadbalancer_imagename: "registry.rioos.xyz:5000/rioos/loadbalancer".to_string(),
            loadbalancer_cpu: "1".to_string(),
            loadbalancer_mem: "1024 MiB".to_string(),
            loadbalancer_disk: "1 GiB".to_string(),
            dns: "107.152.143.242".to_string(),
        }
    }
}

/// Convert into ServicesConfig  from the ServiceCfg provided as defaults.
impl Into<AssemblerServicesConfig> for ServicesCfg {
    fn into(self) -> AssemblerServicesConfig {
        AssemblerServicesConfig {
            loadbalancer_imagein: self.loadbalancer_imagein,
            loadbalancer_imagename: self.loadbalancer_imagename,
            loadbalancer_cpu: self.loadbalancer_cpu,
            loadbalancer_mem: self.loadbalancer_mem,
            loadbalancer_disk: self.loadbalancer_disk,
            dns: self.dns,
        }
    }
}
