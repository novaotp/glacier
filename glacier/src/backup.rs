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

        let export_items = service.export().await?;

        let uploads = export_items.iter().flat_map(|item| {
            let date = date.clone();

            storages.iter().map(move |storage| {
                let date = date.clone();

                async move {
                    println!(
                        "Putting {} ({}) backup in {}...",
                        service.name(),
                        item.name,
                        storage.name()
                    );

                    let archive_descriptor =
                        ArchiveDescriptor::new(&date, service.name(), &item.name, &item.extension);

                    storage.upload(&archive_descriptor, item.file.path()).await
                }
            })
        });

        futures::future::try_join_all(uploads).await?;
    }

    println!("Completed backups");

    Ok(())
}
