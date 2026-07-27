use std::path::Path;

use async_trait::async_trait;
use tokio::fs;

use crate::{
    config::ConfigLocal,
    storage::{ArchiveDescriptor, Storage},
};

/// A client for interacting with a local storage.
#[derive(Debug, Clone)]
pub struct LocalStorage {
    output_path: String,
}

impl LocalStorage {
    /// Initializes a new local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glacier_core::{config::ConfigLocal, storage::local::LocalStorage};
    ///
    /// # #[tokio::test]
    /// # async fn try_main() -> anyhow::Result<()> {
    /// let config = ConfigLocal {
    ///     output_path: String::from("./output"),
    /// };
    ///
    /// let local = LocalStorage::new(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// If any directory specified by [`config.output_path`](LocalStorage::output_path) does not already exist and could not be created otherwise.
    pub async fn new(config: ConfigLocal) -> anyhow::Result<Self> {
        fs::create_dir_all(&config.output_path).await?;

        Ok(Self {
            output_path: config.output_path,
        })
    }
}

#[async_trait]
impl Storage for LocalStorage {
    fn name(&self) -> &str {
        "local"
    }

    async fn upload(
        &self,
        archive_descriptor: &ArchiveDescriptor,
        data_path: &Path,
    ) -> anyhow::Result<()> {
        let path = format!("{}/{}", self.output_path, archive_descriptor.to_filename());

        fs::copy(data_path, path).await?;

        Ok(())
    }
}
