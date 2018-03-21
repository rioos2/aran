// Copyright 2018 The Rio Advancement Inc

use std::error;
use std::fmt;
use std::io;
use std::result;
use serde_json::Error as SJError;
use openssl::error::ErrorStack;
use base64::DecodeError as B64Error;

use rioos;

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
    OldOTPMustBeRemoved(String),
    CantVerifyOT(String),
    OTPMismatch,
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
            Error::OTPMismatch => format!("Rio/OS OTP Invlid"),
            Error::OldOTPMustBeRemoved(ref e) => format!("{}", e),
            Error::CantVerifyOT(ref e) => format!("{}", e),
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
            Error::OldOTPMustBeRemoved(ref _e) => "OTP Removing error",
            Error::CantVerifyOT(ref _e) => "OTP Mismatch error",
            Error::OTPMismatch => "Rio/OS OTP Invalid",
            Error::SignatureExpired => "signature expired",
            Error::SignatureInvalid => "signature invalid",
            Error::JWTInvalid => "JWT token invalid",
            Error::IssuerInvalid => "JWT token issuer invalid",
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
