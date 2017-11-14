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
pub mod watch_handler;
pub mod job_handler;


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
use self::watch_handler::*;
use self::job_handler::*;

use db::data_store::*;
// use std::sync::mpsc::channel;


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
        authenticate: post "/authenticate" => XHandler::new(C(default_authenticate)),
        //auth API for login (ldap, active directory)
        authenticate_ldap: post "/authenticate/ldap/:code" => XHandler::new(C(default_authenticate)), //ldap_authenticate

        config_ldap: post "/ldap/config" => C(set_ldap_config),

        test_ldap_config: post "/ldap/config/:id/test" => C(test_ldap_config),
        import_ldap: post "/ldap/import/:id" =>C(import_ldap),

        config_saml: post "/auth/saml/providers" => C(config_saml_provider),
        saml_providers_list: get "/auth/saml/providers" =>C(saml_provider_list),
        saml_provider_id: get "/auth/saml/providers/:providerid" =>C(saml_provider_show),

        config_openid: post "/auth/oidc/providers/:providerid " => C(config_oidc_provider),
        openid_listall: get "/auth/oidc/providers" =>C(openid_listall),
        openid_show : get "auth/oidc/providers/:providerid" =>C(openid_provider_show),

        //auth API for creating new account
        signup: post "/accounts" => XHandler::new(C(account_create)),
        cli_signup: post "/force/accounts" => XHandler::new(C(account_create_from_cli)),
        account_get_by_id: get "/accounts/:id" => C(account_get_by_id),
        account_get_by_name: get "/accounts/name/:name" => C(account_get),

        //deploy API: assembly_factory
        assembly_factorys: post "/origins/:origin/assemblyfactorys" => XHandler::new(C(assembly_factory_create)).before(basic.clone()),
        assemblys_factory_show: get "/assemblyfactorys/:id" => XHandler::new(C(assembly_factory_show)).before(basic.clone()),
        assemblys_factorys_list: get "/assemblyfactorys" => XHandler::new(C(assembly_factory_list)).before(basic.clone()),
        assembly_factory_status: put "/assemblyfactorys/:id/status" => XHandler::new(C(assembly_factory_status_update)).before(basic.clone()),
        assemblyfactorys_list_by_origin : get "/origins/:origin/assemblyfactorys" => XHandler::new(C(assemblyfactorys_list_by_origin)).before(basic.clone()),
        assemblyfactorys_describe: get "/assemblyfactorys/:id/describe" => XHandler::new(C(assembly_factorys_describe)).before(basic.clone()),
        plan_list: get "/plans" => XHandler::new(C(plan_list)).before(basic.clone()),

        //deploy API: assembly
        assemblys: post "/origins/:origin/assemblys" => XHandler::new(C(assembly_create)).before(basic.clone()),
        assemblys_list: get "/assemblys" => XHandler::new(C(assembly_list)).before(basic.clone()),
        assembly_show: get "/assemblys/:id" => XHandler::new(C(assembly_show)).before(basic.clone()),
        assembly_status: put "/assemblys/:id/status" => XHandler::new(C(assembly_status_update)).before(basic.clone()),
        assembly_update: put "/assemblys/:id" => XHandler::new(C(assembly_update)).before(basic.clone()),
        assemblys_show_by_origin : get "/origins/:origin/assemblys" => XHandler::new(C(assemblys_show_by_origin)).before(basic.clone()),


        //scaling API: horizontal scaling
        horizontal_scaling: post "/origins/:origin/horizontalscaling" => XHandler::new(C(hs_create)).before(basic.clone()),
        horizontal_scaling_list: get "/horizontalscaling" => XHandler::new(C(hs_list)).before(basic.clone()),
        horizontal_scaling_status: put "/horizontalscaling/:id/status" => XHandler::new(C(hs_status_update)).before(basic.clone()),
        horizontal_scaling_update: put "/horizontalscaling/:id" => XHandler::new(C(hs_update)).before(basic.clone()),
        horizontal_scaling_metrics: get "/horizontalscaling/:id/metrics" => XHandler::new(C(hs_metrics)).before(basic.clone()),
        horizontal_scaling_list_by_origin : get "/origins/:origin/horizontalscaling" => XHandler::new(C(horizontal_scaling_list_by_origin)).before(basic.clone()),


        //authorization API: for roles
        roles: post "/roles" => XHandler::new(C(roles_create)).before(basic.clone()),
        roles_list: get "/roles" => XHandler::new(C(roles_list)).before(basic.clone()),
        roles_show: get "/roles/:id" => XHandler::new(C(roles_show)).before(basic.clone()),

        //authorization API: for permissions
        permissions: post "/permissions" => XHandler::new(C(permissions_create)).before(basic.clone()),
        permissions_list: get "/permissions" => XHandler::new(C(permissions_list)).before(basic.clone()),
        role_based_permission: get "/permissions/roles/:id" => XHandler::new(C(get_rolebased_permissions)).before(basic.clone()),
        permissions_show: get "/permissions/:id" => XHandler::new(C(permissions_show)).before(basic.clone()),
        get_specfic_permission_based_role: get "/permissions/:id/roles/:rid" => XHandler::new(C(get_specfic_permission_based_role)).before(basic.clone()),

        //node API
        nodes: post "/nodes" => XHandler::new(C(node_create)).before(basic.clone()),
        nodes_list: get "/nodes" => XHandler::new(C(node_list)).before(basic.clone()),
        node_status: put "/nodes/:id/status" => XHandler::new(C(node_status_update)).before(basic.clone()),
        node_get: get "/nodes/:id" => XHandler::new(C(node_get)).before(basic.clone()),
        node_get_by_node_ip: get "/nodes/nodeip" => XHandler::new(C(node_get_by_node_ip)).before(basic.clone()),


        //secret API
        secrets: post "/origins/:origin/secrets" => XHandler::new(C(secret_create)).before(basic.clone()),
        secrets_list: get "/secrets" => XHandler::new(C(secret_list)).before(basic.clone()),
        secret_show: get "/secrets/:id" => XHandler::new(C(secret_show)).before(basic.clone()),
        secret_show_by_origin: get "/origins/:origin/secrets" => XHandler::new(C(secret_show_by_origin)),

        //serviceAccount API
        service_accounts: post "/origins/:origin/serviceaccounts/:serviceaccount" => XHandler::new(C(service_account_create)),
        service_account_list: get "/serviceaccounts" => C(service_account_list),
        service_account_get: get "/origins/:origin/serviceaccounts/:serviceaccount" => C(service_account_show),

        //Origin API
        origins: post "/origins" => C(origin_create),
        origin_list: get "/origins" =>C(origin_list),
        origin_show: get "/origins/:origin" => C(origin_show),

        //Network API
        networks: post "/networks" => XHandler::new(C(network_create)).before(basic.clone()),
        network_list: get "/networks" => XHandler::new(C(network_list)).before(basic.clone()),
        //StorageConnectors API
        storages: post "/storageconnectors" => XHandler::new(C(storage_create)).before(basic.clone()),
        storages_list: get "/storageconnectors" => XHandler::new(C(storage_list)).before(basic.clone()),
        storages_show: get "/storageconnectors/:id" => XHandler::new(C(storage_show)).before(basic.clone()),
        storage_status: put "storageconnectors/:id/status" => XHandler::new(C(storage_status_update)).before(basic.clone()),
        storage_update: put "storageconnectors/:id" => XHandler::new(C(storage_update)).before(basic.clone()),
        storage_get_by_ip: get "/storageconnectors/ip" => XHandler::new(C(storage_get_by_ip)).before(basic.clone()),

        //StoragePool API
        storages_pool: post "/storagespool" => XHandler::new(C(storage_pool_create)).before(basic.clone()),
        storages_pool_list: get "/storagespool/:id" => XHandler::new(C(storage_pool_list)).before(basic.clone()),
        storages_pool_list_all: get "/storagespool" => XHandler::new(C(storage_pool_list_all)).before(basic.clone()),
        storages_pool_status_update: put "/storagespool/:id/status" => XHandler::new(C(storage_pool_status_update)).before(basic.clone()),


        //DataCenter API
        data_center: post "/datacenters" => XHandler::new(C(data_center_create)).before(basic.clone()),
        data_center_list: get "/datacenters" => XHandler::new(C(data_center_list)).before(basic.clone()),
        data_center_show: get "/datacenters/:id" => XHandler::new(C(data_center_show)).before(basic.clone()),

        //endpoint API

        endpoints: post "/origins/:origin/endpoints" =>  XHandler::new(C(endpoints_create)).before(basic.clone()),
        endpoints_list: get "/endpoints" =>  XHandler::new(C(endpoints_list)).before(basic.clone()),
        endpoints_show: get "/endpoints/:id" => XHandler::new(C(endpoints_show)).before(basic.clone()),
        endpoints_list_by_origin: get "/origins/:origin/endpoints" => XHandler::new(C(endpoints_list_by_origin)).before(basic.clone()),
        endpoints_get_by_assembly: get "/assemblys/:asmid/endpoints" => XHandler::new(C(endpoints_get_by_assembly)).before(basic.clone()),

        //services API

        services: post "/origins/:origin/services" => XHandler::new(C(services_create)).before(basic.clone()),
        services_show: get "/services/:id" =>XHandler::new(C(services_show)).before(basic.clone()),
        services_list: get "/services" => XHandler::new(C(services_list)).before(basic.clone()),
        services_list_by_origin: get "/origins/:origin/services" => XHandler::new(C(services_list_by_origin)).before(basic.clone()),

        plan_factory: post "/planfactory" =>XHandler::new(C(plan_factory_create)).before(basic.clone()),

        //Jobs API

        jobs: post "/jobs" => XHandler::new(C(jobs_create)).before(basic.clone()),
        jobs_get: get "/jobs" => XHandler::new(C(jobs_get)).before(basic.clone()),
        jobs_status_update: put "/jobs/:jobid/status" => XHandler::new(C(jobs_status_update)).before(basic.clone()),
        jobs_get_by_node: get "/jobs/node" => XHandler::new(C(jobs_get_by_node)).before(basic.clone()),


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

    chain.link(persistent::Read::<SecurerBroker>::both(
        SecurerConn::new(&*config),
    ));

    chain.link(persistent::Read::<DataStoreBroker>::both(
        ({
             let ds = DataStoreConn::new().unwrap();
             ds.setup(ui).unwrap().clone()
         }),
    ));
    chain.link_before(DataStoreBroker);
    chain.link_after(Custom404);

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
