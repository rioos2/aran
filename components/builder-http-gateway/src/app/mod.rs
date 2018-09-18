// Copyright (c) 2018 Rio Advancement Inc
//

//! # Example HttpGateway
//!
//! ```rust,no_run
//! extern crate rioos_builder_protocol as protocol;
//! extern crate builder_http_gateway as http_gateway;
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate router;
//!
//! use std::process;
//!
//! use http_gateway::app::prelude::*;
//!
//! pub mod config {
//!     use http_gateway::config::prelude::*;
//!
//!     #[derive(Default)]
//!     pub struct SrvConfig {
//!         pub http: HttpCfg,
//!         pub routers: Vec<RouterAddr>,
//!     }
//!
//!     impl GatewayCfg for SrvConfig {
//!         fn listen_addr(&self) -> &IpAddr {
//!             &self.http.listen
//!         }
//!
//!         fn listen_port(&self) -> u16 {
//!             self.http.port
//!         }
//!
//!         fn route_addrs(&self) -> &[RouterAddr] {
//!             self.routers.as_slice()
//!         }
//!     }
//!
//!     pub struct HttpCfg {
//!         pub listen: IpAddr,
//!         pub port: u16,
//!     }
//!
//!     impl Default for HttpCfg {
//!         fn default() -> Self {
//!             HttpCfg {
//!                 listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
//!                 port: 1234,
//!             }
//!         }
//!     }
//! }
//!
//! mod handlers {
//!     use http_gateway::http::controller::*;
//!
//!     pub fn status(_req: &mut Request) -> IronResult<Response> {
//!         Ok(Response::with(status::Ok))
//!     }
//! }
//!
//! use config::SrvConfig;
//!
//! struct MyGatewaySrv;
//! impl HttpGateway for MyGatewaySrv {
//!     const APP_NAME: &'static str = "my-gateway";
//!
//!     type Config = SrvConfig;
//!
//!     fn router(config: Arc<Self::Config>) -> Router {
//!         router!(
//!             status: get "/status" => handlers::status,
//!         )
//!     }
//! }
//!
//! fn main() {
//!     let config = SrvConfig::default();
//!     if let Err(err) = http_gateway::start::<MyGatewaySrv>(config) {
//!         error!("{}", err);
//!         process::exit(1);
//!     }
//! }
//! ```

pub mod error;
pub mod prelude;

use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use iron;
use iron::prelude::*;

use mount::Mount;
use router::Router;
use db::data_store::DataStoreConn;
use self::error::AppResult;

use config::GatewayCfg;
use http::middleware::Cors;
use hyper_native_tls::NativeTlsServer;

/// Apply to a networked application which will act as a Gateway connecting to a RouteSrv.
pub trait HttpGateway {
    const APP_NAME: &'static str;

    type Config: GatewayCfg;

    /// Callback for adding or removing middleware to the `iron::Chain` before server start.
    fn add_middleware(Arc<Self::Config>, &mut iron::Chain, _ds: Box<DataStoreConn>) {
        ()
    }

    /// Callback for mounting additional Iron Routers before server start.
    fn mount(Arc<Self::Config>, chain: iron::Chain) -> Mount {
        let mut mount = Mount::new();
        mount.mount("/", chain);
        mount
    }

    /// Returns the Iron Router used when starting the server.
    fn router(Arc<Self::Config>, ds: Box<DataStoreConn>) -> Router;
}

/// Runs the main server and starts and manages all supporting threads. This function will
/// block the calling thread.
///
/// # Errors
///
/// * HTTP server could not start
pub fn start<T, B, A>(persister_event: (B, A), cfg: Arc<T::Config>, ds: Box<DataStoreConn>) -> AppResult<()>
where
    T: HttpGateway,
    B: iron::BeforeMiddleware,
    A: iron::AfterMiddleware,
{
    let mut chain = Chain::new(T::router(cfg.clone(), ds.clone()));
    T::add_middleware(cfg.clone(), &mut chain, ds.clone());
    chain.link_after(Cors);

    chain.link(persister_event);

    let mount = T::mount(cfg.clone(), chain);
    let mut server = Iron::new(mount);
    server.threads = cfg.handler_count();
    let https_listen_addr = (cfg.listen_addr().clone(), cfg.listen_port());

    let tls_tuple =
        cfg.tls_pair()
            .unwrap_or(("api-server.pfx".to_string(), vec![], "".to_string()));

    thread::Builder::new()
        .name("http-handler".to_string())
        .spawn(move || {
            let tls_server =
                NativeTlsServer::new(PathBuf::from(&tls_tuple.0.clone()), &tls_tuple.2.clone())
                    .unwrap();
            server.https(https_listen_addr, tls_server)
        })
        .unwrap();

    info!(
        "HTTP Gateway listening on {}:{}",
        cfg.listen_addr(),
        cfg.listen_port()
    );
    info!("{} is ready to go.", T::APP_NAME);
    Ok(())
}
