// Copyright (c) 2017 RioCorp Inc.

use std::io::BufRead;

use error::{Error, Result};

pub struct Plan {
    pub name: String,
    pub version: String,
}

impl Plan {
    pub fn new(name: String, version: String) -> Self {
        Plan {
            name: name,
            version: version,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut name: Option<String> = None;
        let mut version: Option<String> = None;
        for line in bytes.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.splitn(2, "=").collect();
                match parts[0] {
                    "pkg_name" => name = Some(parts[1].to_string()),
                    "pkg_version" => version = Some(parts[1].to_string()),
                    _ => (),
                }
            }
        }

        // Only the name is required to be present initiallly in the plan.sh
        if name.is_none() {
            return Err(Error::PlanMalformed);
        }

        // Default the version to 'undefined' if it's not present
        let v = if version.is_none() {
            String::from("undefined")
        } else {
            version.unwrap()
        };

        let plan = Plan::new(name.unwrap(), v);
        Ok(plan)
    }
}
