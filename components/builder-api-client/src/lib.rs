// Copyright (c) 2017 RioCorp Inc.
//


#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rioos_builder_protocol as protocol;
extern crate rioos_core as rioos_core;
extern crate rioos_http_client as rioos_http;
extern crate broadcast;
#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate tee;
extern crate url;

pub mod error;
pub use error::{Error, Result};

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::string::ToString;

use broadcast::BroadcastWriter;
use rioos_http::ApiClient;
use rioos_http::util::decoded_response;
use hyper::client::{Body, IntoUrl, Response, RequestBuilder};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Bearer};
use hyper::Url;
use protocol::{originsrv, net};
use protocol::net::NetError;
use rand::{Rng, thread_rng};
use tee::TeeReader;
use url::percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }

const DEFAULT_API_PATH: &'static str = "/v1";


/// Custom conversion logic to allow `serde` to successfully
/// round-trip `u64` datatypes through JSON serialization.
///
/// To use it, add `#[serde(with = "json_u64")]` to any `u64`-typed struct
/// fields.
mod json_u64 {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", num);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

pub trait DisplayProgress: Write {
    fn size(&mut self, size: u64);
    fn finish(&mut self);
}

pub struct Client(ApiClient);

impl Client {
    pub fn new<U>(endpoint: U, product: &str, version: &str, fs_root_path: Option<&Path>) -> Result<Self>
    where
        U: IntoUrl,
    {
        let mut endpoint = endpoint.into_url()?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }
        Ok(Client(
            ApiClient::new(endpoint, product, version, fs_root_path)?,
        ))
    }


    /// Download a public key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub fn fetch_origin_key<D, P: ?Sized>(&self, origin: &str, revision: &str, dst_path: &P, progress: Option<D>) -> Result<PathBuf>
    where
        P: AsRef<Path>,
        D: DisplayProgress + Sized,
    {
        self.download(
            &format!("depot/origins/{}/keys/{}", origin, revision),
            dst_path.as_ref(),
            progress,
        )
    }


    /// Download the latest builder public key from a remote Builder
    /// to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub fn fetch_builder_latest_key<D, P: ?Sized>(&self, dst_path: &P, progress: Option<D>) -> Result<PathBuf>
    where
        P: AsRef<Path>,
        D: DisplayProgress + Sized,
    {
        self.download("builder/keys/latest", dst_path.as_ref(), progress)
    }


    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
    }

    fn download<D>(&self, path: &str, dst_path: &Path, progress: Option<D>) -> Result<PathBuf>
    where
        D: DisplayProgress + Sized,
    {
        let mut res = self.0.get(path).send()?;
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }
        fs::create_dir_all(&dst_path)?;

        let file_name = match res.headers.get::<XFileName>() {
            Some(filename) => format!("{}", filename),
            None => return Err(Error::NoXFilename),
        };
        let tmp_file_path = dst_path.join(format!(
            "{}.tmp-{}",
            file_name,
            thread_rng().gen_ascii_chars().take(8).collect::<String>()
        ));
        let dst_file_path = dst_path.join(file_name);
        debug!("Writing to {}", &tmp_file_path.display());
        let mut f = File::create(&tmp_file_path)?;
        match progress {
            Some(mut progress) => {
                let size: u64 = res.headers.get::<hyper::header::ContentLength>().map_or(
                    0,
                    |v| **v,
                );
                progress.size(size);
                let mut writer = BroadcastWriter::new(&mut f, progress);
                io::copy(&mut res, &mut writer)?
            }
            None => io::copy(&mut res, &mut f)?,
        };
        debug!(
            "Moving {} to {}",
            &tmp_file_path.display(),
            &dst_file_path.display()
        );
        fs::rename(&tmp_file_path, &dst_file_path)?;
        Ok(dst_file_path)
    }
}

fn err_from_response(mut response: hyper::client::Response) -> Error {
    if response.status == StatusCode::Unauthorized {
        return Error::APIError(
            response.status,
            "Your GitHub token requires both user:email and read:org permissions.".to_string(),
        );
    }

    let mut buff = String::new();
    match response.read_to_string(&mut buff) {
        Ok(_) => {
            match serde_json::from_str::<NetError>(&buff) {
                Ok(err) => Error::APIError(response.status, err.to_string()),
                Err(_) => Error::APIError(response.status, buff),
            }
        }
        Err(_) => {
            buff.truncate(0);
            Error::APIError(response.status, buff)
        }
    }
}

fn origin_keys_path(origin: &str) -> String {
    format!("depot/origins/{}/keys", origin)
}

fn origin_secret_keys_latest(origin: &str) -> String {
    format!("depot/origins/{}/secret_keys/latest", origin)
}


fn package_search(term: &str) -> String {
    let encoded_term = percent_encode(term.as_bytes(), PATH_SEGMENT_ENCODE_SET);
    format!("depot/pkgs/search/{}", encoded_term)
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn json_round_trip_u64_fields() {
        let pre = OriginSecretKey {
            id: 705705315793903646,
            origin_id: 705705305031319582,
            name: "core".to_string(),
            revision: "20160810182414".to_string(),
            body: vec![1, 2, 3],
            owner_id: 0,
        };

        let as_json = serde_json::to_value(&pre).unwrap();
        let expected = json!({
            "id": "705705315793903646",
            "origin_id": "705705305031319582",
            "name": "core",
            "revision": "20160810182414",
            "body": [
                1,
                2,
                3
            ],
            "owner_id": "0"
        });
        assert_eq!(as_json, expected);

    }
}
