use super::StorageClient;
use super::{BucketOutput, BucketOutputList};
use api::objectstorage::config::ObjectStorageCfg;
use error::{Error, Result};
use openio_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};
use openio_sdk_rust::aws::common::region::Region;
use openio_sdk_rust::aws::s3::bucket::*;
use openio_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use openio_sdk_rust::aws::s3::object::*;
use openio_sdk_rust::aws::s3::s3client::S3Client;
use protocol::api::base::MetaFields;
use protocol::api::objectstorage::Bucket;
use url::Url;

pub struct Storage {
    parameters: Option<ParametersProvider>,
    endpoint: Endpoint,
}

impl Storage {
    pub fn new(conn: &ObjectStorageCfg) -> Self {
        let params: Option<ParametersProvider> = Some(
            ParametersProvider::with_parameters(
                conn.access_key.clone(),
                conn.secret_key.clone(),
                None,
            ).unwrap(),
        );

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

        Storage {
            parameters: params,
            endpoint: endpoint,
        }
    }

    fn credentials(&self) -> Result<DefaultCredentialsProvider> {
        DefaultCredentialsProvider::new(self.parameters.clone())
            .map_err(Error::OpenIOCredentialsError)
    }
}

impl StorageClient for Storage {
    fn onboard(&self) -> Result<()> {
        Ok(())
    }

    fn create_bucket(&self, bucket: &Bucket) -> BucketOutput {
        let creds = self.credentials()?;
        println!("+++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:?}", self.endpoint.clone());
        let client: S3Client<DefaultCredentialsProvider, _> =
            S3Client::new(creds, self.endpoint.clone());

        let bucket_name = format!("{}_{} ", bucket.get_account(),bucket.get_name());
        let mut bucket_req = CreateBucketRequest::default();
        bucket_req.bucket = bucket_name.clone();

        info!("☛ Create bucket {} ", bucket_name);

        match client.create_bucket(&bucket_req) {
            Ok(_) => Ok(Some(bucket.clone())),
            Err(e) => Err(Error::OpenIOS3Error(e)),
        }
    }

    fn list_bucket(&self) -> BucketOutputList {
        let creds = self.credentials()?;

        let client: S3Client<DefaultCredentialsProvider, _> =
            S3Client::new(creds, self.endpoint.clone());

        info!("☛ List buckets ");
        match client.list_buckets() {
            //TO-DO: Do a .map(.. ) on the result to create a Bucket.
            // Make sure the owner of the bucket is included
            Ok(_) => Ok(None),
            Err(e) => Err(Error::OpenIOS3Error(e)),
        }
    }

    fn upload_accessor(&self, bucket: &Bucket) -> BucketOutput {
        let creds = self.credentials()?;

        let client: S3Client<DefaultCredentialsProvider, _> =
            S3Client::new(creds, self.endpoint.clone());

        let bucket_name = bucket.get_name();
        let mut put_req = GetObjectRequest::default();
        put_req.bucket = bucket_name.clone();

        //TO-DO: Replace with a filename key from Bucket
        put_req.key = bucket_name.clone();

        info!("☛ Upload bucket accessor {} ", bucket_name);

        let _url = client.put_object_url(&put_req, None);

        //TO-DO: Stick the url received into the  below bucket
        //using set_ methods.
        let ba = Bucket::new();
        Ok(Some(ba))
    }

    fn download_accessor(&self, bucket: &Bucket) -> BucketOutput {
        let creds = self.credentials()?;

        let client: S3Client<DefaultCredentialsProvider, _> =
            S3Client::new(creds, self.endpoint.clone());

        let bucket_name = bucket.get_name();
        let mut get_req = GetObjectRequest::default();
        get_req.bucket = bucket_name.clone();

        //TO-DO: Replace with a filename key from Bucket
        get_req.key = bucket_name.clone();

        info!("☛ Download bucket accessor {} ", bucket_name);

        let _url = client.get_object_url(&get_req, None);

        //TO-DO: Stick the url received into the  below bucket
        //using set_ methods.
        let ba = Bucket::new();
        Ok(Some(ba))
    }
}
