use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use bytes::Bytes;
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;

pub async fn create_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load().await;
    Client::new(&config)
}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    bytes: Vec<u8>,
    key: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from(bytes);

    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .send()
        .await
}

