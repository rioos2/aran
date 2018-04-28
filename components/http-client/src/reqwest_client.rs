use std::path::Path;
use std::fs::File;
use std::io::Read;

use reqwest::Url;
use reqwest::StatusCode;
use super::error::{Error, Result};
use reqwest;
use std::time::Duration;
use api_client::err_from_response;

use reqwest::header::{Authorization, Bearer, UserAgent};
const USER_AGENT: &'static str = "Rio/OS Aran";
const HTTP_TIMEOUT: u64 = 3_000;

fn reqwest_client(fs_root_path: Option<&Path>) -> Result<reqwest::Client> {
    let mut buf = Vec::new();
    if !fs_root_path.is_none() && (File::open(fs_root_path.unwrap()).map(|mut x| x.read_to_end(&mut buf))).is_ok() {
        Ok(reqwest::Client::builder()
            .add_root_certificate(reqwest::Certificate::from_pem(&buf)?)
            .timeout(Duration::from_millis(HTTP_TIMEOUT))
            .build()?)
    } else {
        Ok(reqwest::Client::builder()
            .timeout(Duration::from_millis(HTTP_TIMEOUT))
            .build()?)
    }
}

pub fn http_bearer_get(path: &str, token: &str) -> Result<reqwest::Response> {
    let url = Url::parse(path)?;
    let res = reqwest_client(None)?
        .get(url)
        .header(Authorization(Bearer { token: token.to_owned() }))
        .header(UserAgent::new(USER_AGENT.to_string()))
        .send()
        .map_err(Error::ReqwestError)?;
    if res.status() != StatusCode::Ok {
        return Err(err_from_response(res));
    };
    Ok(res)
}

pub fn http_basic_get(path: &str, username: String, password: String) -> Result<reqwest::Response> {
    let url = Url::parse(path)?;
    let res = reqwest_client(None)?
        .get(url)
        .basic_auth(username.to_string(), Some(password.to_string()))
        .send()
        .map_err(Error::ReqwestError)?;
    if res.status() != StatusCode::Ok {
        return Err(err_from_response(res));
    };
    Ok(res)
}
