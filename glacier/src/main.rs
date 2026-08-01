mod backup;
mod cli;
mod update;

use clap::Parser;
use cli::{GlacierCli, ServiceTarget, StorageTarget};
use glacier_crypto::{decrypt::decrypt_file, encrypt::encrypt_file};
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    match GlacierCli::parse() {
        GlacierCli::Backup(args) => {
            let services = if args.all_services {
                ensure_not_empty(
                    ServiceTarget::iter()
                        .filter(|service| !args.exclude_services.contains(service))
                        .collect(),
                    "services",
                )?
            } else {
                args.services
            };

            let storages = if args.storages.is_empty() {
                ensure_not_empty(
                    StorageTarget::iter()
                        .filter(|storage| !args.exclude_storages.contains(storage))
                        .collect(),
                    "storage backends",
                )?
            } else {
                args.storages
            };

            backup::backup(&services, &storages).await?;
        }
        GlacierCli::Encrypt(args) => encrypt_file(&args.input, &args.output, &args.password)?,
        GlacierCli::Decrypt(args) => decrypt_file(&args.input, &args.output, &args.password)?,
        GlacierCli::Update => update::update()?,
    }

    Ok(())
}

fn ensure_not_empty<T>(items: Vec<T>, what: &str) -> anyhow::Result<Vec<T>> {
    if items.is_empty() {
        anyhow::bail!("No {what} remain after applying exclusions.");
    }

    Ok(items)
}
