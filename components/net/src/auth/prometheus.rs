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
use error::{Error, Result};

const USER_AGENT: &'static str = "Rio/OS Aran";
const HTTP_TIMEOUT: u64 = 3_000;
// These OAuth scopes are required for a user to be authenticated. If this list is updated, then
// the front-end also needs to be updated in `components/builder-web/app/util.ts`. Both the
// front-end app and back-end app should have identical requirements to make things easier for
// our users and less cumbersome for us to message out.
// https://developer.github.com/v3/oauth/#scopes
//const AUTH_SCOPES: &'static [&'static str] = &["user:email", "read:org"];

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
            "{}/nodes/{}",
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
        r#"
        "gauges": {
            "title": "Cumulative operations counter",
             "counters": [{
              "name": "cpu",
              "description": "CPU ..Throttled",
              "cpu": "percentage",
              "counter": "100"
            },
            {
              "name": "ram",
              "description": "RAM ..Throttled",
              "cpu": "percentage",
              "counter": "100"
            },
            {
              "name": "disk",
              "description": "DISK ..Throttled",
              "cpu": "percentage",
              "counter": "100"
            }]
        }"#;

        {
  "result": {
    "title": "Command center operations",

    "statistics": {
        "title": "Statistics of the nodes",
        "nodes": [{
          "name": "name_of_the_node",
          "description": "CPU ..Throttled",
          "cpu": "percentage",
          "counter": "100",
          "cost_of_consumption": "2000 USD",
          "health": "green/red/yellow"
        },  {
          "name": "name_of_the_node",
          "description": "CPU ..Throttled",
          "cpu": "percentage",
          "counter": "100",
          "cost_of_consumption": "2000 USD",
          "health": "green/red/yellow"
        }

]
    },
    "osusages": {
        "title": "Operating systems consumed",
        "from_date": "2001-01-11:10:1010Z",
        "to_date": "2011-01-11:10:1010Z",
        "cumulative" :  {
          "cpu" : "percentage",
          "counter" : "90",
          "alerts": "no"
        },
        "item": {
          "name": "name_of_the_os",
          "cpu": [
          {  "1504157541.068": "276.88" },
          {  "1504157541.068": "276.88" }
        ]
      }
    }

  }
}




        let type_gua: nodesrv::Guages = serde_json::from_str(gua).unwrap();
        response.set_guages(type_gua);

        // let sta = "{
        //     \"title\":\"Statistics of the nodes\",
        //     \"nodes\":[
        //     {
        //         \"name\":\"name_of_the_node\",
        //         \"description\":\"CPU ..Throttled\",
        //         \"cpu\":\"percentage\",
        //         \"counter\":\"100\",
        //         \"cost_of_consumption\":\"2000 USD\",
        //         \"health\":\"green/red/yellow\"
        //     },
        //     {
        //         \"name\":\"name_of_the_node\",
        //         \"description\":\"CPU ..Throttled\",
        //         \"cpu\":\"percentage\",
        //         \"counter\":\"100\",
        //         \"cost_of_consumption\":\"2000 USD\",
        //         \"health\":\"green/red/yellow\"
        //     }
        //     ]
        // },";
        // let type_sta: nodesrv::Statistics = serde_json::from_str(sta).unwrap();
        // response.set_statistics(type_sta);

        // let os = "{
        //     \"title\":\"Operating systems consumed\",
        //     \"from_date\":\"2001-01-11:10:1010Z\",
        //     \"to_date\":\"2011-01-11:10:1010Z\",
        //     \"cumulative\":{
        //         \"cpu\":\"percentage\",
        //         \"counter\":\"90\",
        //         \"alerts\":\"no\"
        //     },
        //     \"item\":{
        //         \"name\":\"name_of_the_os\",
        //         \"cpu\":{
        //
        //             \"1504157541.068\":\"276.88\",
        //
        //             \"1504157541.068\":\"276.88\",
        //     }
        //     }
        // }";

        // let mut res = nodesrv::Item::new();
        // res.set_cpu("vino".to_string(), "hai".to_string());
        //
        //
        // let type_os: nodesrv::Osusages = serde_json::from_str(os).unwrap();
        // response.set_osusages(type_os);
        // println!("---------------------------------------{:?}", response);

        //add what you want to do here.
        Ok(contents)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Contents {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: usize,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub content: String,
    pub encoding: String,
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
fn hyper_client() -> hyper::Client {
    let ssl = OpensslClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = hyper::Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client.set_write_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client
}
//
fn hyper_to_net_err(err: hyper::error::Error) -> net::NetError {
    net::err(net::ErrCode::BAD_REMOTE_REPLY, err.description())
}
