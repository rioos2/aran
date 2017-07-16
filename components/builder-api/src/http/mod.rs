// Copyright (c) 2017 RioCorp Inc.

//! A module containing the HTTP server and assembly_handlers for servicing client requests

pub mod deployment_handler;

use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use hab_net::http::middleware::*;
use hab_core::event::EventLogger;
use iron::prelude::*;
use mount::Mount;
use persistent::{self, Read};
use staticfile::Static;

use config::Config;
use error::Result;
use self::assembly_handlers::*;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>) -> Result<Chain> {
    let basic = Authenticated::new(&*config);
    let router =
        router!(
        status: get "/status" => status,

        // assemblys: post "/assemblys" => XHandler::new(assembly_create).before(basic.clone()),
        assemblys: post "/assemblys" => assembly_create,
        assembly: get "/assemblys/:id" => assembly_show,

        assembly_factorys: post "/assembly_factorys" => assembly_create,
        assembly_factory: get "/assembly_factory/:id" => assembly_show,

    );

    let mut chain = Chain::new(router);

    //TO-DO: I am thinking to stick the Datastore here, which will be created for every request.
    //TO-DO: Just watch the number of Datastore connections in cockroachdb UI
    //TO-DO: Just change the GithubCli to Datastore, Grab the Datastore code from builder_deployment
    chain.link(persistent::Read::<GitHubCli>::both(
        GitHubClient::new(&*config),
    ));

    chain.link(Read::<EventLog>::both(
        EventLogger::new(&config.log_dir, config.events_enabled),
    ));

    chain.link_after(Cors);
    Ok(chain)
}

/// Create a new HTTP listener and run it in a separate thread. This function will block the calling
/// thread until the new listener has successfully started.
///
/// # Errors
///
/// * Couldn't create Router or it's middleware
///
/// # Panics
///
/// * Listener crashed during startup
pub fn run(config: Arc<Config>) -> Result<JoinHandle<()>> {
    let (tx, rx) = mpsc::sync_channel(1);

    let mut mount = Mount::new();

    //TO-DO: I think we don't have a / URL, but we'll probably show some static files. 
    if let Some(ref path) = config.ui.root {
        debug!("Mounting UI at filepath {}", path);
        mount.mount("/", Static::new(path));
    }
    let chain = try!(router(config.clone()));
    mount.mount("/v1", chain);

    let handle = thread::Builder::new()
        .name("http-srv".to_string())
        .spawn(move || {
            let mut server = Iron::new(mount);
            server.threads = HTTP_THREAD_COUNT;
            server.http(&config.http).unwrap();
            tx.send(()).unwrap();
        })
        .unwrap();
    match rx.recv() {
        Ok(()) => Ok(handle),
        Err(e) => panic!("http-srv thread startup error, err={}", e),
    }
}
