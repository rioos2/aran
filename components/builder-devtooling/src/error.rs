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
    BuildConfigCreate(postgres::error::Error),
    BuildConfigGetResponse(postgres::error::Error),
    BuildConfigGet(postgres::error::Error),
    BuildCreate(postgres::error::Error),
    BuildGetResponse(postgres::error::Error),
    BuildGet(postgres::error::Error),
    ImageRefGet(postgres::error::Error),
    ImageRefCreate(postgres::error::Error),
    BuildConfigUpdate(postgres::error::Error),
    BuildUpdate(postgres::error::Error),
    ImageRefUpdate(postgres::error::Error),
    ImageMarksCreate(postgres::error::Error),
    ImageMarksGet(postgres::error::Error),
    ImageMarksUpdate(postgres::error::Error),
    BuildStatusUpdate(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::BuildConfigCreate(ref e) => {
                format!("Database error creating a build config, {}", e)
            }
            Error::BuildConfigGet(ref e) => format!("Database error get the build config, {}", e),
            Error::BuildConfigGetResponse(ref e) => {
                format!("Database error list build config, {}", e)
            }
            Error::BuildCreate(ref e) => format!("Database error creating a build, {}", e),
            Error::BuildGet(ref e) => format!("Database error get the build, {}", e),
            Error::BuildGetResponse(ref e) => format!("Database error list build , {}", e),
            Error::ImageRefCreate(ref e) => format!("Database error  image ref create, {}", e),
            Error::ImageRefGet(ref e) => format!("Database error get the image ref, {}", e),
            Error::BuildConfigUpdate(ref e) => format!("Database error update build config, {}", e),
            Error::BuildUpdate(ref e) => format!("Database error update build , {}", e),
            Error::ImageRefUpdate(ref e) => {
                format!("Database error update image references , {}", e)
            }
            Error::ImageMarksCreate(ref e) => format!("Database error create image marks , {}", e),
            Error::ImageMarksGet(ref e) => format!("Database error get image marks , {}", e),
            Error::ImageMarksUpdate(ref e) => format!("Database error update image marks , {}", e),
            Error::BuildStatusUpdate(ref e) => {
                format!("Database error status update build , {}", e)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::BuildConfigCreate(ref err) => err.description(),
            Error::BuildConfigGetResponse(ref err) => err.description(),
            Error::BuildConfigGet(ref err) => err.description(),
            Error::BuildCreate(ref err) => err.description(),
            Error::BuildGetResponse(ref err) => err.description(),
            Error::BuildGet(ref err) => err.description(),
            Error::ImageRefGet(ref err) => err.description(),
            Error::ImageRefCreate(ref err) => err.description(),
            Error::BuildConfigUpdate(ref err) => err.description(),
            Error::BuildUpdate(ref err) => err.description(),
            Error::ImageRefUpdate(ref err) => err.description(),
            Error::ImageMarksCreate(ref err) => err.description(),
            Error::ImageMarksGet(ref err) => err.description(),
            Error::ImageMarksUpdate(ref err) => err.description(),
            Error::BuildStatusUpdate(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
