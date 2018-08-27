use super::ninja;
use super::senseis as db_senseis;
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::Result;
use protocol::api::{node, senseis};
use protocol::api::base::MetaFields;
use serde_json;
use std::collections::BTreeMap;
use std::ops::Div;
use telemetry::metrics::executer::Executer;
use telemetry::metrics::hooks::before::AHooks;
use telemetry::metrics::prometheus::PrometheusClient;
use telemetry::metrics::query::QueryMaker;

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
    client: &'a PrometheusClient,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn, client: &'a PrometheusClient) -> Self {
        DataStore {
            db: db,
            client: client,
        }
    }

    pub fn healthz_all(&self) -> Result<Option<node::HealthzAllGetResponse>> {
        let mut querys = QueryMaker::new();
        let executer = Executer::new(self.client.clone());
        let res = executer.execute(querys.build_consumption_in_datacenter())?;
        let result = executer.execute_range(&querys.snapshot_os_usage())?;
        let response = self.get_reports(res, result);
        Ok(Some(response.into()))
    }

    fn get_reports(&self, response: AHooks, result: Vec<node::Item>) -> node::HealthzAllGet {
        let mut x = node::HealthzAllGet::new();
        x.set_title("Command center operations".to_string());
        x.set_gauges(mk_guages(&response));
        x.set_statistics(mk_statistics(
            self.merge_live_ninjas(&response),
            self.merge_live_senseis(&response),
        ));
        x.set_osusages(mk_os_usage(&response, result));
        x
    }

    fn merge_live_ninjas(&self, response: &AHooks) -> Vec<node::NodeStatistic> {
        let live: Vec<node::NodeStatistic> = serde_json::from_str(&response.get(node::NINJAS).unwrap()).unwrap();
        match ninja::DataStore::new(self.db).list_blank() {
            Ok(Some(node)) => {
                let mut response = Vec::new();
                node.iter()
                    .map(|x| {
                        if live.is_empty() {
                            response.push(x.clone().into());
                        }
                        let mut node = node::NodeStatistic::new();
                        live.iter()
                            .map(|y| if x.get_id() == y.get_id() {
                                node = y.clone();
                                response.push(y.clone());
                            })
                            .collect::<Vec<_>>();
                        if node.get_id() != x.get_id() && !live.is_empty() {
                            response.push(x.clone().into());
                        }
                    })
                    .collect::<Vec<_>>();
                response
            }
            Ok(None) => live,
            Err(_err) => live,
        }
    }

    fn merge_live_senseis(&self, response: &AHooks) -> Vec<node::NodeStatistic> {
        let live: Vec<node::NodeStatistic> = serde_json::from_str(&response.get(node::SENSEIS).unwrap()).unwrap();
        match db_senseis::DataStore::new(self.db).list_blank() {
            Ok(Some(node)) => {
                let mut response = Vec::new();
                node.iter()
                    .map(|x| {
                        if live.is_empty() {
                            response.push(x.clone().into());
                        }
                        let mut node = node::NodeStatistic::new();
                        live.iter()
                            .map(|y| if x.get_id() == y.get_id() {
                                node = y.clone();
                                response.push(y.clone());
                            })
                            .collect::<Vec<_>>();
                        if node.get_id() != x.get_id() && !live.is_empty() {
                            response.push(x.clone().into());
                        }
                    })
                    .collect::<Vec<_>>();
                response
            }
            Ok(None) => live,
            Err(_err) => live,
        }
    }
}

fn mk_guages(response: &AHooks) -> node::Guages {
    vec![
        serde_json::from_str(&response.get(node::CAPACITY_CPU).unwrap()).unwrap(),
        serde_json::from_str(&response.get(node::CAPACITY_MEMORY).unwrap()).unwrap(),
        serde_json::from_str(&response.get(node::CAPACITY_STORAGE).unwrap()).unwrap(),
    ].into()
}


fn mk_statistics(new_ninjas: Vec<node::NodeStatistic>, new_senseis: Vec<node::NodeStatistic>) -> node::Statistics {
    let mut statistics = node::Statistics::new();
    statistics.set_title("Statistics".to_string());
    statistics.set_ninjas(new_ninjas);
    statistics.set_senseis(new_senseis);
    statistics
}

fn mk_os_usage(response: &AHooks, result: Vec<node::Item>) -> node::OSUsages {
    let mut os_usage = node::OSUsages::new();
    os_usage.set_title("OS Usages".to_owned());
    os_usage.set_cumulative(
        serde_json::from_str(&response.get(node::CUMULATIVE_OS_USAGE).unwrap()).unwrap(),
    );
    os_usage.set_items(result);
    os_usage
}
