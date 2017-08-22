// Copyright (c) 2017 RioCorp Inc.

//! A module containing the HTTP server and assembly_handlers for servicing client requests

pub mod deployment_handler;
pub mod scaling_handler;
pub mod authorize_handler;
pub mod auth_handler;
pub mod node_handler;


use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use rio_net::http::middleware::*;
use rio_net::auth::default::PasswordAuthClient;

// turn it on later
//use rio_core::event::EventLogger;

use iron::prelude::*;
use mount::Mount;
use persistent;
use staticfile::Static;

use config::Config;
use error::Result;
use self::deployment_handler::*;
use self::auth_handler::*;
use self::scaling_handler::*;
use self::authorize_handler::*;
use self::node_handler::*;


use db::data_store::*;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>) -> Result<Chain> {
    let basic = Authenticated::new(&*config);
    //let bioshield = Shielded::new(&*config);

    let router =
        router!(
        status: get "/healthz" => status,

        //auth API for login (default password auth)
        authenticate: post "/authenticate" => default_authenticate,
        //auth API for login (ldap, active directory)
        authenticate_ldap: post "/authenticate/ldap/:code" => default_authenticate, //ldap_authenticate

        //auth API for creating new account
        signup: post "/accounts" => account_create,

        //deploy API: assembly_factory
        assembly_factorys: post "/assemblyfactorys" => assembly_factory_create,
        assemblys_factory: get "/assemblyfactorys/:id" => assembly_factory_show,
        assemblys_factorys_get: get "/assemblyfactorys" => assembly_factory_list,
        assembly_factory_status: put "/assemblyfactorys/status/:id" => assembly_factory_status_update,

        //deploy API: assembly
        // assemblys: post "/assemblys" => XHandler::new(assembly_create).before(basic.clone()),

        assemblys: post "/assemblys" => assembly_create,
        assemblys_get: get "/assemblys" => assembly_list,
        assembly: get "/assemblys/:id" => assembly_show,
        assembly_status: put "/assemblys/status/:id" => assembly_status_update,

        //scaling API: horizontal scaling
        horizontal_scaling: post "/horizontalscaling" => hs_create,
        horizontal_scaling_list: get "/horizontalscaling" => hs_list,
        horizontal_scaling_status: put "/horizontalscaling/status/:id" => hs_status_update,

        //authorization API: for roles
        roles: post "/roles" =>roles_create,
        roles_list: get "/roles" =>roles_list,
        roles_show: get "/roles/:id" =>roles_show,

        //authorization API: for permissions
        permissions: post "/permissions" =>permissions_create,
        permissions_list: get "/permissions" => permissions_list,
        role_based_permission: get "/permissions/roles/:id" => get_rolebased_permissions,
        permissions_show: get "/permissions/:id" => permissions_show,
        get_specfic_permission_based_role: get "/permissions/:id/roles/:rid" => get_specfic_permission_based_role,

        //node API
        nodes: post "/nodes" => node_create,

    );

    let mut chain = Chain::new(router);

    chain.link(persistent::Read::<PasswordAuthCli>::both(
        PasswordAuthClient::new(&*config),
    ));

    chain.link(persistent::Read::<DataStoreBroker>::both(
        ({
             let ds = DataStoreConn::new().unwrap();
             ds.setup().unwrap().clone()
         }),
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
    mount.mount("/api/v1", chain);

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
