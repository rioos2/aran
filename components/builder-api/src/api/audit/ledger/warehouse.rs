// Copyright 2018 The Rio Advancement Inc
//

//! Ledger variant which uses Exonum blockchain (or an API compatible clone) for
//! the warehouse.
//!
//! Has been tested against [exonum](https://exonum.com) backed by rocksdb.
//!
//! All audits are recorded in the warehouse using the pub/secret keys of the
//! service run by rioos-blockchain-server
//!
//! # Configuration
//!
//! Currently the rioos-blockchain-server must be configured with configuration
//! must be available in $RIOOS_HOME/config/blockchain.toml

use rioos_http::ApiClient as ReqwestClient;
use rioos_http::api_client::err_from_response;

use error::{Result, Error};
use serde_json;
use reqwest::IntoUrl;
use reqwest::header::{ContentType, Accept};
use reqwest::{StatusCode, Body};

use super::Ledger;
use protocol::api::audit::{Envelope, EnvelopeResponse};
use protocol::api::base::{IdGet, MetaFields};

use api::audit::config::BlockchainConn;
use api::audit::ledger::EnvelopeOutputList;

pub struct ExonumClient {
    _inner: ReqwestClient,
    _token: String,
}
impl ExonumClient {
    pub fn new<U>(url: U, token: &str) -> Result<Self>
    where
        U: IntoUrl,
    {
        let url = url.into_url()?;
        Ok(ExonumClient {
            _token: token.to_string(),
            _inner: ReqwestClient::new(url, "rioos", "v1", None).map_err(Error::RioHttpClient)?,
        })
    }
}
pub struct Blockchain {
    _client: ExonumClient,
}

impl Blockchain {
    pub fn new(config: &BlockchainConn) -> Result<Self> {
        let token = "";

        Ok(Blockchain { _client: ExonumClient::new(&config.url, token)? })
    }
}

impl Ledger for Blockchain {
    fn record(&self, envl_req: &Envelope) -> Result<()> {
        let url = format!("api/services/habitat/v1/audits");
        let sbody = serde_json::to_string(&envl_req).unwrap();

        let res = self._client._inner.post(&url).body(Body::from(sbody)).header(Accept::json()).header(ContentType::json()).send().map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            debug!("Failed to signup, status: {:?}", res.status());
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        Ok(())
    }

    fn retrieve_by(&self, id: &IdGet) -> EnvelopeOutputList {
        let url = format!("api/services/habitat/v1/accounts/{}/audits", id.get_id());
        let mut res = self._client._inner.get(&url).header(Accept::json()).header(ContentType::json()).send().map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            debug!("Failed to get audits, status: {:?}", res.status());
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let audits: Vec<Envelope> = res.json()?;

        if audits.is_empty() {
            return Ok(None);
        }

        let data = audits.iter().map(|x| EnvelopeResponse::with(x.event.type_meta(), x.event.object_meta(), x.clone())).collect::<Vec<_>>();
        Ok(Some(data))
    }
}
