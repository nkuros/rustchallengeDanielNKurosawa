use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use std::collections::BTreeMap as Map;

use crate::database::schema::nodes;


#[diesel(table_name = nodes)]
#[derive(Debug, Queryable, Insertable, Clone)]
pub struct NodeDB {
    pub public_key:  String,
    pub alias:  String,
    pub channels: i64,
    pub capacity: i64,
    pub first_seen: i64,
    pub updated_at: i64,

}

impl NodeDB {
    pub fn from_db(&self) -> NodeFromDB {
        NodeFromDB {
            public_key: self.public_key.clone(),
            alias: self.alias.clone(),
            channels: self.channels,
            capacity: self.capacity as f64/100_000_000.0,
            first_seen: DateTime::from_timestamp_nanos(self.first_seen),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]

pub struct NodeToDB{
    public_key: String,
    alias: String,
    channels: i64,
    capacity: i64,
    first_seen: i64,
    updated_at: i64,
    city: Option<Map<String, String>>,
    country: Option<Map<String, String>>,
}

impl NodeToDB {
    pub fn to_node_db(&self) -> NodeDB {
        NodeDB {
            public_key: self.public_key.clone(),
            alias: self.alias.clone(),
            channels: self.channels,
            capacity: self.capacity,
            first_seen: self.first_seen,
            updated_at: self.updated_at, 
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeFromDB{
    public_key: String,
    alias: String,
    channels: i64,
    capacity: f64,
    first_seen: DateTime<Utc>,
}


