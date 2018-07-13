// Copyright 2018 The Rio Advancement Inc
//

use config;
use error::{Error, Result};
use http_client::reqwest_client::http_bearer_get;
use serde_json::{self, Value};
use std::io::Read;

const ENDPOINT: &'static str = "https://secure.licenseapi.com";
const SKU: &'static str = "rioos";
const TOKEN: &'static str = "1317caffd6874b5f81bcea8142f27cb5c42e9db6";
const FORMAT: &'static str = "JSON";

const CLOUD_LIB_OPEN: &str = "CloudLibOpen";
const CLOUD_EXPIRED: &str = "EXPIRED";
const CLOUD_ACTIVATION_CODE: &str = "ActivationCode";

#[derive(Debug)]
pub struct LicenseCloud {
    fascade: API,
}

impl LicenseCloud {
    pub fn new<T: config::License>(config: &T) -> Self {
        LicenseCloud { fascade: API::new(config.activation_code()) }
    }

    // Returns the status of license verified with LicenseCloud
    pub fn verify(&self) -> Result<()> {
        self.fascade.check_license()?;
        Ok(())
    }
}

#[derive(Debug)]
struct API {
    activation_code: Option<String>,
}

impl API {
    fn new(activation_code: Option<String>) -> Self {
        API { activation_code: activation_code }
    }

    fn check_license(&self) -> Result<()> {
        let code = match self.activation_code {
            Some(ref ac) => ac,
            None => return LicenseCloudResult::from_err(CLOUD_ACTIVATION_CODE),
        };
        Self::call_dynamic(code.clone())?;
        Ok(())
    }

    fn call_dynamic(activation_code: String) -> Result<()> {
        let url = format!(
            "{}/?token={}&sku={}&license={}&format={}",
            ENDPOINT,
            TOKEN,
            SKU,
            activation_code,
            FORMAT
        );
        let path = "";
        let mut rep = match http_bearer_get(&url, &path) {
            Ok(res) => res,
            Err(_) => return LicenseCloudResult::from_err(CLOUD_LIB_OPEN),
        };
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        let v: Value = match serde_json::from_str(&body) {
            Ok(res) => res,
            Err(_) => return LicenseCloudResult::from_err(CLOUD_LIB_OPEN),
        };

        let error_num = v["licensecloud"]["error_num"].to_string().replace('"', "");
        if error_num != "0".to_string() {
            return LicenseCloudResult::from_value(v["licensecloud"]["error_desc"].to_string());
        }

        return LicenseCloudResult::from_err(&v["licensecloud"]["license"]["status"]
            .to_string()
            .replace('"', ""));
    }
}

enum LicenseCloudResult {}

impl LicenseCloudResult {
    pub fn from_value(desc: String) -> Result<()> {
        // Error can Generated based on licensecloud error code refer link: https://www.licensecloud.com/api-reference/error-codes/
        Err(Error::RioosAranCore(desc))
    }
    pub fn from_err(name: &str) -> Result<()> {
        match name {
            CLOUD_LIB_OPEN => Err(Error::LicenseAPINotFound),
            CLOUD_EXPIRED => Err(Error::ProductExpired),
            CLOUD_ACTIVATION_CODE => Err(Error::SubscriptionExpired),
            _ => Ok(()),
        }
    }
}
