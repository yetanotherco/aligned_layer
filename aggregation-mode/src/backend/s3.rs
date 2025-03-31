use aligned_sdk::core::types::VerificationData;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::Client;
use tracing::info;

pub struct S3Client {
    client: Client,
    bucket_name: String,
}

pub enum GetBatchProofsError {
    Fetching,
    Deserialization,
    EmptyBody,
}

impl S3Client {
    pub async fn new(bucket_name: String, endpoint_url: Option<String>) -> Self {
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

        let client = Client::from_conf(s3_config_builder.build());

        Self {
            client,
            bucket_name,
        }
    }

    pub async fn get_aligned_batch(
        &self,
        key: String,
    ) -> Result<Vec<VerificationData>, GetBatchProofsError> {
        let result = self
            .client
            .get_object()
            .bucket(self.bucket_name.clone())
            .key(key)
            .send()
            .await
            .map_err(|_| GetBatchProofsError::Fetching)?;

        let Some(bytes) = result.body.bytes() else {
            return Err(GetBatchProofsError::EmptyBody);
        };

        let data: Vec<VerificationData> =
            ciborium::from_reader(bytes).map_err(|_| GetBatchProofsError::Deserialization)?;

        Ok(data)
    }
}
