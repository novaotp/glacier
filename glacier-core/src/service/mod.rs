pub mod bitwarden;
pub mod nextcloud;
pub mod outline;

use async_trait::async_trait;
use glacier_crypto::encrypt::encrypt_file;
use tempfile::NamedTempFile;
use tokio::task;

/// A trait for any service that can export its data.
#[async_trait]
pub trait Service {
    /// Returns the unique name of the service in lower case.
    fn name(&self) -> &str;

    /// Performs the export and returns one or more exported artifacts.
    async fn export(&self) -> anyhow::Result<Vec<ExportItem>>;

    /// Encrypts the file if the encryption password is set, otherwise returns as-is.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while encrypting the file.
    async fn maybe_encrypt(
        &self,
        temp_file: NamedTempFile,
        format: &str,
        encrypt_password: Option<&str>,
    ) -> anyhow::Result<(NamedTempFile, String)> {
        let Some(password) = encrypt_password else {
            return Ok((temp_file, format.to_owned()));
        };

        let password = password.to_owned();

        let encrypted_temp_file = task::spawn_blocking(move || -> anyhow::Result<NamedTempFile> {
            let encrypted_temp_file = NamedTempFile::new()?;

            println!("Encrypting file...");
            encrypt_file(temp_file.path(), encrypted_temp_file.path(), &password)?;

            Ok(encrypted_temp_file)
        })
        .await??;

        Ok((encrypted_temp_file, format!("{}.glacier", format)))
    }
}

/// A single artifact produced by a service export.
#[derive(Debug)]
pub struct ExportItem {
    /// The logical name of the exported artifact, such as `files` or `dump`.
    pub name: String,
    /// The file extension of the exported artifact, without a leading dot.
    ///
    /// Compound extensions such as `tar.gz` are supported.
    pub extension: String,
    /// The temporary file containing the exported data.
    pub file: NamedTempFile,
}

impl ExportItem {
    /// Creates a new export item.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glacier_core::service::ExportItem;
    /// use tempfile::NamedTempFile;
    ///
    /// // The file containing data.
    /// let file = NamedTempFile::new()?;
    ///
    /// let export_item = ExportItem::new("files", "tar.gz", file);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new(name: impl Into<String>, extension: impl Into<String>, file: NamedTempFile) -> Self {
        Self {
            name: name.into(),
            extension: extension.into(),
            file,
        }
    }
}
