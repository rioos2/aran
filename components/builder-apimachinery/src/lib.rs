// Copyright 2018 The Rio Advancement Inc
//
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate iron;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate linked_hash_map;
extern crate time;
extern crate chrono_humanize;

pub mod api;
pub mod error;
pub mod cache;
pub mod sharding;
pub use self::error::{Error, Result};

pub use self::sharding::{ShardId, SHARD_COUNT, InstaId};
