use std::path::PathBuf;
use serde_yaml;
use chrono::prelude::*;

use config::Config;
use error::{Result, Error};

use reqwest::header::{ContentType, Accept, Authorization, Bearer};
use reqwest::StatusCode;

use rioos_http::ApiClient as ReqwestClient;
use http_gateway::http::rendering::ResponseList;
use rioos_http::api_client::err_from_response;

use rio_core::fs::{write_to_file, rioconfig_config_path, append};

use protocol::api::marketplace;
use protocol::api::base::MetaFields;

use common::ui::UI;

header! { (XAuthRioOSEmail, "X-AUTH-RIOOS-EMAIL") => [String] }

const IMAGE_URL: &'static str = "rioos_sh_image_url";

lazy_static! {
    static  ref MARKETPLACE_FILE: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("pullcache/marketplaces.yaml").to_str().unwrap());
    static  ref SERVER_CERTIFICATE:  PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("client-marketplaces.cert.pem").to_str().unwrap());
}

pub fn start(ui: &mut UI, config: &Config) -> Result<()> {
    ui.br()?;

    ui.para(&format!(
        "Sync from Rio MarketPlace - {} ...",
        &config.marketplaces.endpoint
    ))?;

    let client = ReqwestClient::new(
        &config.marketplaces.endpoint,
        "rioos",
        "v1",
        Some(&SERVER_CERTIFICATE),
    ).map_err(Error::RioHttpClient)?;

    let url = format!("marketplaces");

    let mut res = client
        .get(&url)
        .header(Accept::json())
        .header(Authorization(
            Bearer { token: config.marketplaces.token.to_owned() },
        ))
        .header(XAuthRioOSEmail(config.marketplaces.username.to_string()))
        .header(ContentType::json())
        .send()
        .map_err(Error::ReqwestError)?;

    if res.status() != StatusCode::Ok {
        return Err(Error::RioHttpClient(err_from_response(res)));
    };

    let market: ResponseList<Vec<marketplace::MarketPlace>> = res.json()?;

    MarketPlaceSaver::new(market, &config.marketplaces.endpoint)
        .with_url()?;

    append(
        &MARKETPLACE_FILE,
        &("\ntime_stamp: ".to_string() + &Utc::now().to_rfc3339()),
    )?;

    ui.para(&format!("Sync complete. Start firing up Rio/OS!"))?;
    Ok(())
}

struct MarketPlaceSaver<'a> {
    content: ResponseList<Vec<marketplace::MarketPlace>>,
    endpoint: &'a str,
}

impl<'a> MarketPlaceSaver<'a> {
    fn new(marketplaces: ResponseList<Vec<marketplace::MarketPlace>>, endpoint: &'a str) -> Self {
        MarketPlaceSaver {
            content: marketplaces,
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
                plan.iter_mut().map(|y| {
                    let mut data = y.get_characteristics().clone();
                    let owner_reference = y.object_meta().owner_references
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
                })
                .collect::<Vec<_>>();
            })
            .collect::<Vec<_>>();

        Ok(self.save()?)
    }

    fn save(&self) -> Result<()> {
        let encoded = serde_yaml::to_string(&self.content)?;
        write_to_file(&MARKETPLACE_FILE, &encoded)?;
        Ok(())
    }
}
