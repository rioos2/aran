// Copyright (c) 2017 RioCorp Inc.
//

//! Archiver variant which uses Vault (or an API compatible clone) for
//! vault.
//!
//! Has been tested against [vaultproject](https://vaultproject.io).
//!
//! All secrets are stored in the vault using the auth/master keys of the
//! vault
//!
//! # Configuration
//!
//! Currently the archiver must be configured with both an access key
//! ID and a secret access key.

use std::panic::{self, AssertUnwindSafe};
use std::str::FromStr;

use extern_url;
use hyper::client::Client as HyperClient;

use super::Securer;
use VERSION;
use config::SecurerCfg;
use error::{Result, Error};

pub struct VaultSecurer {
    client: VaultClient<HyperClient>,
}

impl VaultSecurer {
    pub fn new(config: &SecurerCfg) -> Result<VaultSecurer> {
        let final_endpoint = match config.endpoint {
            Some(ref url) => {
                let url = extern_url::Url::parse(url.as_str()).expect("Invalid endpoint URL given");
                Some(url)
            }
            None => None,
        };

        let user_agent = format!("RioOS-API/{}", VERSION);

        let token = config.token.as_str().unwrap();

        let client = VaultClient::new(endpoint);

        Ok(VaultSecurer { client: client })
    }
}

impl Securer for VaultSecurer {
    fn seal() -> Result<()> {
        Ok(())
    }

    fn status() -> Result<()> {
        Ok(())
    }

    fn unseal() -> Result<()> {
        Ok(())
    }

    fn secure(&self, security_id: u64, security_req: &SecurityCreateReq) -> Result<()> {
        // This panics if it can't resolve the URL (e.g.,
        // there's a netsplit, your Minio goes down, S3 goes down (!)).
        // We have to catch it, otherwise no more logs get captured!
        //
        // The code in the S3 library we're currently using isn't
        // UnwindSafe, so we need to deal with that, too.
        let result = panic::catch_unwind(AssertUnwindSafe(
            || self.client.put_object(&security_req, None),
        ));

        match result {
            Ok(Ok(_)) => Ok(()), // normal result
            Ok(Err(e)) => {
                // This is a "normal", non-panicking error, e.g.,
                // they're configured with a non-existent bucket.
                Err(Error::JobLogArchive(job_id, e))
            }
            Err(e) => {
                let source = match e.downcast_ref::<String>() {
                    Some(string) => string.to_string(),
                    None => format!("{:?}", e),
                };
                Err(Error::CaughtPanic(
                    format!("Failure to archive log for job {}", job_id),
                    source,
                ))
            }
        }
    }

    fn retrieve(&self, security_id: u64) -> Result<Vec<String>> {
        // As above when uploading a job file, we currently need to
        // catch a potential panic if the object store cannot be reached
        let result = panic::catch_unwind(AssertUnwindSafe(
            || self.client.get_object(&security_id, None),
        ));

        let body = match result {
            Ok(Ok(response)) => response.body, // normal result
            Ok(Err(e)) => {
                // This is a "normal", non-panicking error, e.g.,
                // they're configured with a non-existent bucket.
                return Err(Error::JobLogRetrieval(job_id, e));
            }
            Err(e) => {
                let source = match e.downcast_ref::<String>() {
                    Some(string) => string.to_string(),
                    None => format!("{:?}", e),
                };
                return Err(Error::CaughtPanic(
                    format!(
                        "Failure to retrieve archived log for job {}",
                        job_id
                    ),
                    source,
                ));
            }
        };

        let lines = String::from_utf8_lossy(body.as_slice())
            .lines()
            .map(|l| l.to_string())
            .collect();

        Ok(lines)
    }
}
