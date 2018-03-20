// Copyright 2018 The Rio Advancement Inc

//! A module containing the config file loader

use std::error::Error as StdError;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::DeserializeOwned;
use toml;

use error::Error;

pub trait ConfigFile: DeserializeOwned + Sized {
    type Error: StdError + From<Error>;

    fn from_file<T: AsRef<Path>>(filepath: T) -> Result<Self, Self::Error> {
        let mut file = match File::open(filepath.as_ref()) {
            Ok(f) => f,
            Err(e) => {
                return Err(Self::Error::from(Error::ConfigFileIO(
                    filepath.as_ref().to_path_buf(),
                    e,
                )))
            }
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Ok(_) => (),
            Err(e) => {
                return Err(Self::Error::from(Error::ConfigFileIO(
                    filepath.as_ref().to_path_buf(),
                    e,
                )))
            }
        }
        Self::from_raw(&raw)
    }

    fn from_raw(raw: &str) -> Result<Self, Self::Error> {
        let value = toml::from_str(&raw).map_err(|e| Error::ConfigFileSyntax(e))?;
        Ok(value)
    }
}
