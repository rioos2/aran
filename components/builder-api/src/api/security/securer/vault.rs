// Copyright 2018 The Rio Advancement Inc
//

//! Securer variant which uses Vault (or an API compatible clone) for
//! vault.
//!
//! Has been tested against [envkey](https://envkey.com).
//!
//! All secrets are stored in the vault using the auth/master keys of the
//! vault
//!
//! # Configuration
//!
//! Currently the envkey must be configured with an admin access key
//! and a write binary should be available in $RIOOS_HOME/config

use rioos_http::ApiClient as ReqwestClient;

use rio_net::http::middleware::SecurerConn;
use error::{Result, Error};

use reqwest::IntoUrl;

use super::Securer;
use protocol::api::secret::Secret;
use protocol::api::base::IdGet;

use service::{SecretOutput, SecretOutputList};

pub struct EnvKeyClient {
    _inner: ReqwestClient,
    _token: String,
}
impl EnvKeyClient {
    pub fn new<U>(url: U, token: &str) -> Result<Self>
    where
        U: IntoUrl,
    {
        let url = url.into_url()?;
        Ok(EnvKeyClient {
            _token: token.to_string(),
            _inner: ReqwestClient::new(url, "rioos", "v1", None).map_err(
                Error::RioHttpClient,
            )?,
        })
    }
}
pub struct EnvKeySecurer {
    _client: EnvKeyClient,
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
            _client: EnvKeyClient::new(&config.endpoint, token[0])?,
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

    fn secure(&self, security_req: &Secret) -> SecretOutput {
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

    fn retrieve_by(&self, _id: &IdGet) -> SecretOutputList {
        // let mut res = self.client
        //     .inner
        //     .get(&self.client.token)
        //     .header(Accept::json())
        //     .header(ContentType::json())
        //     .send()
        //     .map_err(Error::ReqwestError)?;
        //
        // let mut encoded = String::new();
        //
        // res.read_to_string(&mut encoded).map_err(Error::IO)?;
        // let mut secret = Secret::new();
        // secret.set_secret_type(encoded);
        Ok(Some(vec![Secret::new()]))
    }

    fn retrieve(&self) -> SecretOutputList {
        Ok(Some(vec![Secret::new()]))
    }
}
