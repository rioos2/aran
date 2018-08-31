///// Configuration for Secure vault.

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ObjectStorageBackend {
    OpenIO,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ObjectStorageCfg {
    pub backend: ObjectStorageBackend,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Default for ObjectStorageCfg {
    fn default() -> Self {
        ObjectStorageCfg {
            backend: ObjectStorageBackend::OpenIO,
            endpoint: "http://localhost:6007/".to_string(),
            access_key: "demo:demo".to_string(),
            secret_key: "DEMO_PASS".to_string(),
        }
    }
}

pub trait ObjectStorage {
    fn storage_backend(&self) -> ObjectStorageBackend;
    fn storage_endpoint(&self) -> &str;
    fn storage_access_key(&self) -> &str;
    fn storage_secret_key(&self) -> &str;
}

#[allow(unused_variables)]
impl ObjectStorageCfg {
    pub fn new<T: ObjectStorage>(config: &T) -> Self {
        ObjectStorageCfg {
            backend: config.storage_backend(),
            endpoint: config.storage_endpoint().to_string(),
            access_key: config.storage_access_key().to_string(),
            secret_key: config.storage_secret_key().to_string(),
        }
    }
}
