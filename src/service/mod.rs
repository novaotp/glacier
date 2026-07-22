pub mod outline;

use async_trait::async_trait;
use bytes::Bytes;

/// A trait for any service that can export its data.
#[async_trait]
pub trait Service {
    /// Returns the unique name of the service in lower case.
    fn name(&self) -> &str;

    /// Performs the export and returns the data as a bytes.
    async fn export(&self) -> anyhow::Result<Bytes>;
}
