// Copyright (c) 2017 RioCorp Inc.

//! A module containing the HTTP server and assembly_handlers for servicing client requests

pub mod deployment_handler;
pub mod scaling_handler;
pub mod authorize_handler;
pub mod auth_handler;
pub mod node_handler;
pub mod service_account_handler;
pub mod origin_handler;
pub mod network_handler;
pub mod storage_handler;
pub mod plan_handler;
pub mod watch_handler;

use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use common::ui::UI;
use rio_net::http::middleware::*;
use rio_net::auth::default::PasswordAuthClient;
use rio_net::metrics::prometheus::PrometheusClient;

// turn it on later
//use rio_core::event::EventLogger;

use iron::prelude::*;
use hyper_native_tls::NativeTlsServer;
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
use self::origin_handler::*;
use self::service_account_handler::*;
use self::network_handler::*;
use self::storage_handler::*;
use self::plan_handler::*;
use self::watch_handler::*;

use db::data_store::*;
use std::sync::mpsc::channel;


// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

const TLS_PKCS12_PWD: &'static str = "RIO123";

#[allow(unused_must_use)]
/// Create a new `iron::Chain` containing a Router and it's required middleware
pub fn router(config: Arc<Config>, ui: &mut UI) -> Result<Chain> {
    let basic = Authenticated::new(&*config);
    //let bioshield = Shielded::new(&*config);
    ui.begin("Router ");
    let router =
        router!(

        //the status for api server, and overall for command center
        status: get "/healthz" => status,
        healthz_all: get "/healthz/overall" => XHandler::new(C(healthz_all)).before(basic.clone()),

        //auth API for login (default password auth)
        authenticate: post "/authenticate" => default_authenticate,
        //auth API for login (ldap, active directory)
        authenticate_ldap: post "/authenticate/ldap/:code" => default_authenticate, //ldap_authenticate

        config_ldap: post "/ldap/config" => set_ldap_config,

        test_ldap_config: post "/ldap/config/:id/test" => test_ldap_config,

        config_saml: post "/auth/saml/providers" => config_saml_provider,
        saml_providers_list: get "/auth/saml/providers" =>saml_provider_list,
        saml_provider_id: get "/auth/saml/providers/:providerid" =>saml_provider_show,

        config_openid: post "/auth/oidc/providers/:providerid " => config_oidc_provider,
        openid_listall: get "/auth/oidc/providers" =>openid_listall,
        // openid_show : get "auth/oidc/providers/:providerid" =>openid_provider_show,

        //auth API for creating new account
        signup: post "/accounts" => account_create,
        account_get_by_id: get "/accounts/:id" => account_get_by_id,
        account_get_by_name: get "/accounts/name/:name" => account_get,

        //deploy API: assembly_factory
        assembly_factorys: post "/assemblyfactorys" => XHandler::new(assembly_factory_create).before(basic.clone()),
        assemblys_factory_show: get "/assemblyfactorys/:id" => XHandler::new(assembly_factory_show).before(basic.clone()),
        assemblys_factorys_list: get "/assemblyfactorys" => XHandler::new(assembly_factory_list).before(basic.clone()),
        assembly_factory_status: put "/assemblyfactorys/:id/status" => XHandler::new(assembly_factory_status_update).before(basic.clone()),
        plan_list: get "/plans" => XHandler::new(plan_list).before(basic.clone()),

        //deploy API: assembly
        assemblys: post "/assemblys" => XHandler::new(assembly_create).before(basic.clone()),
        assemblys_list: get "/assemblys" => XHandler::new(assembly_list).before(basic.clone()),
        assembly_show: get "/assemblys/:id" => XHandler::new(assembly_show).before(basic.clone()),
        assembly_status: put "/assemblys/:id/status" => XHandler::new(assembly_status_update).before(basic.clone()),
        assembly_update: put "/assemblys/:id" => XHandler::new(assembly_update).before(basic.clone()),

        //scaling API: horizontal scaling
        horizontal_scaling: post "/horizontalscaling" => XHandler::new(hs_create).before(basic.clone()),
        horizontal_scaling_list: get "/horizontalscaling" => XHandler::new(hs_list).before(basic.clone()),
        horizontal_scaling_status: put "/horizontalscaling/:id/status" => XHandler::new(hs_status_update).before(basic.clone()),
        horizontal_scaling_update: put "/horizontalscaling/:id" => XHandler::new(hs_update).before(basic.clone()),
        horizontal_scaling_metrics: get "/horizontalscaling/:id/metrics" => XHandler::new(hs_metrics).before(basic.clone()),


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
        secrets: post "/secrets" => XHandler::new(C(secret_create)).before(basic.clone()),
        secrets_list: get "/secrets" => XHandler::new(secret_list),
        secret_show: get "/secrets/:id" => XHandler::new(C(secret_show)).before(basic.clone()),
        secret_show_by_origin: get "/origins/:origin/secrets" => XHandler::new(secret_show_by_origin),

        //serviceAccount API
        service_accounts: post "/origins/:origin/serviceaccounts/:serviceaccount" => XHandler::new(service_account_create),
        service_account_list: get "/serviceaccounts" => service_account_list,
        service_account_get: get "/origins/:origin/serviceaccounts/:serviceaccount" => service_account_show,

        //Origin API
        origins: post "/origins" => origin_create,
        origin_list: get "/origins" =>origin_list,
        origin_show: get "/origins/:origin" => origin_show,

        //Network API
        networks: post "/networks" => XHandler::new(network_create).before(basic.clone()),
        network_list: get "/networks" => XHandler::new(network_list).before(basic.clone()),

        //StorageConnectors API
        storages: post "/storageconnectors" => XHandler::new(storage_create).before(basic.clone()),
        storages_list: get "/storageconnectors" => XHandler::new(storage_list).before(basic.clone()),
        storages_show: get "/storageconnectors/:id" => XHandler::new(storage_show).before(basic.clone()),
        storage_status: put "storageconnectors/:id/status" => XHandler::new(storage_status_update).before(basic.clone()),
        storage_update: put "storageconnectors/:id" => XHandler::new(storage_update).before(basic.clone()),

        //StoragePool API
        storages_pool: post "/storagespool" => XHandler::new(storage_pool_create).before(basic.clone()),
        storages_pool_list: get "/storagespool/:id" => XHandler::new(storage_pool_list).before(basic.clone()),
        storages_pool_list_all: get "/storagespool" => XHandler::new(storage_pool_list_all).before(basic.clone()),
        storages_pool_status_update: put "/storagespool/:id/status" => XHandler::new(storage_pool_status_update).before(basic.clone()),


        //DataCenter API
        data_center: post "/datacenters" => XHandler::new(data_center_create).before(basic.clone()),
        data_center_list: get "/datacenters" => XHandler::new(data_center_list).before(basic.clone()),
        data_center_show: get "/datacenters/:id" => XHandler::new(data_center_show).before(basic.clone()),

        plan_factory: post "/planfactory" =>XHandler::new(plan_factory_create).before(basic.clone()),

        //Internal: Streaming watch
        watches: get "/:name/watch/list" => watch_show,

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
             ds.setup(ui).unwrap().clone()
         }),
    ));

    chain.link_before(DataStoreBroker);

    chain.link_after(Cors);

    ui.end("Router ");
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
pub fn run(config: Arc<Config>, ui: &mut UI) -> Result<JoinHandle<()>> {
    let (tx, rx) = mpsc::sync_channel(1);

    let mut mount = Mount::new();

    if let Some(ref path) = config.ui.root {
        debug!("Mounting UI at filepath {}", path);
        mount.mount("/", Static::new(path));
    }

    let chain = try!(router(config.clone(), ui));
    mount.mount("/api/v1", chain);

    let handle = thread::Builder::new()
        .name("http-srv".to_string())
        .spawn(move || {
            let mut server = Iron::new(mount);
            server.threads = HTTP_THREAD_COUNT;

            match config.http.tls_pkcs12_file {
                Some(ref tls_location) => {
                    let tls = NativeTlsServer::new(tls_location, TLS_PKCS12_PWD).unwrap();
                    server.https(&config.http, tls).unwrap()
                }
                None => server.http(&config.http).unwrap(),
            };

            tx.send(()).unwrap();
        })
        .unwrap();
    match rx.recv() {
        Ok(()) => Ok(handle),
        Err(e) => panic!("http-srv thread startup error, err={}", e),
    }
}
