mod client;

use async_trait::async_trait;
use glacier_crypto::encrypt::encrypt_file;
use tempfile::NamedTempFile;
use tokio::task;

use crate::{
    config::ConfigBitwarden,
    service::{
        ExportItem, Service,
        bitwarden::client::{BitwardenClient, BitwardenExportFormat},
    },
};

/// A backup service that exports all passwords from a Bitwarden account.
pub struct BitwardenService {
    config: ConfigBitwarden,
}

impl BitwardenService {
    /// Creates a new `BitwardenService`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use glacier_core::{
    ///     config::{BitwardenAuth, ConfigBitwarden},
    ///     service::bitwarden::BitwardenService
    /// };
    ///
    /// let config = ConfigBitwarden {
    ///     auth: BitwardenAuth {
    ///         client_id: String::from("YOUR_CLIENT_ID"),
    ///         client_secret: String::from("YOUR_CLIENT_SECRET"),
    ///     },
    ///     master_password: String::from("YOUR_MASTER_PASSWORD"),
    ///     format: String::from("csv"),
    ///     encrypt_password: None,
    /// };
    ///
    /// let bitwarden = BitwardenService::new(config);
    /// ```
    pub fn new(config: ConfigBitwarden) -> Self {
        Self { config }
    }
}

impl BitwardenService {
    /// Attempts to export the vault data using the given session.
    ///
    /// # Errors
    ///
    /// Returns an error if :
    /// - an unsupported/invalid format is given.
    /// - there was an error with the tempfile.
    /// - the export failed.
    async fn try_export(
        &self,
        client: &BitwardenClient,
        session: &str,
    ) -> anyhow::Result<NamedTempFile> {
        let format = match self.config.format.as_str() {
            "csv" => BitwardenExportFormat::Csv,
            "json" => BitwardenExportFormat::Json,
            "encrypted_json" => anyhow::bail!(
                "encrypted_json is not supported. Encrypt your file manually after using another export format."
            ),
            "zip" => BitwardenExportFormat::Zip,
            _ => anyhow::bail!("Unsupported Bitwarden export format."),
        };

        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file
            .path()
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid path format"))?;

        client.export(temp_path, format, session).await?;

        Ok(temp_file)
    }

    /// Encrypts the file if the encryption password is set, otherwise returns as-is.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while encrypting the file.
    async fn maybe_encrypt(
        &self,
        temp_file: NamedTempFile,
    ) -> anyhow::Result<(NamedTempFile, String)> {
        let Some(password) = self.config.encrypt_password.clone() else {
            return Ok((temp_file, self.config.format.clone()));
        };

        let encrypted_temp_file = task::spawn_blocking(move || -> anyhow::Result<NamedTempFile> {
            let encrypted_temp_file = NamedTempFile::new()?;

            println!("Encrypting file...");
            encrypt_file(temp_file.path(), encrypted_temp_file.path(), &password)?;

            Ok(encrypted_temp_file)
        })
        .await??;

        Ok((
            encrypted_temp_file,
            format!("{}.glacier", self.config.format),
        ))
    }
}

#[async_trait]
impl Service for BitwardenService {
    fn name(&self) -> &str {
        "bitwarden"
    }

    async fn export(&self) -> anyhow::Result<Vec<ExportItem>> {
        let client = BitwardenClient::new();

        // Ensure we are not logged in, but we don't care if we are not
        let _ = client.logout().await;

        let result = async {
            println!("Logging into account...");
            client.login(&self.config.auth).await?;

            println!("Unlocking vault...");
            let session = client.unlock(&self.config.master_password).await?;

            println!("Exporting Bitwarden data...");
            self.try_export(&client, &session).await
        }
        .await;
        if result.is_err() {
            println!("An error occurred. Aborting...")
        }

        println!("Logging out of account...");
        let _ = client.logout().await;

        let (temp_file, format) = self.maybe_encrypt(result?).await?;

        Ok(vec![ExportItem::new("passwords", format, temp_file)])
    }
}
