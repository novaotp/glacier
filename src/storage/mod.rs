pub mod local;
pub mod s3;

use std::path::Path;

use async_trait::async_trait;

/// A trait for any storage that can store some data.
#[async_trait]
pub trait Storage {
    /// Returns the unique name of the storage in lower case.
    fn name(&self) -> &str;

    /// Uploads the given data to a storage endpoint
    ///
    /// # Arguments
    ///
    /// * `archive_descriptor`: Information to build a path or object key.
    /// * `data_path`: The path to the file containing the data.
    async fn upload(
        &self,
        archive_descriptor: &ArchiveDescriptor,
        data_path: &Path,
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
