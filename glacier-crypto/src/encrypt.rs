use crate::format::{HEADER_MAGIC, KEY_LEN, NONCE_LEN, SALT_LEN};
use anyhow::Context;
use argon2::Argon2;
use chacha20poly1305::{
    XChaCha20Poly1305,
    aead::{Aead, KeyInit},
};
use rand::{TryRng, rngs::SysRng};
use std::{fs, io::Write, path::Path};

/// Encrypts a file with a password using `Argon2` for key derivation and `XChaCha20Poly1305` for encryption.
///
/// The encrypted output follows the layout: `HEADER_MAGIC | salt_len | nonce_len | salt | nonce | ciphertext`.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use glacier_crypto::encrypt::encrypt_file;
///
/// # fn main() -> anyhow::Result<()> {
/// let input = Path::new("secret.txt");
/// let output = Path::new("secret.enc");
///
/// encrypt_file(input, output, "my_strong_password")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// * Reading the input file fails (e.g., file does not exist or insufficient permissions).
/// * Generating random bytes for the salt or nonce fails.
/// * Key derivation via Argon2 fails.
/// * Encryption of the plaintext fails.
/// * Creating, writing to, flushing, or persisting the output file fails.
pub fn encrypt_file(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    password: &str,
) -> anyhow::Result<()> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();

    let plaintext =
        fs::read(input_path).with_context(|| format!("reading input file {:?}", input_path))?;

    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    SysRng.try_fill_bytes(&mut salt)?;
    SysRng.try_fill_bytes(&mut nonce)?;

    let mut key = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| anyhow::anyhow!("argon2 key derivation failed: {e}"))?;

    let cipher = XChaCha20Poly1305::new((&key).into());
    let ciphertext = cipher
        .encrypt(&nonce.into(), plaintext.as_ref())
        .context("encryption failed")?;

    let mut out = fs::File::create(output_path)
        .with_context(|| format!("creating output file {:?}", output_path))?;

    out.write_all(HEADER_MAGIC)?;
    out.write_all(&[SALT_LEN as u8])?;
    out.write_all(&[NONCE_LEN as u8])?;
    out.write_all(&salt)?;
    out.write_all(&nonce)?;
    out.write_all(&ciphertext)?;
    out.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_encrypt_file_valid() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("input.txt");
        let output_path = dir.path().join("output.enc");

        fs::write(&input_path, b"confidential payload").unwrap();

        let result = encrypt_file(&input_path, &output_path, "secure_password");
        assert!(result.is_ok());
        assert!(output_path.exists());

        let encrypted_data = fs::read(&output_path).unwrap();
        assert_eq!(&encrypted_data[0..8], HEADER_MAGIC);

        let expected_min_len = 8 + 1 + 1 + SALT_LEN + NONCE_LEN;
        assert!(encrypted_data.len() > expected_min_len);
    }

    #[test]
    fn test_encrypt_file_nonexistent_input() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("does_not_exist.txt");
        let output_path = dir.path().join("output.enc");

        let result = encrypt_file(&input_path, &output_path, "password");
        assert!(result.is_err());
        assert!(!output_path.exists());
    }

    #[test]
    fn test_encrypt_file_empty_plaintext() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("empty.txt");
        let output_path = dir.path().join("output.enc");

        fs::write(&input_path, b"").unwrap();

        let result = encrypt_file(&input_path, &output_path, "password");
        assert!(result.is_ok());

        let encrypted_data = fs::read(&output_path).unwrap();
        let expected_header_len = 8 + 1 + 1 + SALT_LEN + NONCE_LEN;
        assert!(encrypted_data.len() >= expected_header_len);
    }
}
