#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Backend {
    LicenseCloud,
}

///// Configuration structure for validating license

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LicensesCfg {
    /// The standard object (.so)  path to use
    pub so_file: String,
    /// The activation license code bought by the customer (or) we will assume we are on trial mode.
    pub activation_code: Option<String>,
    pub backend: Backend,
}

impl Default for LicensesCfg {
    fn default() -> Self {
        LicensesCfg {
            so_file: "ShaferFilechck.so".to_string(),
            activation_code: None,
            backend: Backend::LicenseCloud,
        }
    }
}

/// Apply to every api requests when the api server receives connection requests
pub trait License {
    /// Return the licensors API .so (.dll = nalp_linux_64.so or nalp_freebsd_64.so) file path
    fn so_file(&self) -> &str;
    /// Return the license code for the site (Default is blank)
    fn activation_code(&self) -> Option<String>;
    fn backend(&self) -> Backend;
}
