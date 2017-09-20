// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use std::error::Error as StdError;
use std::collections::HashMap;
use std::io::Read;
use std::result::Result as StdResult;
use std::time::Duration;

use hyper::{self, Url};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use protocol::{net};
use serde_json;

use config;

const USER_AGENT: &'static str = "Rio/OS Aran";
const HTTP_TIMEOUT: u64 = 3_000;


#[derive(Clone)]
pub struct PrometheusClient {
    pub url: String,
}

impl PrometheusClient {
    pub fn new<T>(config: &T) -> Self
    where
        T: config::Prometheus,
    {
        PrometheusClient { url: config.prometheus_url().to_string() }
    }

    /// Returns the contents of the node metrics
    pub fn overall(&self, token: &str, path: &str) -> Result<Contents> {
        let url = Url::parse(&format!(
            "{}/v2/{}",
            self.url,
            path
        )).unwrap();


        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;

        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(Error::PrometheusAPI(rep.status, err));
        }
        let  contents: Contents = serde_json::from_str(&body).unwrap();

        Ok(contents)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Contents {
    pub name: String,
}


fn http_get(url: Url, token: &str) -> StdResult<hyper::client::response::Response, net::NetError> {
    hyper_client()
        .get(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
        ]))
        .header(Authorization(Bearer { token: token.to_owned() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(hyper_to_net_err)
}

//
// fn http_post(url: Url) -> StdResult<hyper::client::response::Response, net::NetError> {
//     hyper_client()
//         .post(url)
//         .header(Accept(vec![
//             qitem(
//                 Mime(TopLevel::Application, SubLevel::Json, vec![])
//             ),
//         ]))
//         .header(UserAgent(USER_AGENT.to_string()))
//         .send()
//         .map_err(hyper_to_net_err)
// }
//
// fn hyper_client() -> hyper::Client {
//     let ssl = OpensslClient::new().unwrap();
//     let connector = HttpsConnector::new(ssl);
//     let mut client = hyper::Client::with_connector(connector);
//     client.set_read_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
//     client.set_write_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
//     client
// }
// //
// fn hyper_to_net_err(err: hyper::error::Error) -> net::NetError {
//     net::err(net::ErrCode::BAD_REMOTE_REPLY, err.description())
// }
