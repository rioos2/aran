// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use message::{Persistable, Routable};
use protobuf::{ProtobufEnum, RepeatedField};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sharding::InstaId;
use std::result;
use std::str::FromStr;

pub use message::asmsrv::*;

impl Serialize for Assembly {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = try!(serializer.serialize_struct("assembly", 10));
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field("name", &self.get_name().to_string())?;
        strukt.serialize_field("uri", &self.get_uri().to_string())?;
        strukt.serialize_field("description", &self.get_description().to_string())?;
        strukt.serialize_field("tags", &self.get_tags().to_string())?;
        strukt.serialize_field("representation_skew", &self.get_representation_skew().to_string())?;
        strukt.serialize_field("external_management_resource", &self.get_external_management_resource().to_string())?;
        strukt.serialize_field("component_collection", &self.get_component_collection().to_string())?;
        strukt.serialize_field("plan", &self.get_plan().to_string())?;
        strukt.serialize_field("operation_collection", &self.get_operation_collection().to_string())?;
        strukt.serialize_field("sensor_collection", &self.get_sensor_collection().to_string())?;
        strukt.serialize_field("metadata", &self.get_metadata().to_string())?;
        strukt.serialize_field("created_at", &self.get_created_at())?;
        strukt.serialize_field("updated_at", &self.get_updated_at())?;

        strukt.end()
    }
}

#[derive(Debug)]
pub enum Error {
    BadJobState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_ansi_stripping() {
        let mut log = JobLog::new();
        log.set_is_complete(false);
        log.set_start(0);
        log.set_stop(4);

        let lines = vec![
            "[1;33m» Importing origin key from standard log[0m",
            "[1;34m★ Imported secret origin key core-20160810182414.[0m",
            "[1;33m» Installing core/hab-backline[0m",
            "[1;32m↓ Downloading[0m core/hab-backline/0.23.0/20170511220008",
        ];

        let input_lines = lines.iter().map(|l| l.to_string());
        let content = RepeatedField::from_iter(input_lines);
        log.set_content(content);

        log.strip_ansi();

        let stripped_lines: Vec<String> = log.get_content()
            .into_iter()
            .map(|l| l.to_string())
            .collect();

        let expected = vec![
            "» Importing origin key from standard log",
            "★ Imported secret origin key core-20160810182414.",
            "» Installing core/hab-backline",
            "↓ Downloading core/hab-backline/0.23.0/20170511220008",
        ];
        assert_eq!(stripped_lines, expected);
    }

}
