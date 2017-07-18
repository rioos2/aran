// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;


#[derive(PartialEq,Clone,Default)]
pub struct Assembly {
    // message fields
    id: ::std::option::Option<u64>,
    uri: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularPtrField<super::originsrv::OriginProject>,
    tags: ::protobuf::SingularPtrField<super::net::NetError>,
    representation_skew: ::protobuf::SingularField<::std::string::String>,
    external_management_resource: ::protobuf::SingularField<::std::string::String>,
    component_collection: ::protobuf::SingularField<::std::string::String>,
    plan: ::protobuf::SingularPtrField<super::originsrv::OriginPackageIdent>,
    operation_collection: ::std::option::Option<bool>,
    metadata: ::protobuf::SingularField<::std::string::String>,
    created_at: ::protobuf::SingularField<::std::string::String>,
    updated_at: ::protobuf::SingularField<::std::string::String>
}

// // see codegen.rs for the explanation why impl Sync explicitly
// unsafe impl ::std::marker::Sync for Assembly {}
impl Assembly {
    pub fn new() -> Assembly {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AssemblyGet {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
// unsafe impl ::std::marker::Sync for AssemblyGet {}
impl AssemblyGet {
    pub fn new() -> AssemblyGet {
        ::std::default::Default::default()
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

}
