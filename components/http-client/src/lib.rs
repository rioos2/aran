// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//


extern crate base64;
extern crate rioos_core as rio_core;
extern crate httparse;
#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate openssl;
extern crate url;

pub mod api_client;
pub mod error;
pub mod net;
pub mod proxy;
pub mod util;

pub use api_client::ApiClient;
pub use error::{Error, Result};

#[cfg(not(target_os = "macos"))]
mod ssl {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    use rio_core::fs::rioconfig_ssl_path;
    use openssl::ssl::SslContextBuilder;

    use error::Result;

    const CACERT_PEM: &'static str = include_str!(concat!(env!("OUT_DIR"), "/cacert.pem"));

    pub fn set_ca(ctx: &mut SslContextBuilder, fs_root_path: Option<&Path>) -> Result<()> {
        let cached_certs = rioconfig_ssl_path(fs_root_path).join("cert.pem");
        if !cached_certs.exists() {
            try!(fs::create_dir_all(rioconfig_ssl_path(fs_root_path)));
            debug!("Creating cached cacert.pem at: {}", cached_certs.display());
            let mut file = try!(File::create(&cached_certs));
            try!(file.write_all(CACERT_PEM.as_bytes()));
        }
        debug!(
            "Setting CA file for SSL context to: {}",
            cached_certs.display()
        );
        try!(ctx.set_ca_file(cached_certs));

        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod ssl {
    use std::path::Path;

    use openssl::ssl::SslContextBuilder;

    use error::Result;

    pub fn set_ca(ctx: &mut SslContextBuilder, _fs_root_path: Option<&Path>) -> Result<()> {
        try!(ctx.set_default_verify_paths());
        Ok(())
    }
}
