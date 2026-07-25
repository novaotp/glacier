use clap::{ArgAction, Args, Parser, ValueEnum, crate_version};
use strum::EnumIter;

#[derive(Parser)]
#[command(name = "Glacier")]
#[command(version = crate_version!())]
#[command(
    about = "A backup service for preserving data from the software I use every day.",
    long_about = None
)]
pub enum GlacierCli {
    Backup(BackupArgs),
}

/// Backup one or more services.
#[derive(Args, Debug)]
pub struct BackupArgs {
    /// Includes all services. Cannot be used with --services.
    #[arg(
        long,
        required_unless_present = "services",
        conflicts_with = "services"
    )]
    pub all_services: bool,

    /// Includes specific services. Cannot be used with --all-services.
    #[arg(
        long,
        value_enum,
        num_args = 1..,
        value_delimiter = ',',
        action = ArgAction::Append,
        conflicts_with = "all_services"
    )]
    pub services: Vec<ServiceTarget>,

    /// Excludes services. Only valid with --all-services.
    #[arg(
        long,
        value_enum,
        requires = "all_services",
        num_args = 1..,
        value_delimiter = ',',
        action = ArgAction::Append
    )]
    pub exclude_services: Vec<ServiceTarget>,

    /// Upload only to the specified storage backends.
    #[arg(
        long,
        value_enum,
        num_args = 1..,
        value_delimiter = ',',
        action = ArgAction::Append,
        conflicts_with = "exclude_storages"
    )]
    pub storages: Vec<StorageTarget>,

    /// Excludes storage backends. Cannot be used with --storages.
    #[arg(
        long,
        value_enum,
        num_args = 1..,
        value_delimiter = ',',
        action = ArgAction::Append,
        conflicts_with = "storages"
    )]
    pub exclude_storages: Vec<StorageTarget>,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, Hash, ValueEnum)]
pub enum ServiceTarget {
    Bitwarden,
    Nextcloud,
    Outline,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, Hash, ValueEnum)]
pub enum StorageTarget {
    Local,
    S3,
}
