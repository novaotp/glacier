pub mod nextcloud;
pub mod outline;

use async_trait::async_trait;
use tempfile::NamedTempFile;

/// A trait for any service that can export its data.
#[async_trait]
pub trait Service {
    /// Returns the unique name of the service in lower case.
    fn name(&self) -> &str;

    /// Returns the file extension used by the export, without the leading dot.
    fn file_extension(&self) -> &str;

    /// Performs the export and returns the data in a temporary file.
    async fn export(&self) -> anyhow::Result<NamedTempFile>;
}
