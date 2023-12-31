// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use postgres;
use std::error;
use std::fmt;
use std::result;
use db;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    StorageCreate(postgres::error::Error),
    StorageGetResponse(postgres::error::Error),
    StorageGet(postgres::error::Error),
    StorageSetStatus(postgres::error::Error),
    DcCreate(postgres::error::Error),
    DcGetResponse(postgres::error::Error),
    StoragePoolCreate(postgres::error::Error),
    StoragePoolGetResponse(postgres::error::Error),
    StoragePoolSetStatus(postgres::error::Error),
    DatacenterUpdate(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::StorageCreate(ref e) => format!("Database error creating a storage, {}", e),
            Error::StorageGetResponse(ref e) => format!("Database error list storages, {}", e),
            Error::StorageGet(ref e) => format!("Database error retrive the storage, {}", e),
            Error::StorageSetStatus(ref e) => format!("Database error updating the storage, {}", e),
            Error::DcCreate(ref e) => format!("Database error creating a data_center, {}", e),
            Error::DcGetResponse(ref e) => format!("Database error list data_center, {}", e),
            Error::StoragePoolCreate(ref e) => format!("Database error creating a storage pool, {}", e),
            Error::StoragePoolGetResponse(ref e) => format!("Database error list storages pool, {}", e),
            Error::StoragePoolSetStatus(ref e) => format!("Database error for storages pool status update, {}", e),
            Error::DatacenterUpdate(ref e) => format!("Database error for datacenter update, {}", e),

        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::StorageCreate(ref err) => err.description(),
            Error::StorageGetResponse(ref err) => err.description(),
            Error::StorageGet(ref err) => err.description(),
            Error::StorageSetStatus(ref err) => err.description(),
            Error::DcCreate(ref err) => err.description(),
            Error::DcGetResponse(ref err) => err.description(),
            Error::StoragePoolCreate(ref err) => err.description(),
            Error::StoragePoolGetResponse(ref err) => err.description(),
            Error::StoragePoolSetStatus(ref err) => err.description(),
            Error::DatacenterUpdate(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
