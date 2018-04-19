// Copyright 2018 The Rio Advancement Inc
//
use std::env;
use std::net::{IpAddr, Ipv4Addr};

/// Public listening net address for HTTP requests, Watch requests.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,               // The listen ip address for [https api, http2 streamer, wss websocket]
    pub port: u16,                    // The https api server port
    pub tls: Option<String>,          // The tls_pkcs12 is the pfx file that is used as security to start the server.
    pub tls_password: Option<String>, // The tls_pkcs12_pwd  is the pfx file password.
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 7443,
            tls: Some("api-server.pfx"),
            tls_password: ("TEAMRIOADVANCEMENT123"),         
        }
    }
}

/// Path to UI files to host over HTTP. If not set the UI will be disabled.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    pub root: Option<String>,
}
