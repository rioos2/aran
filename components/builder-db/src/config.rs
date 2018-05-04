// Copyright 2018 The Rio Advancement Inc

use std::error::Error;
use std::fmt;
use std::net::{Ipv4Addr, IpAddr};

use num_cpus;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use postgres::params::{ConnectParams, Host, IntoConnectParams};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DataStore {
    pub host: IpAddr,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub database: String,
    /// Timing to retry the connection to the data store if it cannot be established
    pub connection_retry_ms: u64,
    /// How often to cycle a connection from the pool
    pub connection_timeout_sec: u64,
    /// If the datastore connection is under test
    pub connection_test: bool,
    /// Number of database connections to start in pool.
    pub pool_size: u32,
}

impl Default for DataStore {
    fn default() -> Self {
        DataStore {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5432,
            user: String::from("rioos"),
            password: Some("rioos".to_string()),
            database: String::from("rioosdb"),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
            pool_size: (num_cpus::get() * 1) as u32,
        }
    }
}

impl<'a> IntoConnectParams for &'a DataStore {
    fn into_connect_params(self) -> Result<ConnectParams, Box<Error + Sync + Send>> {
        let mut builder = ConnectParams::builder();
        builder.port(self.port);
        builder.user(&self.user, self.password.as_ref().map(|p| &**p));
        builder.database(&self.database);
        Ok(builder.build(Host::Tcp(self.host.to_string())))
    }
}

impl fmt::Display for DataStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut connect = format!("postgres://{}", self.user);
        connect = match self.password {
            Some(ref p) => {
                // We can potentially get non-url friendly chars here so we need to encode them
                let encoded_password = utf8_percent_encode(p, PATH_SEGMENT_ENCODE_SET).to_string();
                format!("{}:{}", connect, encoded_password)
            }
            None => connect,
        };
        connect = format!("{}@{}:{}/{}", connect, self.host, self.port, self.database);
        write!(f, "{}", connect)
    }
}
