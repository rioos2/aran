use std::path::PathBuf;
use std::collections::BTreeMap;

use iron;
use iron::prelude::*;
use iron::headers::{Authorization, Bearer};

use super::headers::*;
use util::errors::{bad_err, not_acceptable_error};
use http::rendering::render_json_error;

use auth::util::authenticatable::Authenticatable;

pub trait HeaderExtracter {
    const AUTH_CONF_NAME: &'static str;

    fn exists_conf_key() -> Option<String> {
        if !Self::AUTH_CONF_NAME.trim().is_empty() {
            return Some(Self::AUTH_CONF_NAME.to_string());
        }
        None
    }

    fn extract(req: iron::Headers, token: String, config_value: Option<&String>) -> Option<Authenticatable>;
}

struct EmailHeader {}

impl HeaderExtracter for EmailHeader {
    const AUTH_CONF_NAME: &'static str = "email";

    fn extract(req: iron::Headers, token: String, config_value: Option<&String>) -> Option<Authenticatable> {
        let email = req.get::<XAuthRioOSEmail>();

        if !email.is_none() {
            return Some(Authenticatable::UserEmailAndToken {
                email: email.unwrap().0.clone(),
                token: token,
            });
        }
        None
    }
}

struct ServiceAccountHeader {}

impl HeaderExtracter for ServiceAccountHeader {
    const AUTH_CONF_NAME: &'static str = "service_account";

    fn extract(req: iron::Headers, token: String, config_value: Option<&String>) -> Option<Authenticatable> {
        let serviceaccount = req.get::<XAuthRioOSServiceAccountName>();
        if !serviceaccount.is_none() {
            return Some(Authenticatable::ServiceAccountNameAndWebtoken {
                name: serviceaccount.unwrap().0.clone(),
                webtoken: token,
                key: PathBuf::from(config_value.unwrap_or(&"service-account.pub".to_string())),
            });
        }
        None
    }
}

struct EmailWithJWTTokenHeader {}

impl HeaderExtracter for EmailWithJWTTokenHeader {
    const AUTH_CONF_NAME: &'static str = "email_jwt";

    fn extract(req: iron::Headers, token: String, config_value: Option<&String>) -> Option<Authenticatable> {
        let useraccount = req.get::<XAuthRioOSUserAccountEmail>();
        if !useraccount.is_none() {
            return Some(Authenticatable::UserEmailAndWebtoken {
                email: useraccount.unwrap().0.clone(),
                webtoken: token,
            });
        }
        None
    }
}

struct PassTicketHeader {}

impl HeaderExtracter for PassTicketHeader {
    const AUTH_CONF_NAME: &'static str = "passticket";

    fn extract(req: iron::Headers, _token: String, config_value: Option<&String>) -> Option<Authenticatable> {
        let otp = req.get::<XAuthRioOSOTP>();
        if !otp.is_none() {
            return Some(Authenticatable::PassTicket {
                token: otp.unwrap().0.clone(),
            });
        }
        None
    }
}

//Decide the header to validate the header and access
pub struct HeaderDecider {
    extractables: Vec<Option<Authenticatable>>,
}

impl HeaderDecider {
    pub fn new(req_headers: iron::Headers, conf_map: BTreeMap<String, String>) -> IronResult<Self> {
        let req = req_headers.clone();
        let token = match req.get::<Authorization<Bearer>>() {
            Some(&Authorization(Bearer { ref token })) => token,
            _ => {
                let err = not_acceptable_error(&format!("Authorization Bearer: token not found."));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        };
    
        let extractables = vec![
            EmailHeader::extract(
                req.clone(),
                token.to_string(),
                EmailHeader::exists_conf_key().and_then(|x| conf_map.get(&x)),
            ),
            ServiceAccountHeader::extract(
                req.clone(),
                token.to_string(),
                ServiceAccountHeader::exists_conf_key().and_then(|x| conf_map.get(&x)),
            ),
            EmailWithJWTTokenHeader::extract(
                req.clone(),
                token.to_string(),
                EmailWithJWTTokenHeader::exists_conf_key().and_then(|x| conf_map.get(&x)),
            ),
            PassTicketHeader::extract(
                req.clone(),
                token.to_string(),
                EmailWithJWTTokenHeader::exists_conf_key().and_then(|x| conf_map.get(&x)),
            ),
        ];

        Ok(HeaderDecider {
            extractables: extractables,
        })
    }

    pub fn decide(&self) -> IronResult<Authenticatable> {
        let validate = valid_header(&self.extractables);

        if validate.is_some() {
            return Ok(validate.unwrap());
        }
        let err = not_acceptable_error(&format!(
            "Authentication not supported. You must have headers for the supported authetication. Refer https://www.rioos.sh/admin/auth."
        ));
        return Err(render_json_error(&bad_err(&err), err.http_code()));
    }
}

fn valid_header(extractable_res: &Vec<Option<Authenticatable>>) -> Option<Authenticatable> {
    let validated = extractable_res
        .iter()
        .fold(None, |acc, x| acc.or(x.clone()).clone());
    validated
}
