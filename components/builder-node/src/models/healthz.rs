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
        let mut mk_query = QueryMaker::new(self.client);
        let querys = mk_query.build_consumption_in_datacenter();
        let res = Collector::new(mk_query.pull_metrics(querys)?).get_reports();
        Ok(Some(res.into()))
    }
}
