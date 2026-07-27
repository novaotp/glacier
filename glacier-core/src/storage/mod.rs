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

/// Describes an exported artifact for constructing a storage-specific path or object key.
pub struct ArchiveDescriptor {
    /// The date of the export.
    pub date: String,
    /// The name of the service that produced the artifact.
    pub service_name: String,
    /// The logical name of the exported artifact.
    pub item_name: String,
    /// The file extension of the exported artifact.
    pub extension: String,
}

impl ArchiveDescriptor {
    /// Creates a new archive descriptor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use glacier_core::storage::ArchiveDescriptor;
    ///
    /// let archive_descriptor = ArchiveDescriptor::new(
    ///     "20260727_09h38",
    ///     "bitwarden",
    ///     "passwords",
    ///     "csv"
    /// );
    /// ```
    pub fn new(
        date: impl Into<String>,
        service_name: impl Into<String>,
        item_name: impl Into<String>,
        extension: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            service_name: service_name.into(),
            item_name: item_name.into(),
            extension: extension.into(),
        }
    }

    /// Returns the default filename for the exported artifact.
    ///
    /// The filename follows the format `<date>_<service_name>_<item_name>.<extension>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use glacier_core::storage::ArchiveDescriptor;
    ///
    /// let archive_descriptor = ArchiveDescriptor::new(
    ///     "20260727_09h38",
    ///     "bitwarden",
    ///     "passwords",
    ///     "csv"
    /// );
    ///
    /// assert_eq!(
    ///     "20260727_09h38_bitwarden_passwords.csv",
    ///     archive_descriptor.to_filename()
    /// );
    /// ```
    pub fn to_filename(&self) -> String {
        format!(
            "{}_{}_{}.{}",
            self.date, self.service_name, self.item_name, self.extension
        )
    }
}
