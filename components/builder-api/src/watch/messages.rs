// Copyright 2018 The Rio Advancement Inc
//
use api::security::config::SecurerConn;
use api::{cluster, deploy, devtooling, security};
use bytes::Bytes;
use db::data_store::DataStoreConn;
use protocol::api::base::IdGet;
use telemetry::metrics::prometheus::PrometheusClient;

//which is help for build response structure and which type of response
//handler handle this enum
custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum Messages {
        Secrets,
        Networks,
        Jobs,
        Storagespool,
        Storageconnectors,
        Datacenters,
        Horizontalscaling,
        Verticalscaling,
        Settingsmap,
        Endpoints,
        Origins,
        Nodes,
        Senseis,
        Plans,
        Services,
        Serviceaccounts,
        Assemblyfactorys,
        Assemblys,
        Builds,
        Buildconfigs,
    }
}

pub fn handle_assembly(
    idget: IdGet,
    typ: String,
    datastore: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
) -> Bytes {
    let mut assembly = deploy::assembly::AssemblyApi::new(datastore, prom);
    assembly.watch(idget, typ)
}

pub fn handle_assembly_list(
    idget: IdGet,
    datastore: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
) -> Option<String> {
    let mut assembly = deploy::assembly::AssemblyApi::new(datastore, prom);
    assembly.watch_list_by_account(idget, "GET:accountsassemblys".to_string())
}

pub fn handle_assemblyfactory(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut assembly_factory = deploy::assembly_factory::AssemblyFactoryApi::new(datastore);
    assembly_factory.watch(idget, typ)
}

pub fn handle_assemblyfactory_list(idget: IdGet, datastore: Box<DataStoreConn>) -> Option<String> {
    let mut assembly_factory = deploy::assembly_factory::AssemblyFactoryApi::new(datastore);
    assembly_factory.watch_list_by_account(idget, "GET:accountsassemblyfactorys".to_string())
}

pub fn handle_services(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut services = deploy::service::ServiceApi::new(datastore);
    services.watch(idget, typ)
}

pub fn handle_nodes(
    idget: IdGet,
    typ: String,
    datastore: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
) -> Bytes {
    let mut node = cluster::node_api::NodeApi::new(datastore, prom);
    node.watch(idget, typ)
}
pub fn handle_senseis(idget: IdGet,typ: String,datastore: Box<DataStoreConn>) -> Bytes {
    let mut sensei = cluster::senseis_api::SenseisApi::new(datastore);
    sensei.watch(idget, typ)
}

pub fn handle_secrets(
    idget: IdGet,
    typ: String,
    datastore: Box<DataStoreConn>,
    securer: Box<SecurerConn>,
) -> Bytes {
    let mut secret = security::secret_api::SecretApi::new(datastore, securer);
    secret.watch(idget, typ)
}

pub fn handle_secrets_list(
    idget: IdGet,
    datastore: Box<DataStoreConn>,
    securer: Box<SecurerConn>,
) -> Option<String> {
    let secret = security::secret_api::SecretApi::new(datastore, securer);
    secret.watch_list_by_account(idget, "GET:accountssecrets".to_string())
}

pub fn handle_jobs(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut job = deploy::job::JobApi::new(datastore);
    job.watch(idget, typ)
}

pub fn handle_horizontalscaling(
    idget: IdGet,
    typ: String,
    datastore: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
) -> Bytes {
    let mut hscale = deploy::horizontalscaling::HorizontalScalingApi::new(datastore, prom);
    hscale.watch(idget, typ)
}

pub fn handle_networks(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut network = cluster::network_api::NetworkApi::new(datastore);
    network.watch(idget, typ)
}

pub fn handle_storagespool(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut storagespool = cluster::storage_api::StorageApi::new(datastore);
    storagespool.storage_pool_watch(idget, typ)
}

pub fn handle_storageconnectors(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut storageconnectors = cluster::storage_api::StorageApi::new(datastore);
    storageconnectors.watch(idget, typ)
}

pub fn handle_datacenters(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut datacenters = cluster::storage_api::StorageApi::new(datastore);
    datacenters.data_center_watch(idget, typ)
}

pub fn handle_verticalscaling(
    idget: IdGet,
    typ: String,
    datastore: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
) -> Bytes {
    let mut verticalscaling = deploy::vertical_scaling::VerticalScalingApi::new(datastore, prom);
    verticalscaling.watch(idget, typ)
}

pub fn handle_settingsmap(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut settingsmap = security::settings_map_api::SettingsMapApi::new(datastore);
    settingsmap.watch(idget, typ)
}

pub fn handle_endpoints(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut endpoints = deploy::endpoint::EndpointApi::new(datastore);
    endpoints.watch(idget, typ)
}

pub fn handle_origins(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut origins = deploy::origin::OriginApi::new(datastore);
    origins.watch(idget, typ)
}

pub fn handle_plans(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut plans = deploy::plan_factory::PlanFactory::new(datastore);
    plans.watch(idget, typ)
}

pub fn handle_serviceaccounts(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut serviceaccounts = security::service_account_api::SeriveAccountApi::new(datastore);
    serviceaccounts.watch(idget, typ)
}

pub fn handle_builds(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut builds = devtooling::build::BuildApi::new(datastore);
    builds.watch(idget, typ)
}

pub fn handle_builds_config(idget: IdGet, typ: String, datastore: Box<DataStoreConn>) -> Bytes {
    let mut build_conf = devtooling::build_config::BuildConfigApi::new(datastore);
    build_conf.watch(idget, typ)
}
