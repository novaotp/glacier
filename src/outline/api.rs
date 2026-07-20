use serde::Deserialize;
use std::collections::HashMap;

/// The API response returned by an endpoint.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        ok: bool,
        status: u16,
        data: T,
        success: Option<bool>,
    },
    Failure {
        ok: bool,
        status: u16,
        error: String,
        message: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
    },
}

/// The payload returned by the `collections.export_all` endpoint.
#[derive(Debug, Deserialize)]
pub struct ExportCollections {
    /// The asynchronous file operation created for this export request.
    #[serde(rename = "fileOperation")]
    pub file_operation: FileOperation,
}

/// Represents the state of a background file operation.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum FileOperationState {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "expired")]
    Expired,
}

/// Represents the status and metadata of a background file operation.
#[derive(Debug, Deserialize)]
pub struct FileOperation {
    /// Unique identifier used to poll or download the generated archive.
    pub id: String,
    /// The current state of the file operation.
    pub state: FileOperationState,
}
