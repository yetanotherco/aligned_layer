use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

// https://docs.aws.amazon.com/sdk-for-rust/latest/dg/localstack.html
const LOCALSTACK_ENDPOINT: &str = "http://127.0.0.1:4566/";

pub async fn create_client(environment: String) -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let mut config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider);
    if environment == "local" {
        config = config.endpoint_url(LOCALSTACK_ENDPOINT);
    }
    let config = config
        .load()
        .await;

    let mut s3_config_builder = aws_sdk_s3::config::Builder::from(&config);
    if environment == "local" {
        s3_config_builder = s3_config_builder.force_path_style(true);
    }
    Client::from_conf(s3_config_builder.build())
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
