// Copyright 2018 The Rio Advancement Inc

use error::Error;
use std::fmt;
use std::result;
use std::str::FromStr;
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::uname;
#[cfg(not(windows))]
pub mod linux;
#[cfg(not(windows))]
pub use self::linux::uname;

//The distribution architecture is figured out from here.
pub mod dist;

#[derive(Debug)]
pub struct Uname {
    pub sys_name: String,  /* Operating system name (e.g., "Linux") */
    pub node_name: String, /* Name within "some implementation-defined network" */
    pub release: String,   /* Operating system release (e.g., "2.6.28") */
    pub version: String,   /* Operating system version */
    pub machine: String,   /* Hardware identifier */
}

#[derive(Debug, Hash, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Platform {
    Linux,
    Windows,
    Darwin,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let platform_string = format!("{:?}", self);
        write!(f, "{}", platform_string.to_lowercase())
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let platform = value.trim().to_lowercase();
        match platform.as_ref() {
            "linux" => Ok(Platform::Linux),
            "windows" => Ok(Platform::Windows),
            "darwin" => Ok(Platform::Darwin),
            _ => return Err(Error::InvalidPlatform(value.to_string())),
        }
    }
}
