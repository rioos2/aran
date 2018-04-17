// Copyright (c) 2018 Rio Advancement Inc
//

pub mod prelude;

use std::net::IpAddr;

use hab_net::app::config::RouterAddr;
use num_cpus;

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

    /// Return a list of router addresses
    fn route_addrs(&self) -> &[RouterAddr];
}
