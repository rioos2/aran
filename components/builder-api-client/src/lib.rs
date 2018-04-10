// Copyright 2018 The Rio Advancement Inc
//

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_http_client as rioos_http;
extern crate rioos_net as rio_net;

#[macro_use]
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
extern crate url;

pub mod error;
pub use error::{Error, Result};

use std::path::Path;
use std::string::ToString;

use reqwest::IntoUrl;
use reqwest::{StatusCode, Body};
use reqwest::header::{ContentType, Accept, Authorization, Bearer, Headers};

use rioos_http::ApiClient;
use rio_net::http::rendering::ResponseList;
use rio_net::util::errors::err_from_response;

use protocol::api::{session, deploy, blueprint, job, network, node, storage, origin, scale, secret};
use protocol::api::base::MetaFields;

const DEFAULT_API_PATH: &'static str = "/api/v1";

header! { (XAuthRioOSEmail, "X-AUTH-RIOOS-EMAIL") => [String] }

#[derive(Debug)]
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
                .map_err(Error::RioHttpClient)?,
        ))
    }

    pub fn signup(&self, body: session::SessionCreate) -> Result<session::Session> {
        let mut res = self.0
            .post(&format!("accounts"))
            .body(Body::from(serde_json::to_string(&body)?))
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let data: session::Session = res.json()?;
        Ok(data)
    }

    pub fn login(&self, userid: &str, password: &str) -> Result<session::Session> {
        let body = json!({
            "email": format!("{}", userid),
            "password": format!("{}", password)
        });
        let mut res = self.0
            .post(&format!("authenticate"))
            .body(Body::from(serde_json::to_string(&body)?))
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let data: session::Session = res.json()?;
        Ok(data)
    }

    pub fn logout(&self, token: &str) -> Result<(String)> {
        let res = self.0.get(&format!("logout/{}", token)).send().map_err(
            Error::ReqwestError,
        )?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        Ok("".to_string())
    }

    pub fn deploy_digicloud(&self, assembly_fac: deploy::AssemblyFactory, token: &str, email: &str) -> Result<deploy::AssemblyFactory> {
        let mut res = self.0
            .post(
                &format!("accounts/{}/assemblyfactorys",assembly_fac.object_meta().account),
            )
            .body(Body::from(serde_json::to_string(&assembly_fac)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;
        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };
        let assembly_fat: deploy::AssemblyFactory = res.json()?;
        Ok(assembly_fat)
    }


    pub fn list_deploy(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("assemblyfactorys"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;
        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut assemblyfactory: ResponseList<Vec<deploy::AssemblyFactory>> = res.json()?;

        Ok(
            assemblyfactory
                .items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_replicas().to_string(),
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }

    pub fn describe_deploy(&self, token: &str, email: &str, id: &str) -> Result<deploy::AssemblyFactory> {
        let mut res = self.0
            .get(&format!("assemblyfactorys/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let assembly: deploy::AssemblyFactory = res.json()?;
        Ok(assembly)
    }



    pub fn create_network(&self, network: network::Network, token: &str, email: &str) -> Result<network::Network> {
        let mut res = self.0
            .post(&format!("networks"))
            .body(Body::from(serde_json::to_string(&network)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;
        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let network: network::Network = res.json()?;
        Ok(network)
    }

    pub fn create_datacenter(&self, storage: storage::DataCenter, token: &str, email: &str) -> Result<()> {
        let res = self.0
            .post(&format!("datacenters"))
            .body(Body::from(serde_json::to_string(&storage)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        Ok(())
    }

    pub fn create_secret(&self, secret: secret::Secret, origin: &str, token: &str, email: &str) -> Result<()> {
        let res = self.0
            .post(&format!("origins/{}/secrets",origin))
            .body(Body::from(serde_json::to_string(&secret)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        Ok(())
    }


    pub fn create_horizontal_scaling(&self, hscale: scale::HorizontalScaling, token: &str, email: &str) -> Result<()> {
        let res = self.0
            .post(&format!("horizontalscaling"))
            .body(Body::from(serde_json::to_string(&hscale)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        Ok(())
    }



    pub fn get_assembly_by_id(&self, token: &str, email: &str, id: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("assemblyfactorys/{}/describe", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut assembly: ResponseList<Vec<deploy::Assembly>> = res.json()?;

        Ok(
            assembly
                .items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.object_meta().account,
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }
    pub fn list_node(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("nodes"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };
        let mut node: ResponseList<Vec<node::Node>> = res.json()?;
        Ok(
            node.items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_status().get_phase(),
                    i.get_spec().get_unschedulable().to_string(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }

    pub fn node_describe(&self, token: &str, email: &str, id: &str) -> Result<node::Node> {
        let mut res = self.0
            .get(&format!("/nodes/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let node: node::Node = res.json()?;
        Ok(node)
    }
    pub fn list_image(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("plans"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut plan: ResponseList<Vec<blueprint::Plan>> = res.json()?;
        Ok(
            plan.items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_category(),
                    i.get_version(),
                    i.get_description(),
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }
    pub fn list_datacenters(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("datacenters"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut datacenter: ResponseList<Vec<storage::DataCenter>> = res.json()?;
        Ok(
            datacenter
                .items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_enabled().to_string(),
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }

    pub fn list_origins(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {

        let mut res = self.0
            .get(&format!("origins"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut origin: ResponseList<Vec<origin::Origin>> = res.json()?;
        Ok(
            origin
                .items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.get_name(),
                    i.object_meta().account,
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }
    pub fn list_job(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("jobs"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut job: ResponseList<Vec<job::Jobs>> = res.json()?;
        Ok(
            job.items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_spec().get_node_id(),
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }
    pub fn list_network(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("networks"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let mut network: ResponseList<Vec<network::Network>> = res.json()?;
        Ok(
            network
                .items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_network_type(),
                    i.get_subnet_ip(),
                    i.get_netmask(),
                    i.get_gateway(),
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }

    pub fn origin_get(&self, token: &str, email: &str, name: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("/origins/{}", name))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let result: origin::Origin = res.json()?;
        let data = vec![
            vec![
                result.get_id(),
                result.get_name(),
                result.object_meta().account,
                result.get_created_at(),
            ],
        ];
        Ok(data)
    }
    pub fn datacenter_get_by_id(&self, token: &str, email: &str, id: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("/datacenters/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let dc: storage::DataCenter = res.json()?;
        let data = vec![
            vec![
                dc.get_id(),
                dc.object_meta().name,
                dc.get_enabled().to_string(),
                dc.get_status().get_phase(),
                dc.get_created_at(),
            ],
        ];
        Ok(data)
    }

    pub fn describe_datacenter(&self, token: &str, email: &str, id: &str) -> Result<storage::DataCenter> {
        let mut res = self.0
            .get(&format!("/datacenters/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let datacenter: storage::DataCenter = res.json()?;
        Ok(datacenter)
    }

    pub fn get_storageconnector_by_id(&self, token: &str, email: &str, id: &str) -> Result<storage::Storage> {
        let mut res = self.0
            .get(&format!("/storageconnectors/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let strcon: storage::Storage = res.json()?;
        Ok(strcon)
    }
    pub fn get_storageconnector(&self, token: &str, email: &str) -> Result<Vec<storage::Storage>> {
        let mut res = self.0
            .get(&format!("storageconnectors"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let strcon: ResponseList<Vec<storage::Storage>> = res.json()?;
        Ok(strcon.items)
    }

    pub fn get_storagepool_by_scid(&self, token: &str, email: &str, id: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("/storagespool/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };
        let mut strpool: ResponseList<Vec<storage::StoragePool>> = res.json()?;
        Ok(
            strpool
                .items
                .iter_mut()
                .map(|i| {
                    vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_status().get_phase(),
                    i.get_created_at(),
                ]
                })
                .collect(),
        )
    }
    pub fn get_storagepool_by_id(&self, token: &str, email: &str, id: &str) -> Result<Vec<storage::StoragePool>> {
        let mut res = self.0
            .get(&format!("/storageconnectors/{}/storagespool", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioNetError(err_from_response(res)));
        };

        let strpool: ResponseList<Vec<storage::StoragePool>> = res.json()?;
        Ok(strpool.items)
    }


    fn add_authz(&self, token: &str, email: &str) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: token.to_string() }));
        headers.set(XAuthRioOSEmail(email.to_string()));
        headers.set(ContentType::json());
        headers.set(Accept::json());
        headers
    }
}
