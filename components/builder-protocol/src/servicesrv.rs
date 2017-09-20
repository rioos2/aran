#![allow(unknown_lints)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::result;
use std::str::FromStr;
use asmsrv;
use std::collections::BTreeMap;


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    id: String,
    object_meta: asmsrv::ObjectMeta,
    type_meta: asmsrv::TypeMeta,
    data: BTreeMap<String, String>,
    created_at: String,
}
impl Secret {
    pub fn new() -> Secret {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_data(&mut self, v: BTreeMap<String, String>) {
        self.data = v;
    }

    pub fn get_data(&self) -> &BTreeMap<String, String> {
        &self.data
    }
    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: asmsrv::ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &asmsrv::ObjectMeta {
        &self.object_meta
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SecretGet {
    id: String,
}

impl SecretGet {
    pub fn new() -> SecretGet {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
}
