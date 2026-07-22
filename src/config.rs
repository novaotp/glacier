use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub s3: ConfigS3,
    pub outline: ConfigOutline,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigS3 {
    pub enabled: bool,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigOutline {
    pub enabled: bool,
    pub url: String,
    pub api_key: String,
}

impl Config {
    /// Creates a new configuration based on environment variables.
    pub fn new() -> anyhow::Result<Self> {
        Ok(config::Config::builder()
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize::<Config>()?)
    }
}
