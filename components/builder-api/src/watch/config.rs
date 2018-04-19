// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS Streamer server

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct StreamerCfg {
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
            port: 8443,
            websocket: 9443,
            tls: Some("api-server.pfx"),
            tls_password: Some("TEAMRIOADVANCEMENT123"),
        }
    }
}

pub trait Streamer {
    fn http2_port(&self) -> u16;

    fn websocket_port(&self) -> u16;

    fn tls(&self) -> Option<String>;

    fn tls_password(&self) -> Option<String>;
}
