// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//


use std::error;
use std::io;
use std::fmt;
use std::result;

use hyper;
use serde_json;
use url;

use rioos_core;
use rioos_http;

#[derive(Debug)]
pub enum Error {
    APIError(hyper::status::StatusCode, String),
    DownloadFailed(String),
    HabitatCore(rioos_core::Error),
    HabitatHttpClient(rioos_http::Error),
    HyperError(hyper::error::Error),
    IO(io::Error),
    Json(serde_json::Error),
    NoFilePart,
    NoXFilename,
    IdentNotFullyQualified,
    UploadFailed(String),
    UrlParseError(url::ParseError),
    WriteSyncFailed,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::APIError(ref c, ref m) if m.len() > 0 => format!("[{}] {}", c, m),
            Error::APIError(ref c, _) => format!("[{}]", c),
            Error::DownloadFailed(ref s) => format!("Download failed: {}", s),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HabitatHttpClient(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::NoFilePart => {
                format!(
                    "An invalid path was passed - we needed a filename, and this path does \
                         not have one"
                )
            }
            Error::NoXFilename => {
                format!("Invalid download from Builder - missing X-Filename header")
            }
            Error::IdentNotFullyQualified => {
                format!(
                    "Cannot perform the specified operation on a package identifier that is not \
                    fully qualified; please include the package version and release"
                )
            }
            Error::UploadFailed(ref s) => format!("Upload failed: {}", s),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::WriteSyncFailed => {
                format!("Could not write to destination; perhaps the disk is full?")
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIError(_, _) => "Received a non-2XX response code from API",
            Error::DownloadFailed(_) => "Download failed",
            Error::HabitatCore(ref err) => err.description(),
            Error::HabitatHttpClient(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::NoFilePart => {
                "An invalid path was passed - we needed a filename, and this path does not have one"
            }
            Error::NoXFilename => "Invalid download from Builder - missing X-Filename header",
            Error::IdentNotFullyQualified => {
                "Cannot perform the specified operation on a package identifier that is not fully \
                qualified"
            }
            Error::UploadFailed(_) => "Upload failed",
            Error::UrlParseError(ref err) => err.description(),
            Error::WriteSyncFailed => {
                "Could not write to destination; bytes written was 0 on a non-0 buffer"
            }
        }
    }
}

impl From<rioos_core::Error> for Error {
    fn from(err: rioos_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<rioos_http::Error> for Error {
    fn from(err: rioos_http::Error) -> Error {
        Error::HabitatHttpClient(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::HyperError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}