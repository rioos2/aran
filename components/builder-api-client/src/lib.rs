// Copyright 2018 The Rio Advancement Inc
//

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_httpgateway as http_gateway;
extern crate rioos_http_client as rioos_http;

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

use reqwest::header::{Accept, Authorization, Bearer, ContentType, Headers, UserAgent};
use reqwest::IntoUrl;
use reqwest::{Body, StatusCode};

use rioos_http::api_client::err_from_response;
use rioos_http::ApiClient;

use http_gateway::http::rendering::ResponseList;

use protocol::api::base::{hours_ago, MetaFields};
use protocol::api::{
    blueprint, deploy, devtool, job, network, node, origin, scale, secret, session, storage,
};

const DEFAULT_API_PATH: &'static str = "/api/v1";
const USER_AGENT: &'static str = "Rio/OS Blu";

header! { (XAuthRioOSEmail, "X-AUTH-RIOOS-EMAIL") => [String] }

#[derive(Debug)]
pub struct Client(ApiClient);

impl Client {
    pub fn new<U>(
        endpoint: U,
        product: &str,
        version: &str,
        fs_root_path: Option<&Path>,
    ) -> Result<Self>
    where
        U: IntoUrl,
    {
        let mut endpoint = endpoint.into_url()?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }

        Ok(Client(ApiClient::new(
            endpoint,
            product,
            version,
            fs_root_path,
        ).map_err(Error::RioHttpClient)?))
    }

    pub fn signup(&self, body: session::SessionCreate) -> Result<session::Session> {
        let mut res = self.0
            .post(&format!("accounts"))
            .body(Body::from(serde_json::to_string(&body)?))
            .header(Accept::json())
            .header(ContentType::json())
            .header(UserAgent::new(USER_AGENT.to_string()))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
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
            .header(UserAgent::new(USER_AGENT.to_string()))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let data: session::Session = res.json()?;
        Ok(data)
    }

    pub fn logout(&self, token: &str, email: &str) -> Result<()> {
        let body = json!({
            "email": format!("{}", email),
            "token": format!("{}", token)
        });
        let res = self.0
            .post(&format!("logout"))
            .body(Body::from(serde_json::to_string(&body)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn deploy_digicloud(
        &self,
        assembly_fac: deploy::StacksFactory,
        token: &str,
        email: &str,
    ) -> Result<deploy::StacksFactory> {
        let mut res = self.0
            .post(&format!(
                "accounts/{}/stacksfactorys",
                assembly_fac.object_meta().account
            ))
            .body(Body::from(serde_json::to_string(&assembly_fac)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;
        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let stacks: deploy::StacksFactory = res.json()?;
        Ok(stacks)
    }

    pub fn list_deploy(&self, token: &str, email: &str, account: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("accounts/{}/assemblyfactorys", account))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;
        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut assemblyfactory: ResponseList<Vec<deploy::AssemblyFactory>> = res.json()?;

        Ok(assemblyfactory
            .items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_replicas().to_string(),
                    i.get_status().get_phase(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }

    pub fn describe_deploy(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<deploy::AssemblyFactory> {
        let mut res = self.0
            .get(&format!("assemblyfactorys/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let assembly: deploy::AssemblyFactory = res.json()?;
        Ok(assembly)
    }

    pub fn create_network(
        &self,
        network: network::Network,
        token: &str,
        email: &str,
    ) -> Result<network::Network> {
        let mut res = self.0
            .post(&format!("networks"))
            .body(Body::from(serde_json::to_string(&network)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;
        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let network: network::Network = res.json()?;
        Ok(network)
    }

    pub fn create_datacenter(
        &self,
        storage: storage::DataCenter,
        token: &str,
        email: &str,
    ) -> Result<()> {
        let res = self.0
            .post(&format!("datacenters"))
            .body(Body::from(serde_json::to_string(&storage)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn create_secret(
        &self,
        secret: secret::Secret,
        origin: &str,
        token: &str,
        email: &str,
    ) -> Result<()> {
        let res = self.0
            .post(&format!("origins/{}/secrets", origin))
            .body(Body::from(serde_json::to_string(&secret)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn create_build_config(
        &self,
        build_config: devtool::BuildConfig,
        token: &str,
        email: &str,
    ) -> Result<()> {
        let res = self.0
            .post(&format!("buildconfigs"))
            .body(Body::from(serde_json::to_string(&build_config)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn list_secret(&self, token: &str, email: &str, account: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("accounts/{}/secrets", account))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut secret: ResponseList<Vec<secret::Secret>> = res.json()?;
        Ok(secret
            .items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_secret_type(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }

    pub fn describe_secret(&self, token: &str, email: &str, id: &str) -> Result<secret::Secret> {
        let mut res = self.0
            .get(&format!("secrets/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let secret: secret::Secret = res.json()?;
        Ok(secret)
    }

    pub fn create_horizontal_scaling(
        &self,
        hscale: scale::HorizontalScaling,
        token: &str,
        email: &str,
    ) -> Result<()> {
        let res = self.0
            .post(&format!("horizontalscaling"))
            .body(Body::from(serde_json::to_string(&hscale)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn create_vertical_scaling(
        &self,
        vscale: scale::VerticalScaling,
        token: &str,
        email: &str,
    ) -> Result<()> {
        let res = self.0
            .post(&format!("verticalscaling"))
            .body(Body::from(serde_json::to_string(&vscale)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn get_assembly_by_id(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("assemblyfactorys/{}/describe", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut assembly: ResponseList<Vec<deploy::Assembly>> = res.json()?;

        Ok(assembly
            .items
            .iter_mut()
            .map(|i| {
                let ips_ports = (match i.get_spec().get_endpoints() {
                    None => None,
                    Some(endpoint) => {
                        let subsets = endpoint.get_subsets();
                        Some((
                            subsets
                                .get_addresses()
                                .clone()
                                .iter_mut()
                                .map(|x| x.ip.to_owned())
                                .collect::<Vec<_>>(),
                            subsets
                                .get_ports()
                                .clone()
                                .iter_mut()
                                .map(|x| x.port.to_owned())
                                .collect::<Vec<_>>(),
                        ))
                    }
                }).unwrap_or(([].to_vec(), [].to_vec()));

                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.object_meta().account,
                    ips_ports.0.into_iter().collect(),
                    ips_ports.1.into_iter().collect(),
                    i.get_status().get_phase(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }
    pub fn list_node(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("nodes"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut node: ResponseList<Vec<node::Node>> = res.json()?;
        Ok(node.items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_status().get_phase(),
                    (!i.get_spec().get_unschedulable()).to_string(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }

    pub fn node_describe(&self, token: &str, email: &str, id: &str) -> Result<node::Node> {
        let mut res = self.0
            .get(&format!("/nodes/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
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
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut plan: ResponseList<Vec<blueprint::Plan>> = res.json()?;
        Ok(plan.items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_category(),
                    i.get_version(),
                    i.get_description(),
                    i.get_status().get_phase(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }
    pub fn list_datacenters(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("datacenters"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut datacenter: ResponseList<Vec<storage::DataCenter>> = res.json()?;
        Ok(datacenter
            .items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_enabled().to_string(),
                    i.get_status().get_phase(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }

    pub fn list_origins(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("origins"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut origin: ResponseList<Vec<origin::Origin>> = res.json()?;
        Ok(origin
            .items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.get_name(),
                    i.object_meta().account,
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }
    pub fn list_job(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("jobs"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut job: ResponseList<Vec<job::Jobs>> = res.json()?;
        Ok(job.items
            .iter_mut()
            .map(|i| {
                vec![
                    i.get_id(),
                    i.object_meta().name,
                    i.get_spec().get_node_id(),
                    i.get_status().get_phase(),
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }
    pub fn list_network(&self, token: &str, email: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("networks"))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let mut network: ResponseList<Vec<network::Network>> = res.json()?;
        Ok(network
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
                    hours_ago(i.get_created_at()),
                ]
            })
            .collect())
    }

    pub fn origin_get(&self, token: &str, email: &str, name: &str) -> Result<Vec<Vec<String>>> {
        let mut res = self.0
            .get(&format!("/origins/{}", name))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let result: origin::Origin = res.json()?;
        let data = vec![vec![
            result.get_id(),
            result.get_name(),
            result.object_meta().account,
            hours_ago(result.get_created_at()),
        ]];
        Ok(data)
    }
    pub fn datacenter_get_by_id(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<storage::DataCenter> {
        let mut res = self.0
            .get(&format!("/datacenters/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let dc: storage::DataCenter = res.json()?;
        Ok(dc)
    }

    pub fn network_get_by_id(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<network::Network> {
        let mut res = self.0
            .get(&format!("/networks/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let net: network::Network = res.json()?;
        Ok(net)
    }
    pub fn network_update(&self, token: &str, email: &str, net: network::Network) -> Result<()> {
        let res = self.0
            .put(&format!("networks/{}", net.get_id()))
            .body(Body::from(serde_json::to_string(&net)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn datacenter_update(
        &self,
        token: &str,
        email: &str,
        dc: storage::DataCenter,
    ) -> Result<()> {
        let res = self.0
            .put(&format!("datacenters/{}", dc.get_id()))
            .body(Body::from(serde_json::to_string(&dc)?))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        Ok(())
    }

    pub fn describe_datacenter(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<storage::DataCenter> {
        let mut res = self.0
            .get(&format!("/datacenters/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let datacenter: storage::DataCenter = res.json()?;
        Ok(datacenter)
    }

    pub fn get_storageconnector_by_id(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<storage::Storage> {
        let mut res = self.0
            .get(&format!("/storageconnectors/{}", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
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
            return Err(Error::RioHttpClient(err_from_response(res)));
        };

        let strcon: ResponseList<Vec<storage::Storage>> = res.json()?;
        Ok(strcon.items)
    }

    pub fn get_storagepool_by_scid(
        &self,
        token: &str,
        email: &str,
        id: &str,
    ) -> Result<Vec<storage::StoragePool>> {
        let mut res = self.0
            .get(&format!("/storageconnectors/{}/storagespool", id))
            .headers(self.add_authz(token, email))
            .send()
            .map_err(Error::ReqwestError)?;

        if res.status() != StatusCode::Ok {
            return Err(Error::RioHttpClient(err_from_response(res)));
        };
        let strpool: ResponseList<Vec<storage::StoragePool>> = res.json()?;
        Ok(strpool.items)
    }

    fn add_authz(&self, token: &str, email: &str) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer {
            token: token.to_string(),
        }));
        headers.set(XAuthRioOSEmail(email.to_string()));
        headers.set(UserAgent::new(USER_AGENT.to_string()));
        headers.set(ContentType::json());
        headers.set(Accept::json());
        headers
    }
}
