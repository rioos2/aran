// Copyright (c) 2017 RioCorp Inc.
//

extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;

pub mod error;
pub mod net;
pub mod asmsrv;
pub mod sessionsrv;
pub mod sharding;
pub mod originsrv;
pub mod authsrv;
pub mod scalesrv;
pub mod routesrv;
pub mod nodesrv;
pub mod plansrv;
pub mod servicesrv;
pub mod netsrv;
pub mod storagesrv;
pub mod jobsrv;

pub use self::error::{Error, Result};
pub use self::sharding::{ShardId, SHARD_COUNT, InstaId};

pub const DEFAULT_API_VERSION: &'static str = "v1";
