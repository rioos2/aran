// Copyright 2018 The Rio Advancement Inc
//
#[macro_use]
extern crate log;

extern crate chrono;
extern crate regex;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate linked_hash_map;
extern crate time;

pub mod api;
pub mod error;
pub mod cache;
pub mod sharding;
pub use self::error::{Error, Result};

pub use self::sharding::{ShardId, SHARD_COUNT, InstaId};
