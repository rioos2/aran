// Copyright 2018 The Rio Advancement Inc
//

use std::error;
use std::result;

use core::os::process;

pub trait Application {
    type Error: error::Error;

    fn run(&mut self) -> result::Result<(), Self::Error>;
}

pub trait NetIdent {
    fn component() -> Option<&'static str> {
        None
    }

    fn net_ident() -> String {
        let hostname = super::hostname().unwrap();
        let pid = process::current_pid();
        if let Some(component) = Self::component() {
            format!("{}#{}@{}", component, pid, hostname)
        } else {
            format!("{}@{}", pid, hostname)
        }
    }
}
