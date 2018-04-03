use iron;
use iron::prelude::*;
use iron::headers::{Authorization, Bearer};

use auth::util::authenticatable::Authenticatable;
use super::headers::*;
use util::errors::{not_acceptable_error, bad_err};
use http::rendering::render_json_error;
use core::fs::rioconfig_config_path;


pub trait HeaderExtracter {
    fn extract(req: iron::Headers, token: String) -> Option<Authenticatable>;
}

pub trait HeaderExtracterWithKey {
    fn extract(req: iron::Headers, token: String, key: String) -> Option<Authenticatable>;
}

struct EmailHeader {}

impl HeaderExtracter for EmailHeader {
    fn extract(req: iron::Headers, token: String) -> Option<Authenticatable> {
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

impl HeaderExtracterWithKey for ServiceAccountHeader {
    fn extract(req: iron::Headers, token: String, key: String) -> Option<Authenticatable> {
        let serviceaccount = req.get::<XAuthRioOSServiceAccountName>();
        if !serviceaccount.is_none() {
            return Some(Authenticatable::ServiceAccountNameAndWebtoken {
                name: serviceaccount.unwrap().0.clone(),
                webtoken: token,
                key: format!("{:?}", &rioconfig_config_path(None).join(key)),
            });
        }
        None
    }
}

struct EmailWithJWTTokenHeader {}

impl HeaderExtracter for EmailWithJWTTokenHeader {
    fn extract(req: iron::Headers, token: String) -> Option<Authenticatable> {
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

struct OTPHeader {}

impl HeaderExtracter for OTPHeader {
    fn extract(req: iron::Headers, _token: String) -> Option<Authenticatable> {
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
    extratable: Vec<Option<Authenticatable>>,
}

impl HeaderDecider {
    pub fn new(req_headers: iron::Headers, key: Option<String>) -> IronResult<Self> {
        let req = req_headers.clone();
        let token = match req.get::<Authorization<Bearer>>() {
            Some(&Authorization(Bearer { ref token })) => token,
            _ => {
                let err = not_acceptable_error(&format!("Authorization bearer token not found."));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        };

        let extratable = vec![
            EmailHeader::extract(req.clone(), token.to_string()),
            ServiceAccountHeader::extract(
                req.clone(),
                token.to_string(),
                key.unwrap_or("".to_string())
            ),
            EmailWithJWTTokenHeader::extract(req.clone(), token.to_string()),
            OTPHeader::extract(req.clone(), token.to_string()),
        ];

        Ok(HeaderDecider { extratable: extratable })

    }

    pub fn decide(&self) -> IronResult<Authenticatable> {

        let validate = valid_header(&self.extratable);

        if validate.is_some() {
            return Ok(validate.unwrap());
        }
        let err = not_acceptable_error(&format!(
            "Authentication not supported. You must have headers for the supported authetication. Refer https://www.rioos.sh/admin/auth."
        ));
        return Err(render_json_error(&bad_err(&err), err.http_code()));
    }
}

fn valid_header(extratable_res: &Vec<Option<Authenticatable>>) -> Option<Authenticatable> {
    let validated = extratable_res.iter().fold(
        None,
        |acc, x| acc.or(x.clone()).clone(),
    );
    validated
}
