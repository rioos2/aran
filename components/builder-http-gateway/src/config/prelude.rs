// Copyright (c) 2018 Rio Advancement Inc
//
pub use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

pub use super::GatewayCfg;
pub use config::base::{HttpsCfg, UiCfg};

/// The tuple  for TLS
/// 0.  Is the tls file location
/// 1.  Is the tls file loaded in bytes using the tls_password if supplied
/// 2.  Is the the tls_password supplied or ""
pub type TLSPair = Option<(String, Vec<u8>, String)>;
