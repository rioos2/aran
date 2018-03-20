use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::team;
use protocol::api::base::MetaFields;

use postgres;
use db::data_store::DataStoreConn;
use serde_json;
pub struct TeamDS;

impl TeamDS {
    pub fn create(datastore: &DataStoreConn, team_create: &team::Team) -> Result<Option<team::Team>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_team_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(team_create.get_name() as String),
                &(team_create.get_metadata().get("origin").unwrap()),
                &(serde_json::to_value(team_create.object_meta()).unwrap()),
                &(serde_json::to_value(team_create.type_meta()).unwrap()),
                &(serde_json::to_value(team_create.get_metadata()).unwrap()),
                &(serde_json::to_value(team_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::TeamCreate)?;
        if rows.len() > 0 {
            let team = row_to_team(&rows.get(0))?;
            return Ok(Some(team));
        }
        Ok(None)
    }
}

fn row_to_team(row: &postgres::rows::Row) -> Result<team::Team> {
    let mut team_data = team::Team::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    team_data.set_team_name(row.get("name"));
    team_data.set_id(id.to_string() as String);
    team_data.set_metadata(serde_json::from_value(row.get("meta_data")).unwrap());
    team_data.set_created_at(created_at.to_rfc3339());
    Ok(team_data)
}
