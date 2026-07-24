use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub local: ConfigLocal,
    pub s3: ConfigS3,
    pub outline: ConfigOutline,
    pub nextcloud: ConfigNextcloud,
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
