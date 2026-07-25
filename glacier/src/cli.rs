use std::path::PathBuf;

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
    /// Backup one or more services.
    Backup(BackupArgs),
    /// Encrypts a file.
    Encrypt(EncryptArgs),
    /// Decrypts a file.
    Decrypt(DecryptArgs),
}

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

#[derive(Args, Debug)]
pub struct EncryptArgs {
    /// Input file to encrypt.
    pub input: PathBuf,
    /// Output file for the encrypted data.
    pub output: PathBuf,
    /// Encryption password.
    #[arg(long, env = "GLACIER_PASSWORD")]
    pub password: String,
}

#[derive(Args, Debug)]
pub struct DecryptArgs {
    /// Input file to decrypt.
    pub input: PathBuf,
    /// Output file for the decrypted data.
    pub output: PathBuf,
    /// Decryption password.
    #[arg(long, env = "GLACIER_PASSWORD")]
    pub password: String,
}
