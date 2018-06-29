// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use db;
use postgres;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    SecretCreate(postgres::error::Error),
    SecretGet(postgres::error::Error),
    SecretGetResponse(postgres::error::Error),
    ServiceAccountCreate(postgres::error::Error),
    ServiceAccountUpdate(postgres::error::Error),
    ServiceAccountGetResponse(postgres::error::Error),
    ServiceAccountGet(postgres::error::Error),
    SettingsMapCreate(postgres::error::Error),
    SettingsMapGet(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::SecretCreate(ref e) => format!("Database error creating a secret, {}", e),
            Error::SecretGet(ref e) => format!("Database error get secret, {}", e),
            Error::SecretGetResponse(ref e) => format!("Error retrive secret_list database, {}", e),
            Error::ServiceAccountCreate(ref e) => {
                format!("Database error creating a service_account, {}", e)
            }
            Error::ServiceAccountUpdate(ref e) => {
                format!("Database error updating a service_account, {}", e)
            }
            Error::SettingsMapCreate(ref e) => {
                format!("Database error creating a settings map, {}", e)
            }
            Error::SettingsMapGet(ref e) => format!("Database error getting a settings map, {}", e),

            Error::ServiceAccountGetResponse(ref e) => format!(
                "Error retrive service_account for account in database, {}",
                e
            ),
            Error::ServiceAccountGet(ref e) => format!("Error retrive service_account , {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::SecretCreate(ref err) => err.description(),
            Error::SecretGet(ref err) => err.description(),
            Error::SecretGetResponse(ref err) => err.description(),
            Error::ServiceAccountCreate(ref err) => err.description(),
            Error::ServiceAccountUpdate(ref err) => err.description(),
            Error::ServiceAccountGetResponse(ref err) => err.description(),
            Error::ServiceAccountGet(ref err) => err.description(),
            Error::SettingsMapCreate(ref err) => err.description(),
            Error::SettingsMapGet(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
