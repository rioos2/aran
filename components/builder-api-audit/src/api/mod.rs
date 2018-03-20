// Copyright 2018 The Rio Advancement Inc
//

//! Infrastructure - Cluster part of the Rioos rest api.

pub mod audit;

use exonum::crypto::{hash, Hash};

use protocol::api::audit::Envelope;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponseEnvelopePost {
    pub tx_hash: Hash,
    pub cfg_hash: Hash,
}

pub trait StorageInternalValue {
    fn into_bytes(self) -> Vec<u8>;

    fn from_bytes(v: ::std::borrow::Cow<[u8]>) -> Self;

    fn hash(&self) -> Hash;
}

impl StorageInternalValue for Envelope {
    fn into_bytes(self) -> Vec<u8> {
        self.try_serialize().unwrap()
    }

    fn from_bytes(v: ::std::borrow::Cow<[u8]>) -> Self {
        Envelope::try_deserialize(v.as_ref()).unwrap()
    }

    fn hash(&self) -> Hash {
        let vec_bytes = self.try_serialize().unwrap();
        hash(&vec_bytes)
    }
}
