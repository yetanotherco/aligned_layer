use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use log::info;

pub async fn create_client(endpoint_url: Option<String>) -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let mut config = aws_config::defaults(BehaviorVersion::latest()).region(region_provider);
    if let Some(endpoint_url) = &endpoint_url {
        info!("Using custom endpoint: {}", endpoint_url);
        config = config.endpoint_url(endpoint_url);
    }
    let config = config.load().await;

    let mut s3_config_builder = aws_sdk_s3::config::Builder::from(&config);
    if endpoint_url.is_some() {
        info!("Forcing path style for custom endpoint");
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
