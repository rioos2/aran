use super::StorageClient;
use super::{BucketOutput, BucketOutputList, BucketAccessorOutput};
use api::objectstorage::config::ObjectStorageCfg;
use error::{Error, Result};
use openio_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};
use openio_sdk_rust::aws::common::region::Region;
use openio_sdk_rust::aws::s3::bucket::*;
use openio_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use openio_sdk_rust::aws::s3::object::*;
use openio_sdk_rust::aws::s3::s3client::S3Client;
use openio_sdk_rust::aws::s3::writeparse::ListBucketsOutput;
use protocol::api::base::MetaFields;
use protocol::api::objectstorage::{Bucket, BucketAccessor};
use protocol::api::schema::type_meta_url;
use protocol::api::base::WhoAmITypeMeta;
use url::Url;
use serde_json;
use serde_json::Value;

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

    fn parse_list_bucket(&self, bucket: ListBucketsOutput) -> BucketOutputList {
       let s: Vec<Bucket> = bucket.buckets
                    .into_iter()
                    .map(|x| {
                        let mut ba = Bucket::new();
                        let ref mut om = ba.mut_meta(ba.object_meta(), x.name, "".to_string());
                        ba.set_created_at(x.creation_date);
                        let whoami = ba.who_am_i();
                        ba.set_meta(type_meta_url(whoami), om.clone());
                        ba
                    })
                    .collect();
        Ok(Some(s))
    }

}

impl StorageClient for Storage {
    fn onboard(&self) -> Result<()> {
        Ok(())
    }

    fn create_bucket(&self, bucket: &Bucket) -> BucketOutput {
        let creds = self.credentials()?;       
        let client: S3Client<DefaultCredentialsProvider, _> =
            S3Client::new(creds, self.endpoint.clone());

        let bucket_name = format!("{}-{}", bucket.get_account(),bucket.get_name());
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
            Ok(bucket) => self.parse_list_bucket(bucket),
            Err(e) => Err(Error::OpenIOS3Error(e)),
        }
    }

    fn upload_accessor(&self, bucket: &Bucket, file_name: String) -> BucketAccessorOutput {
        let creds = self.credentials()?;

        let client: S3Client<DefaultCredentialsProvider, _> =
            S3Client::new(creds, self.endpoint.clone());

        let bucket_name = bucket.get_name();
        let mut put_req = GetObjectRequest::default();
        put_req.bucket = bucket_name.clone();

        //TO-DO: Replace with a filename key from Bucket
        put_req.key = file_name.clone();

        info!("☛ Upload bucket accessor {} ", bucket_name);

        let url = client.put_object_url(&put_req, None);

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(&url)?;

        //TO-DO: Stick the url received into the  below bucket
        //using set_ methods.
        let mut ba = BucketAccessor::new();
        let ad = ba.accessor_data();
        ad.set_url(v["url"].to_string());
        ad.set_date(v["date"].to_string());
        ad.set_authorization(v["authorization"].to_string());
        ad.set_content_type("application/octet-stream".to_string());        
        ba.set_accessor_data(ad);

        let ref mut om = ba.mut_meta(ba.object_meta(), bucket_name, "".to_string());                    
        let whoami = ba.who_am_i();
        ba.set_meta(type_meta_url(whoami), om.clone());        
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
