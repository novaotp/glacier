use chrono::Local;
use coolify_backups::{
    config::Config,
    service::{Service as _, outline::OutlineService},
    storage::{ArchiveDescriptor, Storage, local::LocalStorage, s3::S3Storage},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    println!("Loading configuration...");
    let config = Config::new()?;

    if !config.local.enabled && !config.s3.enabled {
        println!("No storage enabled. Aborting...");
        return Ok(());
    }

    if !config.outline.enabled {
        println!("No services to export enabled. Aborting...");
        return Ok(());
    }

    let mut storages: Vec<Box<dyn Storage>> = vec![];
    if config.local.enabled {
        storages.push(Box::new(LocalStorage::new(config.local).await?));
    }

    if config.s3.enabled {
        storages.push(Box::new(S3Storage::new(config.s3).await));
    }

    let services = [OutlineService::new(config.outline)?];

    let date = Local::now().format("%Y%m%d_%Hh%M").to_string();

    for service in &services {
        println!("Starting backup for {} service...", service.name());

        let archive_descriptor =
            ArchiveDescriptor::new(&date, service.name(), service.file_extension());

        let data = service.export().await?;
        let data_path = data.path();

        for storage in &storages {
            println!("Putting {} backup in {}...", service.name(), storage.name());

            storage.upload(&archive_descriptor, data_path).await?;
        }
    }

    println!("Completed backups");

    Ok(())
}
