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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Roles {
    id: String,
    name: String,
    description: String,
    created_at: String,
}

impl Roles {
    pub fn new() -> Roles {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct RolesGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Roles>,
}


impl RolesGetResponse {
    pub fn new() -> RolesGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: Vec<Roles>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Permissions {
    id: String,
    role_id: String,
    name: String,
    description: String,
    created_at: String,
}


impl Permissions {
    pub fn new() -> Permissions {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_role_id(&mut self, v: ::std::string::String) {
        self.role_id = v;
    }

    pub fn get_role_id(&self) -> ::std::string::String {
        self.role_id.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PermissionsGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Permissions>,
}


impl PermissionsGetResponse {
    pub fn new() -> PermissionsGetResponse {
        ::std::default::Default::default()
    }

    pub fn set_permissions(&mut self, v: Vec<Permissions>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}
