// Copyright 2018 The Rio Advancement Inc
//

extern crate crypto;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_builder_servicesrv as secret;
extern crate rioos_builder_session as session;
extern crate rioos_builder_servicesrv as serviceaccount;
extern crate rioos_common as common;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rand;
extern crate iron;
extern crate base64;

extern crate openssl;
   
#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[cfg(not(test))]
extern crate serde_json;

pub mod rioos;
pub mod util;
pub mod error;