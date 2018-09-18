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


use super::Ledger;

use api::blockchain::config::BlockchainConn;
use api::blockchain::ledger::EnvelopeOutputList;

use error::{Error, Result};
use protocol::api::audit::{Envelope, EnvelopeResponse};
use protocol::api::base::{IdGet, MetaFields};
use reqwest::{Body, StatusCode};
use reqwest::IntoUrl;
use reqwest::header::{Accept, ContentType};
use rioos_http::ApiClient as ReqwestClient;
use rioos_http::api_client::err_from_response;
use serde_json;

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
            _inner: ReqwestClient::new(url, "rioos", "v1", None).map_err(
                Error::RioHttpClient,
            )?,
        })
    }
}
pub struct Blockchain {
    _client: ExonumClient,
}

impl Blockchain {
    pub fn new(config: &BlockchainConn) -> Result<Self> {
        let token = "";

        Ok(Blockchain {
            _client: ExonumClient::new(&config.url, token)?,
        })
    }
}

impl Ledger for Blockchain {
    fn record_audit(&self, envl_req: &Envelope) -> Result<()> {
        let url = format!("api/services/audit/v1/audits");
        let sbody = serde_json::to_string(&envl_req).unwrap();

        let res = self._client
            ._inner
            .post(&url)
            .body(Body::from(sbody))
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            debug!("Failed to signup, status: {:?}", res.status());
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        Ok(())
    }

    fn record_event(&self, envl_req: &Envelope) -> Result<()> {
        let url = format!("api/services/event/v1/events");
        let sbody = serde_json::to_string(&envl_req).unwrap();

        let res = self._client
            ._inner
            .post(&url)
            .body(Body::from(sbody))
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            debug!("Failed to signup, status: {:?}", res.status());
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        Ok(())
    }

    fn retrieve_audits(&self) -> EnvelopeOutputList {
        let url = format!("api/services/audit/v1/audits");
        let mut res = self._client
            ._inner
            .get(&url)
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            debug!("Failed to get audits, status: {:?}", res.status());
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let audits: Vec<Envelope> = res.json()?;

        if audits.is_empty() {
            return Ok(None);
        }

        let data = audits
            .iter()
            .map(|x| {
                EnvelopeResponse::with(
                    x.get_event().type_meta(),
                    x.get_event().object_meta(),
                    x.clone(),
                )
            })
            .collect::<Vec<_>>();
        Ok(Some(data))
    }

    fn retrieve_events(&self, id: &IdGet) -> EnvelopeOutputList {
        let url = format!(
            "api/services/event/v1/accounts/events?account={}",
            id.get_name()
        );
        let mut res = self._client
            ._inner
            .get(&url)
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            debug!("Failed to get audits, status: {:?}", res.status());
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let audits: Vec<Envelope> = res.json()?;

        if audits.is_empty() {
            return Ok(None);
        }

        let data = audits
            .iter()
            .map(|x| {
                EnvelopeResponse::with(
                    x.get_event().type_meta(),
                    x.get_event().object_meta(),
                    x.clone(),
                )
            })
            .collect::<Vec<_>>();
        Ok(Some(data))
    }
}
