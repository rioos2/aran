// Copyright 2018 The Rio Advancement Inc
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod openio;


use error::Result;
use protocol::api::objectstorage::Bucket;

use api::objectstorage::config::{ObjectStorageConn, ObjectStorageBackend};


/// Currently implemented securer backends

pub trait StorageClient: Send {

    // /// authenticate storage party
     fn onboard(&self) -> Result<()>;

    /// create bucket
    fn create_bucket(&self, bucket: &Bucket) -> BucketOutput;

    /// list bucket
    fn list_bucket(&self) -> BucketOutputList;

    /// File upload
    fn upload(&self, bucket: &Bucket) -> BucketOutput;

    ///File Download
    fn download(&self) -> BucketOutput;

}

/// Create appropriate Securer variant based on configuration values.
pub fn from_config(config: &ObjectStorageConn) -> Result<Box<StorageClient>> {
    match config.backend {
        ObjectStorageBackend::OpenIO => Ok(Box::new(openio::ObjectStorage::new(config))),

    }
}

/// BucketOutput output loaded from the database
pub type BucketOutput = Result<Option<Bucket>>;

/// BucketOutput output list loaded from the database
pub type BucketOutputList = Result<Option<Vec<Bucket>>>;
