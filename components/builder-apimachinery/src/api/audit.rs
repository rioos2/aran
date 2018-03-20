// Copyright 2018 The Rio Advancement Inc

use std::fmt;

use error::{Error, Result};
use serde_json;
use chrono::prelude::*;

use api::base::{TypeMeta, ObjectMeta, MetaFields};

/// The accessor information for the audit.
/// The 0th value of the tuple is the account_id
/// The 1st value of the tuple is the ip address the request came from
pub type AccessedBy = (String, String);

/// Rio/OS  blockchain global audit envelop
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub account: String,
    pub address: String,
    pub timestamp: String,
    pub event: AuditEvent,
}

impl Envelope {
    pub fn new(event: &AuditEvent, accessed_by: AccessedBy) -> Self {
        Envelope {
            account: accessed_by.0,
            address: accessed_by.1,
            timestamp: Utc::now().to_rfc3339(),
            event: event.clone(),
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSource {
    // Component from which the event is generated.
    // +optional
    component: String,
    // Node name on which the event is generated.
    // +optional
    host: String,
}

// Event is a report of an event somewhere in the cluster.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    reason: String,

    // A human-readable description of the status of this operation.
    // TODO: decide on maximum length.
    // +optional
    message: String,

    // The component reporting this event. Should be a short machine understandable string.
    // +optional
    source: EventSource,

    // Type of this event (Normal, Warning), new types could be added in the future
    // +optional
    type_of_event: String,
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

// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReference {
    // Kind of the referent.
    // +optional
    kind: String,
    // Origin of the referent.
    // +optional
    origin: String,
    // Name of the referent.
    // +optional
    name: String,
    // UID of the referent.
    uid: String,
    // API version of the referent.
    api_version: String,
    // Specific resourceVersion to which this reference is made, if any.
    resource_version: String,

    // If referring to a piece of an object instead of an entire object, this string
    // should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    // For example, if the object reference is to a container within a assemblyfactory, this would take on a value like:
    // "spec.containers{name}" (where "name" refers to the name of the container that triggered
    // the event) or if no container name is specified "spec.containers[2]" (container with
    // index 2 in this assemblyfactory). This syntax is chosen only to have some well-defined way of
    // referencing a part of an object.
    field_path: String,
}
