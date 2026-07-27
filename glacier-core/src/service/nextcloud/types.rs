use serde::{Deserialize, Serialize};

/// The response body returned by a WebDAV `PROPFIND` request.
#[derive(Debug, Serialize, Deserialize)]
pub struct Multistatus {
    /// All items inside the given directory.
    #[serde(rename = "response")]
    pub responses: Vec<Response>,
}

/// Metadata for a single resource returned by a `PROPFIND` request.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    /// The path of the resource, beginning with `/remote.php/dav/...`.
    #[serde(rename = "href")]
    pub href: String,
    /// The ressource's properties.
    #[serde(rename = "propstat")]
    pub propstat: Propstat,
}

impl Response {
    /// Returns `true` if this resource represents a file rather than a collection.
    pub fn is_file(&self) -> bool {
        self.propstat.prop.resource_type.collection.is_none()
    }
}

/// The properties associated with a resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Propstat {
    /// The subset of WebDAV properties required for creating the backup archive.
    #[serde(rename = "prop")]
    pub prop: Prop,
    /// The HTTP status for the enclosed properties (for example, `HTTP/1.1 200 OK`).
    #[serde(rename = "status")]
    pub status: String,
}

/// The subset of WebDAV properties required for creating the backup archive.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prop {
    /// Identifies the type of the WebDAV resource.
    #[serde(rename = "resourcetype")]
    pub resource_type: Resourcetype,
    /// The last modification time of the resource as an RFC 2822 date-time string.
    #[serde(rename = "getlastmodified")]
    pub last_modified: String,
    /// The size of the resource in bytes.
    ///
    /// This property is only present for files.
    #[serde(rename = "getcontentlength")]
    pub content_length: Option<String>,
}

/// The type of the WebDAV resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Resourcetype {
    /// Present if the resource is a collection (directory).
    ///
    /// If this field is `None`, the resource is a file.
    #[serde(rename = "collection")]
    pub collection: Option<Collection>,
}

/// A directory.
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection;
