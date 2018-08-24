use super::ninja;
use super::senseis as db_senseis;
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::Result;
use protocol::api::{node, senseis};
use protocol::api::base::MetaFields;
use std::collections::BTreeMap;
use std::ops::Div;
use telemetry::metrics::collector::Collector;
use telemetry::metrics::executer::Executer;
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
        let querys = QueryMaker::new().build_consumption_in_datacenter();
        let res = Executer::new(self.client.clone()).pull_metrics(querys)?;
        let mut response = Collector::new(res).get_reports();
        let new_ninjas = self.merge_live_ninjas(response.get_statistics().get_ninjas());
        let new_senseis = self.merge_live_senseis(response.get_statistics().get_senseis());
        response.set_statistics(new_statistics(new_ninjas, new_senseis));
        Ok(Some(response.into()))
    }

    fn merge_live_ninjas(&self, live: Vec<node::NodeStatistic>) -> Vec<node::NodeStatistic> {
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

    fn merge_live_senseis(&self, live: Vec<node::NodeStatistic>) -> Vec<node::NodeStatistic> {
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

fn new_statistics(new_ninjas: Vec<node::NodeStatistic>, new_senseis: Vec<node::NodeStatistic>) -> node::Statistics {
    let mut statistics = node::Statistics::new();
    statistics.set_title("Statistics".to_string());
    statistics.set_ninjas(new_ninjas);
    statistics.set_senseis(new_senseis);
    statistics
}
