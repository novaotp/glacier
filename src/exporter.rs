use async_trait::async_trait;
use bytes::Bytes;

/// A trait for any service that can export its data.
#[async_trait]
pub trait Exporter {
    /// Returns the unique name of the service (e.g., "outline" or "nextcloud").
    ///
    /// This name is used to generate the storage path or filename  in the destination bucket.
    fn name(&self) -> &str;

    /// Performs the export and returns the data as a buffer.
    ///
    /// # Errors
    ///
    /// The export process can fail due to network timeouts, API authentication issues, or data
    /// serialization failures.
    async fn export(&self) -> anyhow::Result<Bytes>;
}
