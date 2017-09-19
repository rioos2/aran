#![allow(unknown_lints)]
#![allow(clippy)]

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

use std::error;
use std::fmt;
use std::result;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(PartialEq, Clone, Default)]
pub struct NetOk {}

impl NetOk {
    pub fn new() -> NetOk {
        ::std::default::Default::default()
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ErrCode {
    BUG = 0,
    TIMEOUT = 1,
    REMOTE_REJECTED = 2,
    BAD_REMOTE_REPLY = 3,
    ENTITY_NOT_FOUND = 4,
    NO_SHARD = 6,
    ACCESS_DENIED = 7,
    SESSION_EXPIRED = 8,
    ENTITY_CONFLICT = 9,
    DATA_STORE = 11,
    AUTH_SCOPE = 12,
    WORKSPACE_SETUP = 1000,
    SECRET_KEY_FETCH = 1001,
    SECRET_KEY_IMPORT = 1002,
    VCS_CLONE = 1003,
    BUILD = 1004,
    POST_PROCESSOR = 1005,
}


pub fn err<M: Into<String>>(code: ErrCode, msg: M) -> NetError {
    let mut err = NetError::new();
    err.set_code(code);
    err.set_msg(msg.into());
    err
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct NetError {
    code: ::std::option::Option<ErrCode>,
    msg: ::std::option::Option<String>,
}


impl NetError {
    pub fn new() -> NetError {
        ::std::default::Default::default()
    }

    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ErrCode) {
        self.code = ::std::option::Option::Some(v);
    }

    pub fn get_code(&self) -> ErrCode {
        self.code.clone().unwrap_or(ErrCode::BUG)
    }

    fn get_code_for_reflect(&self) -> &::std::option::Option<ErrCode> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::std::option::Option<ErrCode> {
        &mut self.code
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::std::option::Option::Some(v);
    }


    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(
            || ::std::string::String::new(),
        )
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[err: {:?}, msg: {}]", self.get_code(), self.get_msg())
    }
}

impl error::Error for NetError {
    fn description(&self) -> &str {
        match self.get_code() {
            ErrCode::BUG => "An unexpected error occurred.",
            ErrCode::TIMEOUT => "Network timeout.",
            ErrCode::REMOTE_REJECTED => "Remote server rejected request.",
            ErrCode::BAD_REMOTE_REPLY => "Remote server returned a bad response.",
            ErrCode::ENTITY_NOT_FOUND => "Entity not found in datastore.",
            ErrCode::NO_SHARD => "Shard not available.",
            ErrCode::ACCESS_DENIED => "Operation not allowed by authenticated.",
            ErrCode::SESSION_EXPIRED => "Session expired, user should re-authenticate.",
            ErrCode::ENTITY_CONFLICT => "Entity already exists in datastore.",
            ErrCode::DATA_STORE => "Database error.",
            ErrCode::AUTH_SCOPE => "Additional authorization scope(s) required for action.",
            ErrCode::WORKSPACE_SETUP => "Worker runner unable to setup build workspace.",
            ErrCode::SECRET_KEY_FETCH => "Worker runner unable to fetch secret key for origin.",
            ErrCode::SECRET_KEY_IMPORT => "Worker runner unable to import secret key for origin.",
            ErrCode::VCS_CLONE => "Worker runner unable to retrieve project source to build.",
            ErrCode::BUILD => "Worker runner failed to build project.",
            ErrCode::POST_PROCESSOR => "One or more post processing step failed in Worker runner.",
        }
    }
}

impl Serialize for ErrCode {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.to_owned() as u64)
    }
}

impl Serialize for NetError {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = try!(serializer.serialize_struct("error", 2));
        try!(strukt.serialize_field("code", &self.get_code()));
        try!(strukt.serialize_field("msg", self.get_msg()));
        strukt.end()
    }
}
