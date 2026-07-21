pub mod s3;

use async_trait::async_trait;
use bytes::Bytes;

/// A trait for any storage that can store some data.
#[async_trait]
pub trait Storage {
    /// Returns the unique name of the storage in lower case.
    fn name(&self) -> &str;

    /// Uploads the given data to a storage endpoint
    async fn upload(
        &self,
        archive_descriptor: ArchiveDescriptor,
        data: Bytes,
    ) -> anyhow::Result<()>;
}

/// Describes a backup archive for constructing a storage-specific path or object key.
pub struct ArchiveDescriptor {
    /// The date of the archive.
    pub date: String,
    /// The name of the service.
    pub name: String,
}

impl ArchiveDescriptor {
    /// Creates a new archive descriptor.
    pub fn new(date: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            date: date.into(),
            name: name.into(),
        }
    }
}
