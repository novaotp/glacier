use std::process::Output;
use tokio::process::Command;

use crate::config::BitwardenAuth;

/// A wrapper client for interacting with the global Bitwarden CLI (`bw`) binary.
pub struct BitwardenClient;

impl BitwardenClient {
    /// Creates a new Bitwarden client instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Authenticates with the Bitwarden CLI using API credentials.
    ///
    /// # Errors
    ///
    /// Return an error if :
    /// - the child process cannot be spawned or if there is an error while awaiting its status.
    /// - the command exited with a non-zero status.
    pub async fn login(&self, credentials: &BitwardenAuth) -> anyhow::Result<()> {
        let mut command = Command::new("bw");

        command.env("BW_CLIENTID", &credentials.client_id);
        command.env("BW_CLIENTSECRET", &credentials.client_secret);

        let output = command
            .arg("login")
            .arg("--apikey")
            .arg("--nointeraction")
            .output()
            .await?;
        let _ = bail_on_err(output)?;

        Ok(())
    }

    /// Unlocks the vault using the master password and returns the session key token.
    ///
    /// # Errors
    ///
    /// Return an error if :
    /// - the child process cannot be spawned or if there is an error while awaiting its status.
    /// - the command exited with a non-zero status.
    /// - the output is not UTF-8.
    pub async fn unlock(&self, master_password: impl Into<String>) -> anyhow::Result<String> {
        let mut command = Command::new("bw");

        command.env("BW_PASSWORD", master_password.into());

        let output = command
            .arg("unlock")
            .args(["--passwordenv", "BW_PASSWORD"])
            .arg("--nointeraction")
            .arg("--raw")
            .output()
            .await?;
        let stdout = bail_on_err(output)?;

        let session = String::from_utf8(stdout)?.trim().to_owned();

        Ok(session)
    }

    /// Exports vault data to the specified file path using an active session key.
    ///
    /// # Errors
    ///
    /// Return an error if :
    /// - the child process cannot be spawned or if there is an error while awaiting its status.
    /// - the command exited with a non-zero status.
    pub async fn export(
        &self,
        path: impl Into<String>,
        format: BitwardenExportFormat,
        session: impl Into<String>,
    ) -> anyhow::Result<()> {
        let mut command = Command::new("bw");

        command.env("BW_SESSION", session.into());

        command
            .arg("export")
            .arg("--nointeraction")
            .arg("--output")
            .arg(path.into());

        match format {
            BitwardenExportFormat::Csv => {
                command.args(["--format", "csv"]);
            }
            BitwardenExportFormat::Json => {
                command.args(["--format", "json"]);
            }
            BitwardenExportFormat::Zip => {
                command.args(["--format", "zip"]);
            }
        };

        let output = command.output().await?;
        let _ = bail_on_err(output)?;

        Ok(())
    }

    /// Clears the current login session.
    ///
    /// # Errors
    ///
    /// Return an error if :
    /// - the child process cannot be spawned or if there is an error while awaiting its status.
    /// - the command exited with a non-zero status.
    pub async fn logout(&self) -> anyhow::Result<()> {
        let mut command = Command::new("bw");

        let output = command
            .arg("logout")
            .arg("--nointeraction")
            .output()
            .await?;

        let _ = bail_on_err(output)?;

        Ok(())
    }
}

/// Returns an error if the output exited with a non-zero status, otherwise returns `stdout`.
fn bail_on_err(output: Output) -> anyhow::Result<Vec<u8>> {
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);

        anyhow::bail!(message.into_owned())
    }

    Ok(output.stdout)
}

/// The serialization format for vault exports.
///
/// Does not support `encrypted_json`.
pub enum BitwardenExportFormat {
    Csv,
    Json,
    Zip,
}
