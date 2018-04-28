// Copyright (c) 2017 RioCorp Inc

use std::error;
use std::io;
use std::fmt;
use std::result;
use std::str;
use std::string;
use std::env;

use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CryptoKeyError(String),
    FileNameError,
    IO(io::Error),
    RootRequired,
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    TomlSerializeError(toml::ser::Error),
    EditStatus,
    EditorEnv(env::VarError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::CryptoKeyError(ref s) => format!("Missing or invalid key: {}", s),
            Error::FileNameError => format!("Failed to extract a filename"),
            Error::IO(ref err) => format!("{}", err),
            Error::RootRequired => "Root or administrator permissions required to complete operation".to_string(),
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TomlSerializeError(ref e) => format!("Can't serialize TOML: {}", e),
            Error::EditStatus => format!("Failed edit text command"),
            Error::EditorEnv(ref e) => format!("Missing EDITOR environment variable: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CryptoKeyError(_) => "Missing or invalid key",
            Error::FileNameError => "Failed to extract a filename from a path",
            Error::IO(ref err) => err.description(),
            Error::RootRequired => "Root or administrator permissions required to complete operation",
            Error::StrFromUtf8Error(_) => "Failed to convert a string as UTF-8",
            Error::StringFromUtf8Error(_) => "Failed to convert a string as UTF-8",
            Error::TomlSerializeError(_) => "Can't serialize TOML",
            Error::EditorEnv(_) => "Missing EDITOR environment variable",
            Error::EditStatus => "Failed edit text command",
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::StrFromUtf8Error(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlSerializeError(err)
    }
}
