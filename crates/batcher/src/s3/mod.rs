use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use log::info;

pub struct S3Config {
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub region: Option<String>,
    pub endpoint_url: Option<String>,
}

pub async fn create_client(s3_config: S3Config) -> Client {
    let mut config = aws_config::defaults(BehaviorVersion::latest());

    if let Some(region) = s3_config.region {
        let region_provider =
            RegionProviderChain::first_try(Region::new(region)).or_else("us-east-2");
        config = config.region(region_provider);
    } else {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
        config = config.region(region_provider);
    }

    if let (Some(access_key_id), Some(secret_access_key)) =
        (s3_config.access_key_id, s3_config.secret_access_key)
    {
        let credentials = aws_sdk_s3::config::Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "custom",
        );
        config = config.credentials_provider(credentials);
    }

    if let Some(endpoint_url) = &s3_config.endpoint_url {
        info!("Using custom endpoint: {}", endpoint_url);
        config = config.endpoint_url(endpoint_url);
    }

    let config = config.load().await;

    let mut s3_config_builder = aws_sdk_s3::config::Builder::from(&config);
    if s3_config.endpoint_url.is_some() {
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
