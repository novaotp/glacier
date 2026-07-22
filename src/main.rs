use chrono::Local;
use coolify_backups::{config::Config, exporter::Exporter as _, outline::OutlineExporter, s3::S3};

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

    let s3 = S3::new(config.s3.clone()).await;
    let exporters = [OutlineExporter::new(config.outline)?];

    for exporter in exporters {
        let data = exporter.export().await?;

        println!("Putting {} backup in S3...", exporter.name());

        s3.put(
            config.s3.bucket.clone(),
            generate_path(exporter.name()),
            data,
        )
        .await?;
    }

    println!("Completed backups");

    Ok(())
}

/// Generates a path for the given service.
fn generate_path(name: &str) -> String {
    let date = Local::now().format("%Y%m%d").to_string();

    format!("data/{name}/backups/automatic/{date}_{name}_backup.zip")
}
