use std::{
    fs::File,
    io::{BufWriter, Seek as _, Write as _},
};

use flate2::{Compression, write::GzEncoder};
use futures_util::StreamExt as _;
use reqwest::{Client, Method};
use tempfile::NamedTempFile;

use crate::service::nextcloud::types::Multistatus;

/// A client to interact with a Nextcloud instance.
pub struct NextcloudClient {
    /// The underlying [reqwest] client.
    client: Client,
    /// The Nextcloud URL.
    url: String,
}

/// Credentials used to authenticate requests to Nextcloud.
#[derive(Debug)]
struct BasicAuth {
    /// The username of the Nextcloud account.
    username: String,
    /// The app password of the Nextcloud account for this service.
    password: String,
}

impl BasicAuth {
    /// Creates a new set of credentials.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl NextcloudClient {
    /// Creates a new `NextcloudClient`.
    ///
    /// # Errors
    ///
    /// If the [`reqwest::Client`](Client) cannot be built.
    pub fn new(url: String) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::builder().build()?,
            url,
        })
    }

    /// Exports all files for the given user into a gzip-compressed tar archive.
    ///
    /// # Errors
    ///
    /// Returns an error if traversing the WebDAV hierarchy, downloading a file, or writing the archive fails.
    pub async fn export_all(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> anyhow::Result<NamedTempFile> {
        let temp_file = NamedTempFile::new()?;
        let encoder = GzEncoder::new(temp_file, Compression::default());
        let mut tar = tar::Builder::new(encoder);

        let auth = BasicAuth::new(username, password);
        self.append_to_tar(&mut tar, &auth).await?;

        let file = tar.into_inner()?.finish()?;

        Ok(file)
    }

    /// Traverses the user's WebDAV hierarchy and appends each file to the archive.
    async fn append_to_tar(
        &self,
        tar: &mut tar::Builder<GzEncoder<NamedTempFile>>,
        auth: &BasicAuth,
    ) -> anyhow::Result<()> {
        let mut stack = vec![String::new()];

        while let Some(folder) = stack.pop() {
            let multistatus = self.propfind(&folder, auth).await?;

            for response in &multistatus.responses {
                let folder_path = self.remove_webdav_prefix(&response.href, &auth.username);

                // A PROPFIND response includes the queried collection itself.
                // Skip it to avoid traversing the same directory repeatedly.
                if folder_path == folder {
                    continue;
                }

                if response.is_file() {
                    let mut file = self.get_file(&response.href, auth).await?;

                    let last_modified = chrono::DateTime::parse_from_rfc2822(
                        &response.propstat.prop.last_modified,
                    )?
                    .timestamp() as u64;

                    let mut header = tar::Header::new_gnu();
                    header.set_size(file.metadata()?.len());
                    header.set_mtime(last_modified);
                    header.set_mode(0o644);
                    header.set_cksum();

                    tar.append_data(&mut header, folder_path, &mut file)?;
                } else {
                    stack.push(folder_path.to_owned());
                }
            }
        }

        Ok(())
    }

    /// Performs a `PROPFIND` request for the specified directory.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error :
    /// - while sending request, redirect loop was detected or redirect limit was exhausted.
    /// - decoding the body as text.
    /// - deserializing the XML content.
    async fn propfind(&self, folder: &str, auth: &BasicAuth) -> anyhow::Result<Multistatus> {
        let method = Method::from_bytes("PROPFIND".as_bytes())?;
        let url = format!(
            "{}/{}",
            self.get_webdav_path(&auth.username),
            folder.trim_start_matches('/')
        );

        let response = self
            .client
            .request(method, url)
            .basic_auth(&auth.username, Some(&auth.password))
            .send()
            .await?
            .error_for_status()?;

        let xml = response.text().await?;
        quick_xml::de::from_str(&xml).map_err(Into::into)
    }

    /// Downloads a file into a temporary file.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error :
    /// - while sending request, redirect loop was detected or redirect limit was exhausted.
    /// - with the the temporary file (can not be created, flushing, rewinding).
    /// - receiving a file chunk.
    async fn get_file(&self, path: &str, auth: &BasicAuth) -> anyhow::Result<File> {
        let url = format!("{}{}", self.url, path);

        let response = self
            .client
            .get(url)
            .basic_auth(&auth.username, Some(&auth.password))
            .send()
            .await?
            .error_for_status()?;

        let mut stream = response.bytes_stream();

        let mut temp_file = BufWriter::new(tempfile::tempfile()?);

        while let Some(chunk) = stream.next().await {
            temp_file.write_all(&chunk?)?;
        }

        let mut temp_file = temp_file.into_inner()?;
        temp_file.flush()?;
        temp_file.rewind()?;

        Ok(temp_file)
    }

    /// Returns the root WebDAV URL for the given user.
    fn get_webdav_path(&self, username: &str) -> String {
        format!("{}/remote.php/dav/files/{}", self.url, username)
    }

    /// Removes the `/remote.php/dav/files/<user>/` prefix from a WebDAV path.
    ///
    /// The returned path is suitable for use as the path of an entry within the generated tar
    /// archive as it removes the starting `/`.
    fn remove_webdav_prefix(&self, href: &str, username: &str) -> String {
        // Include last `/` because tar doesn't allow writing with absolute paths
        let prefix = format!("/remote.php/dav/files/{}/", username);

        href.trim_start_matches(&prefix).to_owned()
    }
}
