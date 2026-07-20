mod api;

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{Client, StatusCode, header};
use tokio::time;

use crate::{
    exporter::Exporter,
    outline::api::{ApiResponse, ExportCollections, FileOperation, FileOperationState},
};

/// The exporter for the Outline service.
pub struct OutlineExporter {
    api_url: String,
    client: Client,
}

impl OutlineExporter {
    /// Creates a new `OutlineExporter`.
    ///
    /// # Errors
    ///
    /// If the reqwest [Client] cannot be built.
    pub fn new(api_url: String, api_key: String) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::try_from(&format!("Bearer {}", api_key))?;
        auth_value.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { api_url, client })
    }

    /// Internal helper to execute authenticated requests.
    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> anyhow::Result<T> {
        let response = self
            .client
            .post(format!("{}/api/{}", self.api_url, path))
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

    /// Triggers an export of all collections.
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
    async fn get_operation_info(&self, id: &str) -> anyhow::Result<FileOperation> {
        self.post("fileOperations.info", serde_json::json!({ "id": id }))
            .await
    }
}

#[async_trait]
impl Exporter for OutlineExporter {
    fn name(&self) -> &str {
        "outline"
    }

    async fn export(&self) -> anyhow::Result<Bytes> {
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
            .get(format!("{}/api/fileOperations.redirect", self.api_url))
            .query(&[("id", &file_operation.id)])
            .send()
            .await?;

        let data = response.bytes().await?;

        Ok(data)
    }
}
