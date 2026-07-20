use chrono::Local;
use coolify_backups::{
    environment::Environment, exporter::Exporter, outline::OutlineExporter, s3::S3,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    println!("Loading environment...");
    let environment = Environment::load()?;

    let s3 = S3::new(
        environment.s3.region,
        environment.s3.endpoint,
        environment.s3.access_key,
        environment.s3.secret_key,
    )
    .await;

    let exporters = [OutlineExporter::new(
        environment.outline.api_url,
        environment.outline.api_key,
    )?];

    for exporter in exporters {
        let data = exporter.export().await?;

        println!("Putting {} backup in S3...", exporter.name());

        s3.put(
            environment.s3.bucket.clone(),
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
