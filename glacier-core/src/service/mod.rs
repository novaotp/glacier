pub mod bitwarden;
pub mod nextcloud;
pub mod outline;

use async_trait::async_trait;
use tempfile::NamedTempFile;

/// A trait for any service that can export its data.
#[async_trait]
pub trait Service {
    /// Returns the unique name of the service in lower case.
    fn name(&self) -> &str;

    /// Performs the export and returns one or more exported artifacts.
    async fn export(&self) -> anyhow::Result<Vec<ExportItem>>;
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
    pub fn new(name: impl Into<String>, extension: impl Into<String>, file: NamedTempFile) -> Self {
        Self {
            name: name.into(),
            extension: extension.into(),
            file,
        }
    }
}
