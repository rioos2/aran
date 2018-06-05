// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder deployment

use std::error;
use std::fmt;
use std::result;
use job;
use postgres;
use db;
use telemetry;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    AssemblyFactoryCreate(postgres::error::Error),
    AssemblyFactoryGet(postgres::error::Error),
    AssemblyFactoryUpdate(postgres::error::Error),
    BlockchainFactoryCreate(postgres::error::Error),
    BlockchainFactoryGet(postgres::error::Error),
    BlockchainFactoryUpdate(postgres::error::Error),
    AssemblyCreate(postgres::error::Error),
    AssemblyGet(postgres::error::Error),
    AssemblyUpdate(postgres::error::Error),
    PlanCreate(postgres::error::Error),
    PlanGet(postgres::error::Error),
    EndPointsGet(postgres::error::Error),
    ServicesCreate(postgres::error::Error),
    ServicesGet(postgres::error::Error),
    ServicesUpdate(postgres::error::Error),
    EndPointsCreate(postgres::error::Error),
    VolumesCreate(postgres::error::Error),
    VolumesGet(postgres::error::Error),
    VolumeUpdate(postgres::error::Error),
    Jobs(job::error::Error),
    PromoStatusGetError(telemetry::error::Error),
    PlanSetStatus(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::AssemblyFactoryCreate(ref e) => format!("Database error creating a new assembly factory, {}", e),
            Error::AssemblyFactoryGet(ref e) => format!("Database error getting assembly factory, {}", e),
            Error::AssemblyFactoryUpdate(ref e) => format!("Database error updating assembly factory, {}", e),
            Error::BlockchainFactoryCreate(ref e) => format!("Database error creating a new blockchain factory, {}", e),
            Error::BlockchainFactoryGet(ref e) => format!("Database error getting blockchain factory, {}", e),
            Error::BlockchainFactoryUpdate(ref e) => format!("Database error updating blockchain factory, {}", e),
            Error::AssemblyCreate(ref e) => format!("Database error creating a new assembly, {}", e),
            Error::AssemblyGet(ref e) => format!("Database error getting assembly, {}", e),
            Error::AssemblyUpdate(ref e) => format!("Database error updating a assembly, {}", e),
            Error::PlanCreate(ref e) => format!("Database error creating a plan factory, {}", e),
            Error::PlanGet(ref e) => format!("Database error getting plan data, {}", e),
            Error::EndPointsCreate(ref e) => format!("Database error creating a end points, {}", e),
            Error::EndPointsGet(ref e) => format!("Error retrive endpoint, {}", e),
            Error::ServicesCreate(ref e) => format!("Database error creating services, {}", e),
            Error::ServicesGet(ref e) => format!("Error retrive service, {}", e),
            Error::ServicesUpdate(ref e) => format!("Error updating service, {}", e),
            Error::VolumesCreate(ref e) => format!("Error creating volume, {}", e),
            Error::VolumesGet(ref e) => format!("Error geting volume, {}", e),
            Error::PromoStatusGetError(ref e) => format!("Prometheus connection refused , {}", e),
            Error::VolumeUpdate(ref e) => format!("Error updating volume, {}", e),
            Error::PlanSetStatus(ref e) => format!("Error updating plan status, {}", e),
            Error::Jobs(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::AssemblyFactoryCreate(ref err) => err.description(),
            Error::AssemblyFactoryGet(ref err) => err.description(),
            Error::AssemblyFactoryUpdate(ref err) => err.description(),
            Error::BlockchainFactoryCreate(ref err) => err.description(),
            Error::BlockchainFactoryGet(ref err) => err.description(),
            Error::BlockchainFactoryUpdate(ref err) => err.description(),
            Error::AssemblyCreate(ref err) => err.description(),
            Error::AssemblyGet(ref err) => err.description(),
            Error::AssemblyUpdate(ref err) => err.description(),
            Error::PlanCreate(ref err) => err.description(),
            Error::PlanGet(ref err) => err.description(),
            Error::EndPointsCreate(ref err) => err.description(),
            Error::EndPointsGet(ref err) => err.description(),
            Error::ServicesCreate(ref err) => err.description(),
            Error::ServicesGet(ref err) => err.description(),
            Error::ServicesUpdate(ref err) => err.description(),
            Error::VolumesCreate(ref err) => err.description(),
            Error::VolumesGet(ref err) => err.description(),
            Error::VolumeUpdate(ref err) => err.description(),
            Error::Jobs(ref err) => err.description(),
            Error::PlanSetStatus(ref err) => err.description(),
            Error::PromoStatusGetError(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<job::error::Error> for Error {
    fn from(err: job::error::Error) -> Self {
        Error::Jobs(err)
    }
}

impl From<telemetry::error::Error> for Error {
    fn from(err: telemetry::error::Error) -> Error {
        Error::PromoStatusGetError(err)
    }
}
