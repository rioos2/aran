// Copyright 2018 The Rio Advancement Inc
//
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

/// Public listening net address for HTTP requests, Watch requests.
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct HttpsCfg {
    pub listen: IpAddr,               // The listen ip address for [https api, http2 streamer, wss websocket]
    pub port: u16,                    // The https api server port
    pub tls: Option<String>,          // The tls_pkcs12 is the pfx file that is used as security to start the server.
    pub tls_password: Option<String>, // The tls_pkcs12_pwd  is the pfx file password.
}

impl Default for HttpsCfg {
    fn default() -> Self {
        HttpsCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 7443,
            tls: Some("api-server.pfx".to_string()),
            tls_password: (Some("TEAMRIOADVANCEMENT123".to_string())),
        }
    }
}

impl ToSocketAddrs for HttpsCfg {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        match self.listen {
            IpAddr::V4(ref a) => (*a, self.port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, self.port).to_socket_addrs(),
        }
    }
}

/// Path to UI files to host over HTTP. If not set the UI will be disabled.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    pub root: Option<String>,
}

/// A trait
pub trait AuthenticationFlowCfg {
    //The set of modes and their input.
    //At the moment we support just string input for any of the authentication modes.
    //example: service_account: service_account.pub
    fn modes(&self) -> Vec<(String, String)>;

    //Are all the authentication modes configured and ready.
    fn ready(&self) -> bool;
}
