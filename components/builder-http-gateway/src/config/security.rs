///// Configuration for Secure vault.

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SecureBackend {
    Local,
    EnvKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurerCfg {
    pub backend: SecureBackend,
    pub endpoint: String,
    pub token: String,
}

impl Default for SecurerCfg {
    fn default() -> Self {
        SecurerCfg {
            backend: SecureBackend::Local,
            endpoint: "".to_string(),
            token: "".to_string(),
        }
    }
}

pub trait SecurerAuth {
    fn backend(&self) -> SecureBackend;
    fn endpoint(&self) -> &str;
    fn token(&self) -> &str;
}

/// Trait that feeds the configuration into the APIWirers.
/// This trait feed the configuration into the PasswordClient (via PasswordCLI)
pub trait PasswordAuth {}

/// This trait feed the service account public key credential configuration into the
/// Authenticated (Authenticated is invoked by all APIs (from APIWirers)
pub trait SystemAuth {
    fn serviceaccount_public_key(&self) -> Option<String>;
}
