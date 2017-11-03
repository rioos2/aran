// Copyright (c) 2017 RioCorp Inc.
//


#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rioos_builder_protocol as protocol;
extern crate rioos_core as rioos_core;
extern crate rioos_http_client as rioos_http;
extern crate rioos_net as rio_net;


extern crate broadcast;
#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate tee;
extern crate url;

pub mod error;
pub use error::{Error, Result};

use std::io::{self, Read, Write};
use std::path::Path;
use std::string::ToString;

use rioos_http::ApiClient;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::status::StatusCode;
use hyper::header::{ContentType, Accept, Authorization, Bearer, Headers};
use protocol::net::NetError;
use rand::{Rng, thread_rng};
use url::percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};
use protocol::{sessionsrv, asmsrv, nodesrv, plansrv, storagesrv, originsrv, jobsrv,netsrv};
use rioos_http::util::decoded_response;
use rio_net::http::headers::*;


header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }

const DEFAULT_API_PATH: &'static str = "/api/v1";


/// Custom conversion logic to allow `serde` to successfully
/// round-trip `u64` datatypes through JSON serialization.
///
/// To use it, add `#[serde(with = "json_u64")]` to any `u64`-typed struct
/// fields.
mod json_u64 {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", num);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

pub trait DisplayProgress: Write {
    fn size(&mut self, size: u64);
    fn finish(&mut self);
}

pub struct Client(ApiClient);

impl Client {
    pub fn new<U>(endpoint: U, product: &str, version: &str, fs_root_path: Option<&Path>) -> Result<Self>
    where
        U: IntoUrl,
    {
        let mut endpoint = endpoint.into_url()?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }
        Ok(Client(
            ApiClient::new(endpoint, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        ))
    }

    pub fn login(&self, userid: &str, password: &str) -> Result<(String)> {
        debug!("Logging in for {}", userid);
        let url = format!("authenticate");

        let body = json!({
            "email": format!("{}", userid),
            "password": format!("{}", password)
        });


        let sbody = serde_json::to_string(&body).unwrap();

        let res = self.0
            .post(&url)
            .body(&sbody)
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to login, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<sessionsrv::Session>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => Ok(value.get_token()),
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }

    pub fn logout(&self, token: &str) -> Result<(String)> {
        debug!("Logout for {}", token);
        let url = format!("logout/{}", token);
        let res = self.add_authz(self.0.get(&url), token).send().map_err(
            Error::HyperError,
        )?;

        if res.status != StatusCode::Ok {
            debug!("Failed to logout, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        Ok("".to_string())
    }


    pub fn list_deploy(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("assemblyfactorys");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get AssemblyFactory, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<asmsrv::AssemblyFactoryGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(), i.get_name(), i.get_replicas().to_string(),
                             i.get_properties().clone().get_region(), i.get_origin(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }


    pub fn describe_deploy(&self, token: &str, email: &str, id: &str) -> Result<asmsrv::AssemblyFactory> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("assemblyfactorys/{}",id);

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get AssemblyFactory, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<asmsrv::AssemblyFactory>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => Ok(value),
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }


    pub fn get_assembly_by_id(&self, token: &str, email: &str, id: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("/assemblyfactorys/{}/describe",id);

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get Assembly, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<asmsrv::AssemblysGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(), i.get_name(), i.get_status().get_phase(),
                             i.get_origin(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }

    pub fn list_node(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("nodes");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get nodes, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<nodesrv::NodeGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(), i.get_status().get_phase(),i.get_spec().get_unschedulable().to_string(),
                             i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }

    pub fn list_image(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("plans");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get images, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<plansrv::PlanGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(),i.get_group_name(),i.get_url(),i.get_origin(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }
    pub fn list_datacenters(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("datacenters");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get datacenter, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<storagesrv::DcGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(),i.get_name(),i.get_nodes().iter().cloned().collect(),i.get_networks().iter().cloned().collect(),i.get_storage(),i.get_enabled().to_string(),i.get_status().get_phase(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }


    pub fn list_origins(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("origins");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get images, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<originsrv::OriginGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(),i.get_object_meta().get_origin(),i.get_object_meta().get_uid(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }
    pub fn list_job(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("jobs");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get jobs, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<jobsrv::JobGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(),i.get_spec().get_node_id(),i.get_spec().get_target_ref(),i.get_status().get_phase(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }
    pub fn list_network(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        debug!("Token {}", token);
        debug!("Email {}", email);
        let url = format!("networks");

        let res = self.add_authz(self.0.get(&url), token)
            .header(Accept::json())
            .header(ContentType::json())
            .header(XAuthRioOSEmail(email.to_string()))
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::Ok {
            debug!("Failed to get network, status: {:?}", res.status);
            return Err(err_from_response(res));
        };

        match decoded_response::<netsrv::NetworkGetResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => {
                Ok(
                    value
                        .get_items()
                        .iter_mut()
                        .map(|i| {
                            vec![i.get_id(),i.get_name(),i.get_network_type(),i.get_subnet_ip(),i.get_netmask(),i.get_gateway(),i.get_status().get_phase(),i.get_created_at()]
                        })
                        .collect(),
                )
            }
            Err(e) => {
                debug!("Failed to decode response, err: {:?}", e);
                return Err(e);
            }
        }

    }
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    /*TO-DO: KISHORE
    pub fn deploy_digicloud(&self, ident: &PackageIdent, token: &str) -> Result<(String)> {
        debug!("Creating a job for {}", ident);

        let body = json!({
            "project_id": format!("{}", ident)
        });

        let sbody = serde_json::to_string(&body).unwrap();

        let result = self.add_authz(self.0.post("jobs"), token)
            .body(&sbody)
            .header(Accept::json())
            .header(ContentType::json())
            .send();
        match result {
            Ok(mut response) => {
                match response.status {
                    StatusCode::Created => {
                        let mut encoded = String::new();
                        response.read_to_string(&mut encoded).map_err(Error::IO)?;
                        debug!("Body: {:?}", encoded);
                        let v: serde_json::Value =
                            serde_json::from_str(&encoded).map_err(Error::Json)?;
                        let id = v["id"].as_str().unwrap();
                        Ok(id.to_string())
                    }
                    StatusCode::Unauthorized => {
                        Err(Error::APIError(
                            response.status,
                            "Your GitHub token requires both user:email and read:org \
                                             permissions."
                                .to_string(),
                        ))
                    }
                    _ => Err(err_from_response(response)),
                }
            }
            Err(e) => Err(Error::HyperError(e)),
        }
    }*/


    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
        // rb.header(Authorization(Bearer { token: token.to_string() }))
    }
}

fn err_from_response(mut response: hyper::client::Response) -> Error {
    if response.status == StatusCode::Unauthorized {
        return Error::APIError(
            response.status,
            "Your token mismatch and requires permissions.".to_string(),
        );
    }

    let mut buff = String::new();
    match response.read_to_string(&mut buff) {
        Ok(_) => {
            match serde_json::from_str::<NetError>(&buff) {
                Ok(err) => Error::APIError(response.status, err.to_string()),
                Err(_) => Error::APIError(response.status, buff),
            }
        }
        Err(_) => {
            buff.truncate(0);
            Error::APIError(response.status, buff)
        }
    }
}

fn origin_keys_path(origin: &str) -> String {
    format!("depot/origins/{}/keys", origin)
}

fn origin_secret_keys_latest(origin: &str) -> String {
    format!("depot/origins/{}/secret_keys/latest", origin)
}


fn package_search(term: &str) -> String {
    let encoded_term = percent_encode(term.as_bytes(), PATH_SEGMENT_ENCODE_SET);
    format!("depot/pkgs/search/{}", encoded_term)
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn json_round_trip_u64_fields() {
        let pre = OriginSecretKey {
            id: 705705315793903646,
            origin_id: 705705305031319582,
            name: "core".to_string(),
            revision: "20160810182414".to_string(),
            body: vec![1, 2, 3],
            owner_id: 0,
        };

        let as_json = serde_json::to_value(&pre).unwrap();
        let expected = json!({
            "id": "705705315793903646",
            "origin_id": "705705305031319582",
            "name": "core",
            "revision": "20160810182414",
            "body": [
                1,
                2,
                3
            ],
            "owner_id": "0"
        });
        assert_eq!(as_json, expected);

    }
}
