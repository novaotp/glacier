use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub local: ConfigLocal,
    pub s3: ConfigS3,
    pub outline: ConfigOutline,
    pub nextcloud: ConfigNextcloud,
    pub bitwarden: ConfigBitwarden,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigLocal {
    pub output_path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigS3 {
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigOutline {
    pub url: String,
    pub api_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigNextcloud {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigBitwarden {
    pub auth: BitwardenAuth,
    pub master_password: String,
    pub format: String,
    pub encrypt_password: Option<String>,
}

/// The credentials to authenticate using the API key.
#[derive(Clone, Debug, Deserialize)]
pub struct BitwardenAuth {
    pub client_id: String,
    pub client_secret: String,
}

impl Config {
    /// Creates a new configuration based on config files and environment variables.
    pub fn new() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config_file("~/.config/glacier.toml"))
            .add_source(config_file(".config/glacier.toml"))
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()?;

        Ok(config)
    }
}

fn config_file(path: &str) -> config::File<config::FileSourceFile, config::FileFormat> {
    let expanded_path = shellexpand::tilde(path);
    let path = Path::new(expanded_path.as_ref());

    config::File::from(path).required(false)
}
