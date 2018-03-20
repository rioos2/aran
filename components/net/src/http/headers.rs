// Copyright 2018 The Rio Advancement Inc

use std::fmt;
use std::result;
use std::str::FromStr;
use error::Error;
use iron::headers::{Header, HeaderFormat};
use iron::error::HttpError;
use iron::headers::parsing::from_one_raw_str;

#[derive(Debug, Clone, PartialEq)]
pub struct XAuthRioOSEmail(pub String);

impl Header for XAuthRioOSEmail {
    fn header_name() -> &'static str {
        "X-AUTH-RIOOS-EMAIL"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, HttpError> {
        from_one_raw_str(raw).map(XAuthRioOSEmail)
    }
}

impl HeaderFormat for XAuthRioOSEmail {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl fmt::Display for XAuthRioOSEmail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for XAuthRioOSEmail {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        Ok(XAuthRioOSEmail(value.to_string()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct XAuthRioOSServiceAccountName(pub String);

impl Header for XAuthRioOSServiceAccountName {
    fn header_name() -> &'static str {
        "X-AUTH-RIOOS-SERVICE-ACCOUNT-NAME"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, HttpError> {
        from_one_raw_str(raw).map(XAuthRioOSServiceAccountName)
    }
}

impl HeaderFormat for XAuthRioOSServiceAccountName {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl fmt::Display for XAuthRioOSServiceAccountName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for XAuthRioOSServiceAccountName {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        Ok(XAuthRioOSServiceAccountName(value.to_string()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct XAuthRioOSUserAccountEmail(pub String);

impl Header for XAuthRioOSUserAccountEmail {
    fn header_name() -> &'static str {
        "X-AUTH-RIOOS-USER-ACCOUNT-EMAIL"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, HttpError> {
        from_one_raw_str(raw).map(XAuthRioOSUserAccountEmail)
    }
}

impl HeaderFormat for XAuthRioOSUserAccountEmail {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl fmt::Display for XAuthRioOSUserAccountEmail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for XAuthRioOSUserAccountEmail {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        Ok(XAuthRioOSUserAccountEmail(value.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct XAuthRioOSOTP(pub String);

impl Header for XAuthRioOSOTP {
    fn header_name() -> &'static str {
        "X-AUTH-RIOOS-OTP"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, HttpError> {
        from_one_raw_str(raw).map(XAuthRioOSOTP)
    }
}

impl HeaderFormat for XAuthRioOSOTP {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl fmt::Display for XAuthRioOSOTP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for XAuthRioOSOTP {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        Ok(XAuthRioOSOTP(value.to_string()))
    }
}
