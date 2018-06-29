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
extern crate woothee;

extern crate chrono_humanize;
extern crate linked_hash_map;
extern crate time;

pub mod api;
pub mod cache;
pub mod error;
pub mod sharding;
pub use self::error::{Error, Result};

pub use self::sharding::{InstaId, ShardId, SHARD_COUNT};
