mod client;
mod types;

use async_trait::async_trait;

use crate::{
    config::ConfigNextcloud,
    service::{ExportItem, Service, nextcloud::client::NextcloudClient},
};

/// A backup service that exports all files from a Nextcloud instance.
pub struct NextcloudService {
    /// The underlying Nextcloud client.
    client: NextcloudClient,
    /// The username of the Nextcloud account.
    username: String,
    /// The app password of the Nextcloud account for this service.
    password: String,
    /// The password to use for encryption, if any.
    encrypt_password: Option<String>,
}

impl NextcloudService {
    /// Creates a new `NextcloudService`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use glacier_core::{config::ConfigNextcloud, service::nextcloud::NextcloudService};
    ///
    /// let config = ConfigNextcloud {
    ///     url: String::from("https://nextcloud.example.com"),
    ///     username: String::from("johndoe"),
    ///     password: String::from("YOUR_APP_PASSWORD"),
    ///     encrypt_password: None,
    /// };
    ///
    /// let nextcloud = NextcloudService::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying Nextcloud client cannot be initialized.
    pub fn new(config: ConfigNextcloud) -> anyhow::Result<Self> {
        Ok(Self {
            client: NextcloudClient::new(config.url)?,
            username: config.username,
            password: config.password,
            encrypt_password: config.encrypt_password,
        })
    }
}

#[async_trait]
impl Service for NextcloudService {
    fn name(&self) -> &str {
        "nextcloud"
    }

    async fn export(&self) -> anyhow::Result<Vec<ExportItem>> {
        println!("Exporting Nextcloud files...");

        let temp_file = self
            .client
            .export_all(&self.username, &self.password)
            .await?;

        let (temp_file, format) = self
            .maybe_encrypt(temp_file, "tar.gz", self.encrypt_password.as_deref())
            .await?;

        Ok(vec![ExportItem::new("files", format, temp_file)])
    }
}
