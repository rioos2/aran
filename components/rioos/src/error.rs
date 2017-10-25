// Copyright (c) 2017 RioCorp Inc.
//

use std::env;
use std::error;
use std::fmt;
use std::io;
use std::num;
use std::path::{self, PathBuf};
use std::result;

use api_client;
use common;
use rioos_core;
use handlebars;
use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    APIClient(api_client::Error),
    APIServerDown,
    ArgumentError(&'static str),
    CannotRemoveFromChannel((String, String)),
    CommandNotFoundInPkg((String, String)),
    CryptoCLI(String),
    DigitalCloudNotFound(String),
    EnvJoinPathsError(env::JoinPathsError),
    ExecCommandNotFound(PathBuf),
    FileNotFound(String),
    HabitatCommon(common::Error),
    HabitatCore(rioos_core::Error),
    HandlebarsRenderError(handlebars::TemplateRenderError),
    IO(io::Error),
    JobGroupPromote(api_client::Error),
    JobGroupPromoteUnprocessable,
    PackageArchiveMalformed(String),
    ParseIntError(num::ParseIntError),
    PathPrefixError(path::StripPrefixError),
    ProvidesError(String),
    RootRequired,
    SubcommandNotSupported(String),
    UnsupportedExportFormat(String),
    TomlDeserializeError(toml::de::Error),
    TomlSerializeError(toml::ser::Error),
    Utf8Error(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::APIClient(ref err) => format!("{}", err),
            Error::ArgumentError(ref e) => format!("{}", e),
            Error::CannotRemoveFromChannel((ref p, ref c)) => format!("{} cannot be removed from the {} channel.", p, c),
            Error::CommandNotFoundInPkg((ref p, ref c)) => {
                format!(
                    "`{}' was not found under any 'PATH' directories in the {} package",
                    c,
                    p
                )
            }
            Error::CryptoCLI(ref e) => format!("{}", e),
            Error::EnvJoinPathsError(ref err) => format!("{}", err),
            Error::ExecCommandNotFound(ref c) => {
                format!(
                    "`{}' was not found on the filesystem or in PATH",
                    c.display()
                )
            }
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::HabitatCommon(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HandlebarsRenderError(ref e) => format!("{}", e),
            Error::IO(ref err) => format!("{}", err),
            Error::JobGroupPromoteUnprocessable => format!("Failed to promote job group, the build job is still in progress"),
            Error::JobGroupPromote(ref e) => format!("Failed to promote job group: {:?}", e),
            Error::PackageArchiveMalformed(ref e) => {
                format!(
                    "Package archive was unreadable or contained unexpected contents: {:?}",
                    e
                )
            }
            Error::ParseIntError(ref err) => format!("{}", err),
            Error::PathPrefixError(ref err) => format!("{}", err),
            Error::ProvidesError(ref err) => format!("Can't find {}", err),
            Error::RootRequired => "Root or administrator permissions required to complete operation".to_string(),
            Error::SubcommandNotSupported(ref e) => format!("Subcommand `{}' not supported on this operating system", e),
            Error::UnsupportedExportFormat(ref e) => format!("Unsupported export format: {}", e),
            Error::TomlDeserializeError(ref e) => format!("Can't deserialize TOML: {}", e),
            Error::TomlSerializeError(ref e) => format!("Can't serialize TOML: {}", e),
            Error::Utf8Error(ref e) => format!("Error processing a string as UTF-8: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIClient(ref err) => err.description(),
            Error::ArgumentError(_) => "There was an error parsing an error or with it's value",
            Error::CannotRemoveFromChannel(_) => "Package cannot be removed from the specified channel",
            Error::CommandNotFoundInPkg(_) => "Command was not found under any 'PATH' directories in the package",
            Error::CryptoCLI(_) => "A cryptographic error has occurred",
            Error::EnvJoinPathsError(ref err) => err.description(),
            Error::ExecCommandNotFound(_) => "Exec command was not found on filesystem or in PATH",
            Error::FileNotFound(_) => "File not found",
            Error::HabitatCommon(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::HandlebarsRenderError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::JobGroupPromoteUnprocessable => "Failed to promote job group, the build job is still in progress",
            Error::JobGroupPromote(ref err) => err.description(),
            Error::PackageArchiveMalformed(_) => "Package archive was unreadable or had unexpected contents",
            Error::ParseIntError(ref err) => err.description(),
            Error::PathPrefixError(ref err) => err.description(),
            Error::ProvidesError(_) => "Can't find a package that provides the given search parameter",
            Error::RootRequired => "Root or administrator permissions required to complete operation",
            Error::SubcommandNotSupported(_) => "Subcommand not supported on this operating system",
            Error::UnsupportedExportFormat(_) => "Unsupported export format",
            Error::TomlDeserializeError(_) => "Can't deserialize TOML",
            Error::TomlSerializeError(_) => "Can't serialize TOML",
            Error::Utf8Error(_) => "Error processing string as UTF-8",

        }
    }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Error {
        Error::HabitatCommon(err)
    }
}

impl From<rioos_core::Error> for Error {
    fn from(err: rioos_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Error {
        Error::HandlebarsRenderError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(err: path::StripPrefixError) -> Error {
        Error::PathPrefixError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlDeserializeError(err)
    }
}
impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlSerializeError(err)
    }
}

impl From<env::JoinPathsError> for Error {
    fn from(err: env::JoinPathsError) -> Self {
        Error::EnvJoinPathsError(err)
    }
}
