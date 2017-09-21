// Copyright (c) 2017 RioCorp Inc.

//The protocol for the database marshall/unmarshall
//for session management.


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

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::result;
use std::fmt;
use error::{Error, Result};
use std::str::FromStr;
use {asmsrv, servicesrv};

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct SessionCreate {
    id: String,
    name: String,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    provider: ::std::option::Option<OAuthProvider>,
    api_key: String,
    token: String,
    password: String,
    states: String,
    approval: String,
    suspend: String,
    registration_ip_address: String,
    created_at: String,
}

impl SessionCreate {
    pub fn new() -> SessionCreate {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }


    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.clone().unwrap_or(OAuthProvider::PasswordAuth)
    }

    pub fn set_first_name(&mut self, v: ::std::string::String) {
        self.first_name = v;
    }

    pub fn get_first_name(&self) -> ::std::string::String {
        self.first_name.clone()
    }

    pub fn set_last_name(&mut self, v: ::std::string::String) {
        self.last_name = v;
    }

    pub fn get_last_name(&self) -> ::std::string::String {
        self.last_name.clone()
    }

    pub fn set_phone(&mut self, v: ::std::string::String) {
        self.phone = v;
    }

    pub fn get_phone(&self) -> ::std::string::String {
        self.phone.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_states(&mut self, v: ::std::string::String) {
        self.states = v;
    }

    pub fn get_states(&self) -> ::std::string::String {
        self.states.clone()
    }

    pub fn set_approval(&mut self, v: ::std::string::String) {
        self.approval = v;
    }

    pub fn get_approval(&self) -> ::std::string::String {
        self.approval.clone()
    }

    pub fn set_suspend(&mut self, v: ::std::string::String) {
        self.suspend = v;
    }

    pub fn get_suspend(&self) -> ::std::string::String {
        self.suspend.clone()
    }

    pub fn set_registration_ip_address(&mut self, v: ::std::string::String) {
        self.registration_ip_address = v;
    }

    pub fn get_registration_ip_address(&self) -> ::std::string::String {
        self.registration_ip_address.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl Into<Session> for SessionCreate {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_name(self.get_name().to_owned());
        session.set_token(self.get_token().to_owned());
        session.set_apikey(self.get_apikey().to_owned());
        session
    }
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    id: String,
    email: String,
    name: String,
    token: String,
    api_key: String,
    flags: u32,
}

impl Session {
    pub fn new() -> Session {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }


    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_flags(&mut self, v: u32) {
        self.flags = v;
    }
    pub fn get_flags(&self) -> u32 {
        self.flags
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct SessionGet {
    name: String,
    email: String,
    token: String,
    api_key: String,
}


impl SessionGet {
    pub fn new() -> SessionGet {
        ::std::default::Default::default()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountGet {
    name: String,
    email: String,
    password: String,
}

impl AccountGet {
    pub fn new() -> AccountGet {
        ::std::default::Default::default()
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountGetId {
    id: String,
}


impl AccountGetId {
    pub fn new() -> AccountGetId {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum OAuthProvider {
    PasswordAuth = 0,
    LDAP = 1,
    OpenID = 2,
}

impl Serialize for OAuthProvider {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.to_owned() as u64)
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Account {
    id: String,
    name: String,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    token: String,
    api_key: String,
    password: String,
    states: String,
    approval: String,
    suspend: String,
    registration_ip_address: String,
    created_at: String,
}

impl Into<Session> for Account {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_name(self.get_name().to_owned());
        session.set_apikey(self.get_apikey().to_owned());
        session
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountGetResponse {
    results: Vec<Account>,
}


impl AccountGetResponse {
    pub fn new() -> AccountGetResponse {
        ::std::default::Default::default()
    }

    pub fn set_assemblys(&mut self, v: Vec<Account>) {
        self.results = v;
    }
}


impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_first_name(&mut self, v: ::std::string::String) {
        self.first_name = v;
    }

    pub fn get_first_name(&self) -> ::std::string::String {
        self.first_name.clone()
    }

    pub fn set_last_name(&mut self, v: ::std::string::String) {
        self.last_name = v;
    }

    pub fn get_last_name(&self) -> ::std::string::String {
        self.last_name.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_states(&mut self, v: ::std::string::String) {
        self.states = v;
    }

    pub fn get_states(&self) -> ::std::string::String {
        self.states.clone()
    }

    pub fn set_approval(&mut self, v: ::std::string::String) {
        self.approval = v;
    }

    pub fn get_approval(&self) -> ::std::string::String {
        self.approval.clone()
    }

    pub fn set_suspend(&mut self, v: ::std::string::String) {
        self.suspend = v;
    }

    pub fn get_suspend(&self) -> ::std::string::String {
        self.suspend.clone()
    }

    pub fn set_registration_ip_address(&mut self, v: ::std::string::String) {
        self.registration_ip_address = v;
    }

    pub fn get_registration_ip_address(&self) -> ::std::string::String {
        self.registration_ip_address.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Origin {
    id: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    created_at: String,
}
impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: servicesrv::ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &servicesrv::ObjectMetaData {
        &self.object_meta
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
