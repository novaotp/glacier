use chrono::Local;
use glacier_core::{
    config::Config,
    service::{Service, nextcloud::NextcloudService, outline::OutlineService},
    storage::{ArchiveDescriptor, Storage, local::LocalStorage, s3::S3Storage},
};

use crate::cli::{ServiceTarget, StorageTarget};

/// Backups the services to the storages.
pub async fn backup(
    service_targets: &[ServiceTarget],
    storage_targets: &[StorageTarget],
) -> anyhow::Result<()> {
    println!("Loading configuration...");
    let config = Config::new()?;

    let mut storages: Vec<Box<dyn Storage>> = vec![];
    if storage_targets.contains(&StorageTarget::Local) {
        storages.push(Box::new(LocalStorage::new(config.local).await?));
    }

    if storage_targets.contains(&StorageTarget::S3) {
        storages.push(Box::new(S3Storage::new(config.s3).await));
    }

    let mut services: Vec<Box<dyn Service>> = vec![];
    if service_targets.contains(&ServiceTarget::Nextcloud) {
        services.push(Box::new(NextcloudService::new(config.nextcloud)?));
    }

    if service_targets.contains(&ServiceTarget::Outline) {
        services.push(Box::new(OutlineService::new(config.outline)?));
    }

    let date = Local::now().format("%Y%m%d_%Hh%M").to_string();

    for service in &services {
        println!("Starting backup for {} service...", service.name());

        let archive_descriptor =
            ArchiveDescriptor::new(&date, service.name(), service.file_extension());

        let data = service.export().await?;
        let data_path = data.path();

        let uploads = storages.iter().map(|storage| async {
            println!("Putting {} backup in {}...", service.name(), storage.name());

            storage.upload(&archive_descriptor, data_path).await
        });

        futures::future::try_join_all(uploads).await?;
    }

    println!("Completed backups");

    Ok(())
}
