// Copyright 2018 The Rio Advancement Inc

use base64::DecodeError as B64Error;
use openssl::error::ErrorStack;
use rioos;
use serde_json::Error as SJError;
use std::error;
use std::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    Auth(rioos::AuthErr),
    IO(io::Error),
    SignatureExpired,
    SignatureInvalid,
    JWTInvalid,
    IssuerInvalid,
    FormatInvalid(SJError),
    OpenSslError(ErrorStack),
    ProtocolError(B64Error),
    OldPassticketMustBeRemoved(String),
    CantVerifyPassticket(String),
    PassticketMismatch,
    PermissionError(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Auth(ref e) => format!("Rio/OS authorization error, {}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::FormatInvalid(ref e) => format!("Rio/OS authorization error, {}", e),
            Error::OpenSslError(ref e) => format!("Rio/OS authorization error, {}", e),
            Error::ProtocolError(ref e) => format!("Rio/OS authorization error, {}", e),
            Error::SignatureExpired => format!("signature was expired"),
            Error::SignatureInvalid => format!("signature invalid"),
            Error::JWTInvalid => format!("JWT token is invalid"),
            Error::IssuerInvalid => format!("JWT token issuer was invalid"),
            Error::PassticketMismatch => format!("Passticket mismatch"),
            Error::OldPassticketMustBeRemoved(ref e) => format!("{}", e),
            Error::CantVerifyPassticket(ref e) => format!("{}", e),
            Error::PermissionError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Auth(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::FormatInvalid(ref err) => err.description(),
            Error::OpenSslError(ref err) => err.description(),
            Error::ProtocolError(ref err) => err.description(),
            Error::OldPassticketMustBeRemoved(ref _e) => {
                "Old passticket still remains after one time use."
            }
            Error::CantVerifyPassticket(ref _e) => "Passticket Mismatch error",
            Error::PassticketMismatch => "Passticket mismatch",
            Error::SignatureExpired => "signature expired",
            Error::SignatureInvalid => "signature invalid",
            Error::JWTInvalid => "JWT token invalid",
            Error::IssuerInvalid => "JWT token issuer invalid",
            Error::PermissionError(ref e) => e,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<rioos::AuthErr> for Error {
    fn from(err: rioos::AuthErr) -> Self {
        Error::Auth(err)
    }
}

impl From<SJError> for Error {
    fn from(err: SJError) -> Error {
        Error::FormatInvalid(err)
    }
}

impl From<ErrorStack> for Error {
    fn from(err: ErrorStack) -> Error {
        Error::OpenSslError(err)
    }
}

impl From<B64Error> for Error {
    fn from(err: B64Error) -> Error {
        Error::ProtocolError(err)
    }
}
