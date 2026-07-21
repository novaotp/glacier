use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    Client,
    config::{Builder, Credentials},
    primitives::ByteStream,
};
use bytes::Bytes;

use crate::{
    config::ConfigS3,
    storage::{ArchiveDescriptor, Storage},
};

/// A wrapper client for interacting with S3-compatible storage.
#[derive(Debug, Clone)]
pub struct S3Storage {
    client: Client,
    bucket: String,
}

impl S3Storage {
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
        archive_descriptor: ArchiveDescriptor,
        data: Bytes,
    ) -> anyhow::Result<()> {
        let key = format!(
            "data/{name}/backups/automatic/{date}_{name}_backup.zip",
            name = archive_descriptor.name,
            date = archive_descriptor.date
        );

        let _ = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data))
            .send()
            .await?;

        Ok(())
    }
}
