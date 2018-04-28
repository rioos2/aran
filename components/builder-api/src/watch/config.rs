// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS Streamer server

use std::net::{IpAddr, Ipv4Addr};

use http_gateway::config::prelude::TLSPair;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct StreamerCfg {
    pub listener: IpAddr,
    //  The base listener configuration for https
    pub port: u16,
    //  The streamer (http2, websocker) configuration
    pub websocket: u16,
    //  The tls certificate in pfx format.
    pub tls: Option<String>,
    //  The password of the tls certificate
    pub tls_password: Option<String>,
}

impl Default for StreamerCfg {
    fn default() -> Self {
        StreamerCfg {
            listener: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8443,
            websocket: 9443,
            tls: Some("api-server.pfx".to_string()),
            tls_password: Some("TEAMRIOADVANCEMENT123".to_string()),
        }
    }
}

pub trait Streamer {
    fn http2_port(&self) -> u16;

    fn websocket_port(&self) -> u16;

    fn http2_tls_pair(&self) -> TLSPair;

    fn http2_tls(&self) -> Option<String>;

    fn http2_tls_password(&self) -> Option<String>;
}
