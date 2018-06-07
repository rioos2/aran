use error::Result;
use api::objectstorage::config::ObjectStorageConn;
use protocol::api::objectstorage::Bucket;


use super::StorageClient;
use super::{BucketOutput, BucketOutputList};

use openio_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider, AwsCredentialsProvider};
use openio_sdk_rust::aws::common::request::{DispatchSignedRequest};
use openio_sdk_rust::aws::s3::bucket::*;
use openio_sdk_rust::aws::s3::object::*;
use openio_sdk_rust::aws::s3::acl::*;

use openio_sdk_rust::aws::common::region::Region;
use openio_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use openio_sdk_rust::aws::s3::s3client::S3Client;
use url::Url;



pub struct ObjectStorage {
    client: S3Client
}

impl ObjectStorage {
    pub fn new(conn: &ObjectStorageConn) -> Self {

        let param_provider: Option<ParametersProvider>;
            param_provider = Some(
                 ParametersProvider::with_parameters(conn.access_key,conn.secret_key,None).unwrap()
);

let provider = DefaultCredentialsProvider::new(param_provider).unwrap();

// V4 is the default signature for AWS. However, other systems also use V2.
//let endpoint = Endpoint::new(Region::UsEast1, Signature::V2, None, None, None, None);
let url = Url::parse(conn.endpoint).unwrap();
let endpoint = Endpoint::new(Region::UsEast1, Signature::V2, Some(url), None, None, Some(false));
let client = S3Client::new(provider, endpoint);
        ObjectStorage{client:client};
    }
}

impl StorageClient for ObjectStorage {

    fn onboard(&self) -> Result<()> {
        Ok(())
    }

    fn create_bucket(&self, bucket: &Bucket) -> BucketOutput {
        Ok(())
    }

    fn list_bucket(&self) -> BucketOutputList {
        Ok(())
    }

    fn upload(&self, bucket: &Bucket) -> BucketOutput {
        Ok(())
    }

    fn download(&self) -> BucketOutput {
        Ok(())
    }
}
