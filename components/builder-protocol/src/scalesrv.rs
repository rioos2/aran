use message::{Persistable, Routable};
use protobuf::{ProtobufEnum, RepeatedField};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sharding::InstaId;
use std::result;
use std::str::FromStr;

pub use message::scalesrv::*;

impl Serialize for HorizontalScaling {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = try!(serializer.serialize_struct("horizontal_scaling", 10));
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field("name", &self.get_name().to_string())?;
        strukt.serialize_field(
            "description",
            &self.get_description().to_string(),
        )?;
        strukt.serialize_field("tags", &self.get_tags())?;
        strukt.serialize_field(
            "hs_type",
            &self.get_hs_type().to_string(),
        )?;
        strukt.serialize_field(
            "representation_skew",
            &self.get_representation_skew().to_string(),
        )?;
        strukt.serialize_field(
            "target_resource",
            &self.get_target_resource().to_string(),
        )?;
        strukt.serialize_field("metadata", &self.get_metadata())?;
        strukt.serialize_field("spec", &self.get_spec())?;
        strukt.serialize_field("status", &self.get_status())?;
        strukt.serialize_field("created_at", &self.get_created_at())?;

        strukt.end()
    }
}

impl Serialize for Spec {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("spec", 3)?;
        strukt.serialize_field(
            "scale_target_ref",
            &self.get_scale_target_ref().to_string(),
        )?;
        strukt.serialize_field(
            "min_replicas",
            &self.get_min_replicas().to_string(),
        )?;
        strukt.serialize_field(
            "max_replicas",
            &self.get_max_replicas().to_string(),
        )?;
        strukt.end()
    }
}
