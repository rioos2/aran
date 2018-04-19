// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use config::InfluxClientConn;
use protocol::api::log::{LogOutput, LogQueryBuilder};

use influx_db_client::{Client, keys};
use super::super::LogOutputList;
use error::Result;

pub struct DataStore;

impl DataStore {
    pub fn list_blank(client: &InfluxClientConn, query: &LogQueryBuilder) -> LogOutputList {
        let conn = Client::new(&client.url, &client.db());
        let res = conn.query(
            &("select * from ".to_owned() + &client.table() + " limit " +
                  &query.get_limits("limits")),
            None,
        )?;

        Ok(row_to_log(res)?)
    }

    pub fn list(client: &InfluxClientConn, query: &LogQueryBuilder) -> LogOutputList {
        let conn = Client::new(&client.url, &client.db());
        let res = conn.query(
            &("select * from ".to_owned() + &client.table() + " where (" + &client.path() + " =~ /.*" + &query.get("name") +
                  "*/)"),
            None,
        )?;
        Ok(row_to_log(res)?)
    }
}

fn row_to_log(res: Option<Vec<keys::Node>>) -> Result<Option<Vec<LogOutput>>> {
    match res {
        Some(res) => {
            let mut logs = vec![];
            res.into_iter()
                .map(|i| if i.series.is_some() {
                    i.series
                        .to_owned()
                        .unwrap()
                        .into_iter()
                        .map(|x| {
                            x.values
                                .into_iter()
                                .map(|value| {
                                    logs.push(LogOutput::with(
                                        value[0].as_str().unwrap_or(""),
                                        value[2].as_str().unwrap_or(""),
                                    ));
                                })
                                .collect::<Vec<_>>();
                        })
                        .collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
            if logs.len() != 0 {
                return Ok(Some(logs));
            }
            Ok(None)
        }
        None => Ok(None),
    }
}
