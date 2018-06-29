use chrono::prelude::*;
use serde_yaml;
use std::path::PathBuf;

use config::Config;
use error::{Error, Result};

use reqwest::header::{Accept, Authorization, Bearer, ContentType};
use reqwest::StatusCode;

use http_gateway::http::rendering::ResponseList;
use rioos_http::api_client::err_from_response;
use rioos_http::ApiClient as ReqwestClient;

use rio_core::fs::{append, rioconfig_config_path, write_to_file};

use protocol::api::base::MetaFields;
use protocol::api::marketplace;

use common::ui::UI;

header! { (XAuthRioOSEmail, "X-AUTH-RIOOS-EMAIL") => [String] }

const IMAGE_URL: &'static str = "rioos_sh_image_url";

lazy_static! {
    static ref APPSTORES_FILE: PathBuf = PathBuf::from(&*rioconfig_config_path(None)
        .join("pullcache/appstores.yaml")
        .to_str()
        .unwrap());
    static ref SERVER_CERTIFICATE: PathBuf = PathBuf::from(&*rioconfig_config_path(None)
        .join("client-appstores.cert.pem")
        .to_str()
        .unwrap());
}

pub fn start(ui: &mut UI, config: &Config) -> Result<()> {
    ui.br()?;

    ui.para(&format!(
        "Sync from Rio MarketPlace - {} ...",
        &config.appstores.endpoint
    ))?;

    let client = ReqwestClient::new(
        &config.appstores.endpoint,
        "rioos",
        "v1",
        Some(&SERVER_CERTIFICATE),
    ).map_err(Error::RioHttpClient)?;

    let url = format!("appstores");

    let mut res = client
        .get(&url)
        .header(Accept::json())
        .header(Authorization(Bearer {
            token: config.appstores.token.to_owned(),
        }))
        .header(XAuthRioOSEmail(config.appstores.username.to_string()))
        .header(ContentType::json())
        .send()
        .map_err(Error::ReqwestError)?;

    if res.status() != StatusCode::Ok {
        return Err(Error::RioHttpClient(err_from_response(res)));
    };

    let markets: ResponseList<Vec<marketplace::MarketPlace>> = res.json()?;

    AppStoresSaver::new(markets, &config.appstores.endpoint).with_url()?;

    append(
        &APPSTORES_FILE,
        &("\ntime_stamp: ".to_string() + &Utc::now().to_rfc3339()),
    )?;

    ui.para(&format!("Sync complete. Start firing up Rio/OS!"))?;
    Ok(())
}

struct AppStoresSaver<'a> {
    content: ResponseList<Vec<marketplace::MarketPlace>>,
    endpoint: &'a str,
}

impl<'a> AppStoresSaver<'a> {
    fn new(appstores: ResponseList<Vec<marketplace::MarketPlace>>, endpoint: &'a str) -> Self {
        AppStoresSaver {
            content: appstores,
            endpoint: &*endpoint,
        }
    }

    fn with_url(&mut self) -> Result<()> {
        let endpoint = self.endpoint;
        self.content
            .items
            .iter_mut()
            .map(|x| {
                let mut plan = x.get_plan();
                plan.iter_mut()
                    .map(|y| {
                        let mut data = y.get_characteristics().clone();

                        let owner_reference = y.object_meta()
                            .owner_references
                            .iter_mut()
                            .map(|x| x.get_uid().to_string())
                            .collect::<String>();
                        data.insert(
                            IMAGE_URL.to_string(),
                            format!(
                                "{}/marketplaces/{}/download",
                                endpoint.to_string(),
                                owner_reference
                            ),
                        );
                        y.set_characteristics(data);
                        let mut volumes = y.get_stateful_volumes().clone();
                        volumes.iter_mut().map(|vol| {
                            let mut setting = vol.get_settingmap().clone();
                            setting.set_uri(format!(
                                "{}/marketplaces/{}/settingmap/{}",
                                endpoint.to_string(),
                                owner_reference,
                                vol.name
                            ));
                        });
                    })
                    .collect::<Vec<_>>();
            })
            .collect::<Vec<_>>();

        Ok(self.save()?)
    }

    fn save(&self) -> Result<()> {
        let encoded = serde_yaml::to_string(&self.content)?;
        write_to_file(&APPSTORES_FILE, &encoded)?;
        Ok(())
    }
}
