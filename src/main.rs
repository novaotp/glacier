use chrono::Local;
use coolify_backups::{
    config::Config,
    exporter::Exporter as _,
    outline::OutlineExporter,
    storage::{ArchiveDescriptor, Storage as _, s3::S3Storage},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    println!("Loading configuration...");
    let config = Config::new()?;

    if !config.s3.enabled {
        println!("No storage enabled. Aborting...");
        return Ok(());
    }

    if !config.outline.enabled {
        println!("No services to export enabled. Aborting...");
        return Ok(());
    }

    let s3 = S3Storage::new(config.s3.clone()).await;
    let exporters = [OutlineExporter::new(config.outline)?];

    let date = Local::now().format("%Y%m%d_%Hh%M").to_string();

    for exporter in exporters {
        let data = exporter.export().await?;

        println!("Putting {} backup in S3...", exporter.name());

        s3.upload(ArchiveDescriptor::new(&date, exporter.name()), data)
            .await?;
    }

    println!("Completed backups");

    Ok(())
}
