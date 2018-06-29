// Copyright (c) 2018 Rio Advancement Inc
//
pub mod base;
pub mod prelude;

use num_cpus;
use std::net::IpAddr;

use self::prelude::TLSPair;

pub trait GatewayCfg {
    /// Default number of worker threads to simultaneously handle HTTP requests.
    fn default_handler_count() -> usize {
        num_cpus::get() * 8
    }

    /// Number of worker threads to simultaneously handle HTTP requests.
    fn handler_count(&self) -> usize {
        Self::default_handler_count()
    }

    fn listen_addr(&self) -> &IpAddr;

    fn listen_port(&self) -> u16;

    fn tls_pair(&self) -> TLSPair;

    fn tls(&self) -> Option<String>;

    fn tls_password(&self) -> Option<String>;
}
