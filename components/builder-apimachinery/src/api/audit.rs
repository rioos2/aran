// Copyright 2018 The Rio Advancement Inc

use api::base::{MetaFields, ObjectMeta, ObjectReference, TypeMeta};
use chrono::prelude::*;
use error::{Error, Result};
use serde_json;
use std::fmt;

/// The accessor information for the audit.
/// The 0th value of the tuple is the account_id
/// The 1st value of the tuple is the ip address the request came from
pub type AccessedBy = (String, String);

/// Rio/OS  blockchain global audit envelope
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    account: String,
    address: String,
    timestamp: String,
    event: AuditEvent,
}

impl Envelope {
    /// Creates envelope for an AuditEvent.
    ///
    /// This method is necessary to make an envelope stamped with a timestamp,
    /// ip address of the sender, and the account associated with.   
    pub fn new(event: &AuditEvent, accessed_by: AccessedBy) -> Self {
        Envelope {
            account: accessed_by.0,
            address: accessed_by.1,
            timestamp: Utc::now().to_rfc3339(),
            event: event.clone(),
        }
    }

    pub fn set_account(&mut self, v: ::std::string::String) {
        self.account = v;
    }

    pub fn get_account(&self) -> ::std::string::String {
        self.account.clone()
    }

    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    pub fn get_address(&self) -> ::std::string::String {
        self.address.clone()
    }

    pub fn set_timestamp(&mut self, v: ::std::string::String) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> ::std::string::String {
        self.timestamp.clone()
    }

    pub fn set_event(&mut self, v: AuditEvent) {
        self.event = v;
    }

    pub fn get_event(&self) -> AuditEvent {
        self.event.clone()
    }

    /// Tries to serialize given configuration into the utf8 encoded json.
    pub fn try_serialize(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self).map_err(Error::Json)
    }

    /// Tries to deserialize `Envelop` from the given utf8 encoded json.
    pub fn try_deserialize(serialized: &[u8]) -> Result<Envelope> {
        serde_json::from_slice(serialized).map_err(Error::Json)
    }
}

impl fmt::Display for Envelope {
    // TODO fn: As of rustfmt 0.7.1 the following match block is not well understood. The tool puts
    // all match arms on the same line which blows over the 100-column max which then fails the
    // tool with a `"line exceeded maximum length"` error. This ignore should be removed when we
    // upgrade rustfmt and retry.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}:{}_{} =>{:?}", self.timestamp, self.account, self.address,  self.event)
    }
}

// EventSource contains information for an event.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventSource {
    // Component from which the event is generated.
    // +optional
    component: String,
    // Node name on which the event is generated.
    // +optional
    host: String,
}

// AuditEvent is a report of an event somewhere in the cluster.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditEvent {
    // Standard type metadata.
    type_meta: TypeMeta,

    // Standard object's metadata.
    object_meta: ObjectMeta,

    // ObjectReference contains enough information to let you inspect or modify the referred object.
    object_reference: ObjectReference,

    // This should be a short, machine understandable string that gives the reason
    // for the transition into the object's current status.
    // TODO: provide exact specification for format.
    // +optional
    pub reason: String,

    // A human-readable description of the status of this operation.
    // TODO: decide on maximum length.
    // +optional
    pub message: String,

    // The component reporting this event. Should be a short machine understandable string.
    // +optional
    source: EventSource,

    // Type of this event (Normal, Warning), new types could be added in the future
    // +optional
    type_of_event: String,
}

impl AuditEvent {
    pub fn new() -> AuditEvent {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> AuditEvent {
        AuditEvent {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }

    pub fn set_object_reference(&mut self, v: ObjectReference) {
        self.object_reference = v;
    }
    pub fn get_object_reference(&self) -> ObjectReference {
        self.object_reference.clone()
    }

    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }

    pub fn get_reason(&self) -> ::std::string::String {
        self.reason.clone()
    }

    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    pub fn get_message(&self) -> ::std::string::String {
        self.message.clone()
    }      
    
}

impl MetaFields for AuditEvent {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

/// The envelope response wrapped with typemeta and objectmeta
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvelopeResponse {
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    envelope: Envelope,
}

impl EnvelopeResponse {
    pub fn with(type_meta: TypeMeta, object_meta: ObjectMeta, envelope: Envelope) -> Self {
        EnvelopeResponse {
            type_meta: type_meta,
            object_meta: object_meta,
            envelope: envelope,
        }
    }
}
