// Copyright 2018 The Rio Advancement Inc
//

use iron::prelude::*;
use std::collections::BTreeMap;
use urlencoded::UrlEncodedQuery;

pub fn extract_query_value(req: &mut Request) -> Option<BTreeMap<String, String>> {
    let mut collections = BTreeMap::new();
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref map) => {
            if !map.is_empty() {
                for (k, v) in map.iter() {
                    collections.insert(k.to_owned(), v[0].to_owned());
                }
                return Some(collections);
            }
            None
        }
        Err(_) => None,
    }
}
