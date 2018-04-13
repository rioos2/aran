// Copyright 2018 The Rio Advancement Inc
//

// These functions transform source error types into other.
#![cfg_attr(feature="cargo-clippy", allow(needless_pass_by_value))]

use std::error::Error as StdError;
use std::io;

// Common error helpers

pub fn other_error<S: AsRef<str>>(s: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, s.as_ref())
}

pub fn result_ok<T, E: StdError>(_: T) -> Result<(), E> {
    Ok(())
}

pub fn log_error<E: StdError>(err: E) {
    error!("An error occured: {}", err)
}

pub fn into_other<E: StdError>(err: E) -> io::Error {
    other_error(&format!("An error occured, {}", err.description()))
}

pub trait LogError {
    fn log_error(self);
}

impl<T, E> LogError for Result<T, E>
where
    E: ::std::fmt::Display,
{
    fn log_error(self) {
        if let Err(error) = self {
            error!("An error occurred: {}", error);
        }
    }
}
