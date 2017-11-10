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

use hyper::client::Client as HyperClient;

use super::Securer;
use rio_net::http::middleware::SecurerConn;
use error::{Result, Error};

use protocol::servicesrv::Secret;

//
// pub struct EnvKeySecurer {
//     client: VaultClient<HyperClient>,
// }

pub struct EnvKeySecurer {
    client: SecurerConn,
}

impl EnvKeySecurer {
    pub fn new(config: &SecurerConn) -> Result<Self> {
        // let final_endpoint = match config.endpoint {
        //     Some(ref url) => {
        //         let url = extern_url::Url::parse(url.as_str()).expect("Invalid endpoint URL given");
        //         Some(url)
        //     }
        //     None => None,
        // };
        //
        // let user_agent = format!("RioOS-API/{}", VERSION);
        //
        // let token = config.token.as_str().unwrap();
        //
        // let client = VaultClient::new(endpoint);

        Ok(EnvKeySecurer { client: config.clone() })
    }
}

impl Securer for EnvKeySecurer {
    fn seal(&self) -> Result<()> {
        Ok(())
    }

    fn status(&self) -> Result<()> {
        Ok(())
    }

    fn unseal(&self) -> Result<()> {
        Ok(())
    }

    fn secure(&self, security_req: &Secret) -> Result<Option<Secret>> {
        // // This panics if it can't resolve the URL (e.g.,
        // // there's a netsplit, your Minio goes down, S3 goes down (!)).
        // // We have to catch it, otherwise no more logs get captured!
        // //
        // // The code in the S3 library we're currently using isn't
        // // UnwindSafe, so we need to deal with that, too.
        // let result = panic::catch_unwind(AssertUnwindSafe(
        //     || self.client.put_object(&security_req, None),
        // ));
        //
        // match result {
        //     Ok(Ok(_)) => Ok(()), // normal result
        //     Ok(Err(e)) => {
        //         // This is a "normal", non-panicking error, e.g.,
        //         // they're configured with a non-existent bucket.
        //         Err(Error::JobLogArchive(job_id, e))
        //     }
        //     Err(e) => {
        //         let source = match e.downcast_ref::<String>() {
        //             Some(string) => string.to_string(),
        //             None => format!("{:?}", e),
        //         };
        //         Err(Error::CaughtPanic(
        //             format!("Failure to archive log for job {}", job_id),
        //             source,
        //         ))
        //     }
        // }
        Ok(Some(security_req.clone()))
    }

    fn retrieve(&self, security_id: u64) -> Result<Vec<String>> {
        // //As above when uploading a job file, we currently need to
        // // catch a potential panic if the object store cannot be reached
        // let result = panic::catch_unwind(AssertUnwindSafe(
        //     || self.client.get_object(&security_id, None),
        // ));
        //
        // let body = match result {
        //     Ok(Ok(response)) => response.body, // normal result
        //     Ok(Err(e)) => {
        //         // This is a "normal", non-panicking error, e.g.,
        //         // they're configured with a non-existent bucket.
        //         return Err(Error::JobLogRetrieval(job_id, e));
        //     }
        //     Err(e) => {
        //         let source = match e.downcast_ref::<String>() {
        //             Some(string) => string.to_string(),
        //             None => format!("{:?}", e),
        //         };
        //         return Err(Error::CaughtPanic(
        //             format!(
        //                 "Failure to retrieve archived log for job {}",
        //                 job_id
        //             ),
        //             source,
        //         ));
        //     }
        // };
        //
        // let lines = String::from_utf8_lossy(body.as_slice())
        //     .lines()
        //     .map(|l| l.to_string())
        //     .collect();
        //
        // Ok(lines)

        let data = vec![];
        Ok(data)
    }
}
