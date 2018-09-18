// Copyright 2018 The Rio Advancement Inc

//! A module containing
extern crate errno;

#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate get_if_addrs;
extern crate humansize;
extern crate num_cpus;
extern crate regex;
extern crate serde_json;
extern crate sys_info;
#[cfg(test)]
extern crate tempdir;
extern crate dirs;
extern crate time;
extern crate toml;

#[cfg(not(windows))]
extern crate users as linux_users;

pub use self::error::{Error, Result};

pub mod config;
pub mod crypto;
pub mod env;
pub mod error;
pub mod fs;
pub mod os;
pub mod util;

pub use os::filesystem;
pub use os::users;
