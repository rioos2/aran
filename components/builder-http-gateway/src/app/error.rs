// Copyright (c) 2018 Rio Advancement Inc
//
use std::fmt;
use std::error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {

}

impl fmt::Display for AppError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {};
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        match *self {}
    }
}
