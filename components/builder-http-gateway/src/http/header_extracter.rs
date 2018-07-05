use std::collections::HashMap;
use std::path::PathBuf;

use iron;
use iron::headers::{Authorization, Bearer};
use iron::prelude::*;

use super::headers::*;
use http::rendering::render_json_error;
use util::errors::{bad_err, not_acceptable_error};

use auth::config::PLUGIN_JWT;
use auth::config::{PLUGIN_PASSTICKET, PLUGIN_PASSWORD, PLUGIN_SERVICE_ACCOUNT};

use auth::util::authenticatable::Authenticatable;

//A trait responsible for extracting the identity plugin headers.
pub trait HeaderExtracter {
    const AUTH_CONF_NAME: &'static str;

    fn exists_conf_key() -> Option<String> {
        if !Self::AUTH_CONF_NAME.trim().is_empty() {
            return Some(Self::AUTH_CONF_NAME.to_string());
        }
        None
    }

    fn extract(
        req: iron::Headers,
        token: String,
        config_value: Option<&String>,
    ) -> Option<Authenticatable>;
}

struct EmailHeader {}

//A trait responsible for extracting the email header
impl HeaderExtracter for EmailHeader {
    const AUTH_CONF_NAME: &'static str = "email";

    fn extract(
        req: iron::Headers,
        token: String,
        _config_value: Option<&String>,
    ) -> Option<Authenticatable> {
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

//A trait responsible for extracting the service_account header
impl HeaderExtracter for ServiceAccountHeader {
    const AUTH_CONF_NAME: &'static str = PLUGIN_SERVICE_ACCOUNT;

    fn extract(
        req: iron::Headers,
        token: String,
        config_value: Option<&String>,
    ) -> Option<Authenticatable> {
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
    const AUTH_CONF_NAME: &'static str = "jwt";

    fn extract(
        req: iron::Headers,
        token: String,
        _config_value: Option<&String>,
    ) -> Option<Authenticatable> {
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

    fn extract(
        req: iron::Headers,
        _token: String,
        _config_value: Option<&String>,
    ) -> Option<Authenticatable> {
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
    pub fn new(
        req_headers: iron::Headers,
        plugins: Vec<String>,
        conf: HashMap<String, String>,
    ) -> IronResult<Self> {
        let req = req_headers.clone();

        let token = match req.get::<Authorization<Bearer>>() {
            Some(&Authorization(Bearer { ref token })) => token,
            _ => {
                let err = not_acceptable_error(&format!("Authorization Bearer: token not found."));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        };

        let scrappers = plugins
            .into_iter()
            .map(|p| match p.as_str() {
                PLUGIN_PASSWORD => EmailHeader::extract(
                    req.clone(),
                    token.to_string(),
                    EmailHeader::exists_conf_key().and_then(|x| conf.get(&x)),
                ),
                PLUGIN_SERVICE_ACCOUNT => ServiceAccountHeader::extract(
                    req.clone(),
                    token.to_string(),
                    ServiceAccountHeader::exists_conf_key().and_then(|x| conf.get(&x)),
                ),
                PLUGIN_PASSTICKET => PassTicketHeader::extract(
                    req.clone(),
                    token.to_string(),
                    EmailWithJWTTokenHeader::exists_conf_key().and_then(|x| conf.get(&x)),
                ),
                PLUGIN_JWT => EmailWithJWTTokenHeader::extract(
                    req.clone(),
                    token.to_string(),
                    EmailWithJWTTokenHeader::exists_conf_key().and_then(|x| conf.get(&x)),
                ),
                &_ => None,
            })
            .collect();

        Ok(HeaderDecider {
            extractables: scrappers,
        })
    }

    pub fn decide(&self) -> IronResult<Authenticatable> {
        let validate = valid_header(&self.extractables);

        if validate.is_some() {
            return Ok(validate.unwrap());
        }
        let err = not_acceptable_error(&format!("Authentication not supported. You must have headers for the supported authetication. Refer https://bit.ly/rioos_sh_adminguide"));
        return Err(render_json_error(&bad_err(&err), err.http_code()));
    }
}

fn valid_header(authenticatables: &Vec<Option<Authenticatable>>) -> Option<Authenticatable> {
    authenticatables
        .iter()
        .fold(None, |acc, x| acc.or(x.clone()).clone())
}
