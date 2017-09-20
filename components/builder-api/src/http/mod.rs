// Copyright (c) 2017 RioCorp Inc.

//! A module containing the HTTP server and assembly_handlers for servicing client requests

pub mod deployment_handler;
pub mod scaling_handler;
pub mod authorize_handler;
pub mod auth_handler;
pub mod node_handler;
pub mod service_account_handler;


use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use rio_net::http::middleware::*;
use rio_net::auth::default::PasswordAuthClient;
use rio_net::auth::prometheus::PrometheusClient;

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
use self::service_account_handler::*;

use db::data_store::*;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>) -> Result<Chain> {
    let basic = Authenticated::new(&*config);
    //let bioshield = Shielded::new(&*config);
    //let prometheus = Prometheused::new(&*config);

    let router =
        router!(

        //the status for api server, and overall for command center
        status: get "/healthz" => status,
        healthz_all: get "/healthz/overall" => XHandler::new(healthz_all).before(basic.clone()),

        //auth API for login (default password auth)
        authenticate: post "/authenticate" => default_authenticate,
        //auth API for login (ldap, active directory)
        authenticate_ldap: post "/authenticate/ldap/:code" => default_authenticate, //ldap_authenticate

        //auth API for creating new account
        signup: post "/accounts" => account_create,

        //deploy API: assembly_factory
        assembly_factorys: post "/assemblyfactorys" => XHandler::new(assembly_factory_create).before(basic.clone()),
        assemblys_factory_show: get "/assemblyfactorys/:id" => XHandler::new(assembly_factory_show).before(basic.clone()),
        assemblys_factorys_list: get "/assemblyfactorys" => XHandler::new(assembly_factory_list).before(basic.clone()),
        assembly_factory_status: put "/assemblyfactorys/:id/status" => XHandler::new(assembly_factory_status_update).before(basic.clone()),

        //deploy API: assembly
        assemblys: post "/assemblys" => XHandler::new(assembly_create).before(basic.clone()),
        assemblys_list: get "/assemblys" => XHandler::new(assembly_list).before(basic.clone()),
        assembly_show: get "/assemblys/:id" => XHandler::new(assembly_show).before(basic.clone()),
        assembly_status: put "/assemblys/:id/status" => XHandler::new(assembly_status_update).before(basic.clone()),

        //scaling API: horizontal scaling
        horizontal_scaling: post "/horizontalscaling" => XHandler::new(hs_create).before(basic.clone()),
        horizontal_scaling_list: get "/horizontalscaling" => XHandler::new(hs_list).before(basic.clone()),
        horizontal_scaling_status: put "/horizontalscaling/:id/status" => XHandler::new(hs_status_update).before(basic.clone()),

        //authorization API: for roles
        roles: post "/roles" => XHandler::new(roles_create).before(basic.clone()),
        roles_list: get "/roles" => XHandler::new(roles_list).before(basic.clone()),
        roles_show: get "/roles/:id" => XHandler::new(roles_show).before(basic.clone()),

        //authorization API: for permissions
        permissions: post "/permissions" => XHandler::new(permissions_create).before(basic.clone()),
        permissions_list: get "/permissions" => XHandler::new(permissions_list).before(basic.clone()),
        role_based_permission: get "/permissions/roles/:id" => XHandler::new(get_rolebased_permissions).before(basic.clone()),
        permissions_show: get "/permissions/:id" => XHandler::new(permissions_show).before(basic.clone()),
        get_specfic_permission_based_role: get "/permissions/:id/roles/:rid" => XHandler::new(get_specfic_permission_based_role).before(basic.clone()),

        //node API
        nodes: post "/nodes" => XHandler::new(node_create).before(basic.clone()),
        nodes_list: get "/nodes" => XHandler::new(node_list).before(basic.clone()),
        node_status: put "/nodes/:id/status" => XHandler::new(node_status_update).before(basic.clone()),

        //secret API
        secrets: post "/secret" => XHandler::new(secret_create).before(basic.clone()),
        secret_show: get "/secret/:id" => XHandler::new(secret_show).before(basic.clone()),

        //serviceAccount API
        service_accounts: post "/origins/:origin/serviceaccounts/:serviceaccount" => XHandler::new(service_create).before(basic.clone()),

    );

    let mut chain = Chain::new(router);

    chain.link(persistent::Read::<PasswordAuthCli>::both(
        PasswordAuthClient::new(&*config),
    ));

    chain.link(persistent::Read::<PrometheusCli>::both(
        PrometheusClient::new(&*config),
    ));

    chain.link(persistent::Read::<DataStoreBroker>::both(
        ({
             let ds = DataStoreConn::new().unwrap();
             ds.setup().unwrap().clone()
         }),
    ));

    chain.link_before(DataStoreBroker);

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
