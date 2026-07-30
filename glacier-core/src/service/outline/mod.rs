mod api;

use std::io::Write;

use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::{Client, StatusCode, header};
use tempfile::NamedTempFile;
use tokio::time;

use crate::{
    config::ConfigOutline,
    service::{
        ExportItem, Service,
        outline::api::{ApiResponse, ExportCollections, FileOperation, FileOperationState},
    },
};

/// The Outline service.
pub struct OutlineService {
    client: Client,
    url: String,
    encrypt_password: Option<String>,
}

impl OutlineService {
    /// Creates a new `OutlineService`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use glacier_core::{config::ConfigOutline, service::outline::OutlineService};
    ///
    /// let config = ConfigOutline {
    ///     url: String::from("https://outline.example.com"),
    ///     api_key: String::from("YOUR_API_KEY"),
    ///     encrypt_password: None,
    /// };
    ///
    /// let outline = OutlineService::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// If the reqwest [Client] cannot be built.
    pub fn new(config: ConfigOutline) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::try_from(&format!("Bearer {}", config.api_key))?;
        auth_value.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            url: config.url,
            encrypt_password: config.encrypt_password,
        })
    }

    /// Triggers an export of all collections.
    ///
    /// # Errors
    ///
    /// See [`post`](OutlineService::post) for more information.
    async fn export_collections(&self) -> anyhow::Result<FileOperation> {
        println!("Exporting data...");

        let data: ExportCollections = self
            .post(
                "collections.export_all",
                serde_json::json!({
                    "format": "json",
                    "includeAttachments": true,
                    "includePrivate": true
                }),
            )
            .await?;

        Ok(data.file_operation)
    }

    /// Checks the status of a file operation.
    ///
    /// # Errors
    ///
    /// See [`post`](OutlineService::post) for more information.
    async fn get_operation_info(&self, id: &str) -> anyhow::Result<FileOperation> {
        self.post("fileOperations.info", serde_json::json!({ "id": id }))
            .await
    }

    /// Internal helper to execute authenticated requests.
    ///
    /// # Errors
    ///
    /// This method fails if :
    /// - there was an error while sending request, redirect loop was detected or redirect limit was exhausted.
    /// - the response body is not in JSON format, or it cannot be properly deserialized to target type T.
    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> anyhow::Result<T> {
        let response = self
            .client
            .post(format!("{}/api/{}", self.url, path))
            .json(&body)
            .send()
            .await?;

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            let retry = response
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok());

            return Err(anyhow::anyhow!(
                "Rate limited. Retry after {} seconds.",
                retry.unwrap_or("UNKNOWN")
            ));
        }

        match response.json::<ApiResponse<T>>().await? {
            ApiResponse::Success { data, .. } => Ok(data),
            ApiResponse::Failure { message, error, .. } => {
                Err(anyhow::anyhow!(message.unwrap_or(error)))
            }
        }
    }
}

#[async_trait]
impl Service for OutlineService {
    fn name(&self) -> &str {
        "outline"
    }

    async fn export(&self) -> anyhow::Result<Vec<ExportItem>> {
        let mut file_operation = self.export_collections().await?;

        for _ in 0..3 {
            if file_operation.state == FileOperationState::Complete {
                break;
            } else if file_operation.state == FileOperationState::Expired {
                return Err(anyhow::anyhow!("Export expired on server"));
            } else if file_operation.state == FileOperationState::Error {
                return Err(anyhow::anyhow!("Export failed on server"));
            }

            println!("Checking for file readiness...");

            time::sleep(time::Duration::from_secs(10)).await;
            file_operation = self.get_operation_info(&file_operation.id).await?;
        }

        if file_operation.state != FileOperationState::Complete {
            return Err(anyhow::anyhow!("File not ready after polling"));
        }

        println!("Downloading file...");

        let response = self
            .client
            .get(format!("{}/api/fileOperations.redirect", self.url))
            .query(&[("id", &file_operation.id)])
            .send()
            .await?
            .error_for_status()?;

        let mut stream = response.bytes_stream();

        let mut temp_file = NamedTempFile::new()?;
        while let Some(chunk) = stream.next().await {
            temp_file.write_all(&chunk?)?;
        }

        temp_file.flush()?;

        let (temp_file, format) = self
            .maybe_encrypt(temp_file, "zip", self.encrypt_password.as_deref())
            .await?;

        Ok(vec![ExportItem::new("files", format, temp_file)])
    }
}
