// Copyright (c) 2017 RioCorp Inc.
//

//! Archiver variant which uses EnvKey (or an API compatible clone) for
//! EnvKey.
//!
//!
//! All secrets are stored in the EnvKey using the auth/master keys of the
//! EnvKey
//!
//! # Configuration
//!
//! Currently the archiver must be configured with both an access key
//! ID and a secret access key.

use std::panic::{self, AssertUnwindSafe};
use std::str::FromStr;

use rioos_http::ApiClient as HyperClient;

use super::Securer;
use rio_net::http::middleware::SecurerConn;
use error::{Result, Error};
use hyper::header::{ContentType, Accept};
use hyper::client::IntoUrl;
use std::io::{Read, Write};

use protocol::servicesrv::Secret;

pub struct EnvKeyClient {
    inner: HyperClient,
    token: String,
}
impl EnvKeyClient {
    pub fn new<U>(url: U, token: &str) -> Result<Self>
    where
        U: IntoUrl,
    {
        let url = url.into_url()?;
        Ok(EnvKeyClient {
            token: token.to_string(),
            inner: HyperClient::new(url, "rioos", "v1", None).map_err(
                Error::HabitatHttpClient,
            )?,
        })
    }
}
pub struct EnvKeySecurer {
    client: EnvKeyClient,
}

impl EnvKeySecurer {
    pub fn new(config: &SecurerConn) -> Result<Self> {
        // let final_endpoint = match config.endpoint {
        //     Some(ref url) => {
        //         let url = extern_url::Url::parse(url.as_str()).expect("Invalid endpoint URL given");
        //         url
        //     }
        //     None => None,
        // };

        let token = config.token.split("-").collect::<Vec<_>>();

        Ok(EnvKeySecurer {
            client: EnvKeyClient::new(&config.endpoint, token[0])?,
        })
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

    fn retrieve(&self) -> Result<Option<Secret>> {
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
        // let res = self.client.inner.get("").send().map_err(Error::HyperError)?;


        let mut res = self.client
            .inner
            .get(&self.client.token)
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::HyperError)?;

        let mut encoded = String::new();

        res.read_to_string(&mut encoded).map_err(Error::IO)?;
        let mut secret = Secret::new();
        secret.set_secret_type(encoded);
        Ok(Some(secret))
    }
}
