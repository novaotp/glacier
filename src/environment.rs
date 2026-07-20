use anyhow::Context;
use std::env;

/// Centralized configuration for the application, mapping environment variables to structured data.
pub struct Environment {
    /// S3 storage connection details.
    pub s3: S3Environment,
    /// Outline API authentication and endpoint details.
    pub outline: OutlineEnvironment,
}

/// Configuration settings for the S3-compatible storage service.
pub struct S3Environment {
    /// The target bucket name for uploads.
    pub bucket: String,
    /// The region where the bucket is hosted.
    pub region: String,
    /// The base URL for the S3-compatible service (e.g., Hetzner).
    pub endpoint: String,
    /// The access key ID for authentication.
    pub access_key: String,
    /// The secret access key for authentication.
    pub secret_key: String,
}

/// Configuration settings for the Outline API.
pub struct OutlineEnvironment {
    /// The API key used to authenticate requests to the Outline instance.
    pub api_key: String,
    /// The base URL of the Outline instance.
    pub api_url: String,
}

impl Environment {
    /// Loads the environment variables from the environment or a `.env` file.
    ///
    /// # Errors
    /// Returns an error if any required environment variable is missing or cannot be read.
    pub fn load() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv()?;

        Ok(Self {
            s3: S3Environment {
                bucket: env::var("S3_BUCKET").context("Missing S3_BUCKET")?,
                region: env::var("S3_REGION").context("Missing S3_REGION")?,
                endpoint: env::var("S3_ENDPOINT").context("Missing S3_ENDPOINT")?,
                access_key: env::var("S3_ACCESS_KEY").context("Missing S3_ACCESS_KEY")?,
                secret_key: env::var("S3_SECRET_KEY").context("Missing S3_SECRET_KEY")?,
            },
            outline: OutlineEnvironment {
                api_key: env::var("OUTLINE_API_KEY").context("Missing OUTLINE_API_KEY")?,
                api_url: env::var("OUTLINE_URL").context("Missing OUTLINE_URL")?,
            },
        })
    }
}
