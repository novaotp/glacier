use std::path::PathBuf;

use async_trait::async_trait;
use glob::glob;
use tempfile::NamedTempFile;
use tokio::{fs, process::Command};

use crate::{
    config::ConfigEnte,
    service::{ExportItem, Service},
};

/// The Ente Auth service.
pub struct EnteService {
    export_path: String,
    encrypt_password: Option<String>,
}

impl EnteService {
    /// Creates a new `EnteService`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use glacier_core::{
    ///     config::ConfigEnte,
    ///     service::ente::EnteService
    /// };
    ///
    /// let config = ConfigEnte {
    ///     export_path: String::from("/home/nova/.ente"),
    ///     encrypt_password: None,
    /// };
    ///
    /// let ente = EnteService::new(config);
    /// ```
    pub fn new(config: ConfigEnte) -> Self {
        Self {
            export_path: config.export_path,
            encrypt_password: config.encrypt_password,
        }
    }

    /// Returns the latest auth export.
    fn get_latest_export(&self) -> anyhow::Result<PathBuf> {
        let pattern = format!("{}/ente_auth*.txt", self.export_path);

        let mut latest: Option<(String, PathBuf)> = None;
        let mut fallback: Option<PathBuf> = None;

        for entry in glob(&pattern)?.filter_map(Result::ok) {
            let Some(name) = entry.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            if name == "ente_auth.txt" {
                fallback = Some(entry);
                continue;
            }

            if let Some(timestamp) = name
                .strip_prefix("ente_auth_")
                .and_then(|s| s.strip_suffix(".txt"))
            {
                match &latest {
                    Some((latest_timestamp, _)) if timestamp <= latest_timestamp.as_str() => {}
                    _ => latest = Some((timestamp.to_owned(), entry)),
                }
            }
        }

        let path = latest
            .map(|(_, path)| path)
            .or(fallback)
            .ok_or_else(|| anyhow::anyhow!("No Ente Auth export found."))?;

        Ok(path)
    }
}

#[async_trait]
impl Service for EnteService {
    fn name(&self) -> &str {
        "ente-auth"
    }

    async fn export(&self) -> anyhow::Result<Vec<ExportItem>> {
        println!("Exporting TOTP data...");
        let _ = Command::new("ente").arg("export").output().await?;

        let path = self.get_latest_export()?;

        let temp_file = NamedTempFile::new()?;
        fs::copy(&path, temp_file.path()).await?;

        let (temp_file, format) = self
            .maybe_encrypt(temp_file, "txt", self.encrypt_password.as_deref())
            .await?;

        let _ = fs::remove_file(&path).await;

        Ok(vec![ExportItem::new("totp", format, temp_file)])
    }
}
