mod client;
mod types;

use async_trait::async_trait;
use tempfile::NamedTempFile;

use crate::{
    config::ConfigNextcloud,
    service::{Service, nextcloud::client::NextcloudClient},
};

/// A backup service that exports all files from a Nextcloud instance.
pub struct NextcloudService {
    client: NextcloudClient,
    username: String,
    password: String,
}

impl NextcloudService {
    /// Creates a new `NextcloudService`.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying Nextcloud client cannot be initialized.
    pub fn new(config: ConfigNextcloud) -> anyhow::Result<Self> {
        Ok(Self {
            client: NextcloudClient::new(config.url)?,
            username: config.username,
            password: config.password,
        })
    }
}

#[async_trait]
impl Service for NextcloudService {
    fn name(&self) -> &str {
        "nextcloud"
    }

    fn file_extension(&self) -> &str {
        "tar.gz"
    }

    async fn export(&self) -> anyhow::Result<NamedTempFile> {
        println!("Exporting Nextcloud files...");

        self.client.export_all(&self.username, &self.password).await
    }
}
