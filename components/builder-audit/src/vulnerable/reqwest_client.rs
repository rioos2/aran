use reqwest::Url;
use super::super::error::{Error, Result};
use reqwest;
use std::time::Duration;
const HTTP_TIMEOUT: u64 = 3_000;

pub fn http_basic_get(url: Url, username: String, password: String) -> Result<reqwest::Response> {
    reqwest_client()?
        .get(url)
        .basic_auth(username.to_string(), Some(password.to_string()))
        .send()
        .map_err(Error::ReqwestError)
}

fn reqwest_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(HTTP_TIMEOUT))
        .build()?;
    Ok(client)
}
