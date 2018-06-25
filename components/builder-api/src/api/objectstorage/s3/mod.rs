// Copyright 2018 The Rio Advancement Inc
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod openio;

use error::Result;
use protocol::api::objectstorage::Bucket;
use protocol::api::objectstorage::BucketAccessor;
use api::objectstorage::config::{ObjectStorageBackend, ObjectStorageCfg};

/// Currently implemented securer backends

pub trait StorageClient: Send {
    // /// authenticate storage party
    fn onboard(&self) -> Result<()>;

    /// create bucket
    fn create_bucket(&self, bucket: &Bucket) -> BucketOutput;

    /// list bucket
    fn list_bucket(&self) -> BucketOutputList;

    /// Return the upload accessor signed URL.
    /// The URL returned expires after 1 minute
    fn upload_accessor(&self, bucket: String, file_name: String) -> BucketAccessorOutput;

    /// Return the download accessor signed URL
    /// The URL returned expires after 1 minute
    fn download_accessor(&self, bucket: String, file_name: String) -> BucketAccessorOutput;
}

/// Create appropriate Securer variant based on configuration values.
pub fn from_config(config: &ObjectStorageCfg) -> Result<Box<StorageClient>> {
    match config.backend {
        ObjectStorageBackend::OpenIO => Ok(Box::new(openio::Storage::new(config))),
    }
}

/// BucketOutput output loaded from the database
pub type BucketOutput = Result<Option<Bucket>>;

/// BucketAccessorOutput output loaded from the database
pub type BucketAccessorOutput = Result<Option<BucketAccessor>>;

/// BucketOutput output list loaded from the database
pub type BucketOutputList = Result<Option<Vec<Bucket>>>;
