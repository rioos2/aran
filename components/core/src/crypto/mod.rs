// Copyright 2018 The Rio Advancement Inc

//! Rio/OS core encryption and cryptography.
//!
//! This module uses [openssl](https://docs.rs/openssl) for cryptographic operations.
//!
//! # Concepts and terminology:
//!
//! - All public keys, certificates, and signatures are to be referred to as **public**.
//! - All secret or private keys are to be referred to as **secret**.
//! - In general, the word `key` by itself does not indicate something as
//! **public** or **secret**. The exceptions to this rule are as follows:
//!     - if the word key appears in a URL, then we are referring to a public key to
//!       conform to other APIs that offer similar public key downloading functionality.
//!     - the word `key` appears as part of a file suffix, where it is then considered as
//!       a **secret key** file.
//! # Key file naming
//!
//! ## Certificate authority key
//!
//! ```text
//! server_ca.cert.pem
//! server_ca.key
//! ```
//!
//! ## API Server
//!
//! ```text
//! api-server.pfx [api-server.cert.pem, api-server.key]
//! ```
//!
//! ## Service Accont key
//!
//! ```text
//! service-account.pub
//! service-account.key
//! ```
//!
//! ## Controller, Scheduler, Nodelet, Storlet
//!
//! ```text
//! client-<nodelet/storlet/scheduler/controller>.pub
//! client-<nodelet/storlet/scheduler/controller>.key

use std::path::{Path, PathBuf};

use env as renv;
use fs::rioconfig_key_path;

/// The suffix on the end of a public X509 certs file
pub static PUBLIC_KEY_SUFFIX: &'static str = "cert.pem";

/// The suffix on the end of a private RSA key file
pub static SECRET_SIG_KEY_SUFFIX: &'static str = "key";

/// The suffix on the end of a public RSA key file
pub static PUBLIC_RSA_SUFFIX: &'static str = "pub";

/// The suffix on the end of a public DSA key file
pub static PUBLIC_DSA_SUFFIX: &'static str = "dsa";

/// The suffix on the end of a public ED25519 key file
pub static PUBLIC_ED_SUFFIX: &'static str = "ed";

/// The suffix on the end of a pkcs12 bundled public + private key file
/// Both the X509 public and private RSA key combined into a pkcs12 pfx file
pub static PUBLIC_PFX_SUFFIX: &'static str = "pfx";

/// The prefix of the server root certificate authority
pub static ROOT_CA: &'static str = "server-ca";

/// This environment variable allows you to override the fs::CACHE_KEY_PATH
/// at runtime. This is useful for testing.
pub static CACHE_KEY_PATH_ENV_VAR: &'static str = "RIO_CACHE_KEY_PATH";

/// Create key files with these permissions (both public and secret)
static REGULAR_KEY_PERMISSIONS: u32 = 0o400;
//static SECRET_KEY_PERMISSIONS: u32 = 0o400;

pub use self::keys::sig_key_pair::SigKeyPair;

pub mod keys;

pub fn default_rioconfig_key_path(fs_root_path: Option<&Path>) -> PathBuf {
    match renv::var(CACHE_KEY_PATH_ENV_VAR) {
        Ok(val) => PathBuf::from(val),
        Err(_) => rioconfig_key_path(fs_root_path),
    }
}

#[cfg(test)]
pub mod test_support {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use time;

    use error as herror;

    pub fn fixture(name: &str) -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name);
        if !path.is_file() {
            panic!("Fixture '{}' not found at: {:?}", name, path);
        }
        path
    }

    pub fn fixture_as_string(name: &str) -> String {
        let mut file = File::open(fixture(name)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    pub fn wait_until_ok<F, T>(some_fn: F) -> Option<T>
    where
        F: Fn() -> Result<T, herror::Error>,
    {
        let wait_duration = time::Duration::seconds(30);
        let current_time = time::now_utc().to_timespec();
        let stop_time = current_time + wait_duration;
        while time::now_utc().to_timespec() < stop_time {
            if let Ok(s) = some_fn() {
                return Some(s);
            }
        }
        None
    }
}
