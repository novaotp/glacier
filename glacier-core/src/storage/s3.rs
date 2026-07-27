use std::path::Path;

use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    Client,
    config::{Builder, Credentials},
    primitives::ByteStream,
};

use crate::{
    config::ConfigS3,
    storage::{ArchiveDescriptor, Storage},
};

/// A wrapper client for interacting with S3-compatible storage.
#[derive(Debug, Clone)]
pub struct S3Storage {
    /// The underlying S3 client.
    client: Client,
    /// The bucket to operate on.
    bucket: String,
}

impl S3Storage {
    /// Initializes a new S3 client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glacier_core::{config::ConfigS3, storage::s3::S3Storage};
    ///
    /// # #[tokio::test]
    /// # async fn try_main() {
    /// let config = ConfigS3 {
    ///     bucket: String::from("default-bucket"),
    ///     region: String::from("us-east-1"),
    ///     endpoint: String::from("https://s3.example.com"),
    ///     access_key: String::from("YOUR_ACCESS_KEY"),
    ///     secret_key: String::from("YOUR_SECRET_KEY"),
    /// };
    ///
    /// let s3 = S3Storage::new(config).await;
    /// # Ok(())
    /// # }
    /// ```
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
            // Local S3 container needs it
            builder = builder.force_path_style(true);
        }

        Self {
            client: Client::from_conf(builder.build()),
            bucket: config.bucket,
        }
    }
}

#[async_trait]
impl Storage for S3Storage {
    fn name(&self) -> &str {
        "s3"
    }

    async fn upload(
        &self,
        archive_descriptor: &ArchiveDescriptor,
        data_path: &Path,
    ) -> anyhow::Result<()> {
        let key = format!(
            "data/{}/backups/automatic/{}",
            archive_descriptor.service_name,
            archive_descriptor.to_filename()
        );

        let _ = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from_path(data_path).await?)
            .send()
            .await?;

        Ok(())
    }
}
