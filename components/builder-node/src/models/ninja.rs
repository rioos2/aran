// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use super::super::{NodeOutput, NodeOutputList};
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use discover::search;
use error::{Error, Result};
use postgres;
use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};
use protocol::api::node;
use protocol::api::schema::type_meta_url;
use serde_json;

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db
        }
    }

    pub fn create(&self, node_create: &node::Node) -> NodeOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_node_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(node_create.get_node_ip() as String),
                &(serde_json::to_value(node_create.get_spec()).unwrap()),
                &(serde_json::to_value(node_create.get_status()).unwrap()),
                &(serde_json::to_value(node_create.object_meta()).unwrap()),
                &(serde_json::to_value(node_create.type_meta()).unwrap()),
                &(serde_json::to_value(node_create.get_metadata()).unwrap()),
            ],
        ).map_err(Error::NodeCreate)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;

            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn show(&self, node_get: &IdGet) -> NodeOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * from get_node_v1($1)",
            &[&(node_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::NodeGet)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn show_by_node_ip(&self, node_get: &IdGet) -> NodeOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_nodes_by_node_ip_v1($1)",
            &[&(node_get.get_id() as String)],
        ).map_err(Error::NodeList)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_node(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> NodeOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_nodes_v1()", &[])
            .map_err(Error::NodeList)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_node(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &node::NodeStatusUpdate) -> NodeOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_node_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::NodeSetStatus)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn update(&self, upd_node: &node::Node) -> NodeOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_node_v1($1, $2, $3, $4, $5, $6)",
            &[
                &(upd_node.get_id().parse::<i64>().unwrap()),
                &(upd_node.get_node_ip() as String),
                &(serde_json::to_value(upd_node.get_spec()).unwrap()),
                &(serde_json::to_value(upd_node.get_status()).unwrap()),
                &(serde_json::to_value(upd_node.object_meta()).unwrap()),
                &(serde_json::to_value(upd_node.get_metadata()).unwrap()),
            ],
        ).map_err(Error::NodeUpdate)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn discovery(&self, filters: &node::NodeFilter) -> NodeOutputList {
        let ips = search::Nodes::new(filters.clone()).discovered()?;

        match Self::list_blank(self) {
            Ok(Some(node)) => {
                let mut response = Vec::new();
                ips.iter()
                    .map(|x| {
                        node.iter()
                            .map(|y| {
                                if x.to_string() == y.get_node_ip() {
                                    response.push(y.clone());
                                } else {
                                    response.push(mk_node(x));
                                }
                            })
                            .collect::<Vec<_>>();
                    })
                    .collect::<Vec<_>>();
                Ok(Some(response))
            }
            Ok(None) => {
                let mut response = Vec::new();
                ips.iter()
                    .map(|x| {
                        response.push(mk_node(x));
                    })
                    .collect::<Vec<_>>();
                Ok(Some(response))
            }
            Err(_err) => Ok(None),
        }
    }
}

fn row_to_node(row: &postgres::rows::Row) -> Result<node::Node> {
    let mut node = node::Node::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    node.set_id(id.to_string());
    node.set_node_ip(row.get("node_ip"));
    node.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    node.set_status(serde_json::from_value(row.get("status")).unwrap());
    node.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    node.set_created_at(created_at.to_rfc3339());
    Ok(node)
}

fn mk_node(ip: &str) -> node::Node {
    let mut node = node::Node::new();
    let jackie = node.who_am_i();
    let ref mut om = node.mut_meta(node.object_meta(), ip.to_string(), "".to_string());
    node.set_meta(type_meta_url(jackie), om.clone());
    node.set_node_ip(ip.to_string());

    node.set_id(
        ip.to_string()
            .chars()
            .into_iter()
            .filter(|c| char::is_alphanumeric(*c))
            .collect::<String>(),
    );
    node
}
