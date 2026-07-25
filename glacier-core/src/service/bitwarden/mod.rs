mod client;

use async_trait::async_trait;
use tempfile::NamedTempFile;

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
    pub fn new(config: ConfigBitwarden) -> Self {
        Self { config }
    }
}

impl BitwardenService {
    /// Attempts to export the vault data using the given session.
    pub async fn try_export(
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

        let temp_file = result?;

        Ok(vec![ExportItem::new(
            "passwords",
            self.config.format.clone(),
            temp_file,
        )])
    }
}
