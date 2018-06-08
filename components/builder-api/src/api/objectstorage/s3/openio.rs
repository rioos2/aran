use super::StorageClient;
use super::{BucketOutput, BucketOutputList};
use api::objectstorage::config::ObjectStorageConn;
use error::Result;
use openio_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};
use openio_sdk_rust::aws::common::region::Region;
use openio_sdk_rust::aws::s3::acl::*;
use openio_sdk_rust::aws::s3::bucket::*;
use openio_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use openio_sdk_rust::aws::s3::object::*;
use openio_sdk_rust::aws::s3::s3client::S3Client;
use protocol::api::objectstorage::Bucket;
use url::Url;

pub struct ObjectStorage {
    provider: DefaultCredentialsProvider,
    endpoint: Endpoint,
    // client: S3Client
}

impl ObjectStorage {
    pub fn new(conn: &ObjectStorageConn) -> Self {
        let param_provider: Option<ParametersProvider>;
        param_provider = Some(
            ParametersProvider::with_parameters(
                conn.access_key.clone(),
                conn.secret_key.clone(),
                None,
            ).unwrap(),
        );

        let provider = DefaultCredentialsProvider::new(param_provider).unwrap();

        // V4 is the default signature for AWS. However, other systems also use V2.
        let url = Url::parse(&conn.endpoint).unwrap();
        let endpoint = Endpoint::new(
            Region::UsEast1,
            Signature::V2,
            Some(url),
            None,
            None,
            Some(false),
        );

        ObjectStorage {
            provider: provider,
            endpoint: endpoint,
        }
    }
}

impl StorageClient for ObjectStorage {
    fn onboard(&self) -> Result<()> {
        Ok(())
    }

    fn create_bucket(&self, bucket: &Bucket) -> BucketOutput {
        //let client = S3Client::new(provider, endpoint);
        Ok(None)
    }

    fn list_bucket(&self) -> BucketOutputList {
        Ok(None)
    }

    fn upload(&self, bucket: &Bucket) -> BucketOutput {
        Ok(None)
    }

    fn download(&self) -> BucketOutput {
        Ok(None)
    }
}
