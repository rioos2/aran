use reqwest::Url;
use super::super::error::{Error, Result};
use reqwest;
use std::time::Duration;
use reqwest::header::{Authorization, Bearer, UserAgent};
const USER_AGENT: &'static str = "Rio/OS Aran";
const HTTP_TIMEOUT: u64 = 3_000;

fn reqwest_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(HTTP_TIMEOUT))
        .build()?;
    Ok(client)
}

pub fn http_bearer_get(url: Url, token: &str) -> Result<reqwest::Response> {
    reqwest_client()?
        .get(url)
        .header(Authorization(Bearer { token: token.to_owned() }))
        .header(UserAgent::new(USER_AGENT.to_string()))
        .send()
        .map_err(Error::ReqwestError)
}
