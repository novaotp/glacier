use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    Client,
    config::{Builder, Credentials},
    operation::put_object::PutObjectOutput,
    primitives::ByteStream,
};
use bytes::Bytes;

use crate::config::ConfigS3;

/// A wrapper client for interacting with S3-compatible storage.
pub struct S3(Client);

impl S3 {
    /// Initializes a new S3 client.
    pub async fn new(config: ConfigS3) -> Self {
        let credentials =
            Credentials::new(&config.access_key, &config.secret_key, None, None, "static");
        let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

        let mut builder = Builder::from(&sdk_config)
            .region(Region::new(config.region))
            .credentials_provider(credentials)
            .endpoint_url(config.endpoint);

        #[cfg(debug_assertions)]
        {
            builder = builder.force_path_style(true);
        }

        Self(Client::from_conf(builder.build()))
    }

    /// Uploads a payload to a specific bucket and key.
    pub async fn put(
        &self,
        bucket: impl Into<String>,
        key: impl Into<String>,
        body: Bytes,
    ) -> anyhow::Result<PutObjectOutput> {
        let command = self
            .0
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(body));

        Ok(command.send().await?)
    }
}
