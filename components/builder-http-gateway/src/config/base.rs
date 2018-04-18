// Copyright 2018 The Rio Advancement Inc
//
use std::env;
use std::net::{IpAddr, Ipv4Addr};

/// Public listening net address for HTTP requests, Watch requests.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,       // The listen ip address for [https api, http2 streamer, wss websocket]
    pub port: u16,            // The https api server port
    pub streamer_port: u16,   // The http2 streamer server port
    pub uistreamer_port: u16, // The websocker uistreamer server port
    /// This file is used by both http api/watch server.
    pub tls_pkcs12_file: Option<String>, // The tls_pkcs12 is the pfx file that is used as security to start the server.
    pub tls_pkcs12_pwd: Option<String>, // The tls_pkcs12_pwd  is the pfx file password.
    pub serviceaccount_public_key: Option<String>,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 7443,
            streamer_port: 8443,
            uistreamer_port: 9443,
            tls_pkcs12_file: None,
            tls_pkcs12_pwd: None,
            serviceaccount_public_key: None,
        }
    }
}

/// Path to UI files to host over HTTP. If not set the UI will be disabled.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    pub root: Option<String>,
}
