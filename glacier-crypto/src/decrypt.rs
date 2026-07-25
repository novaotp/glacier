use crate::format::{KEY_LEN, parse_header};
use anyhow::{Context, Result, anyhow};
use argon2::Argon2;
use chacha20poly1305::{
    XChaCha20Poly1305,
    aead::{Aead, KeyInit},
};
use std::{fs, path::Path};

/// Decrypts a file with a password using `Argon2` for key derivation and `XChaCha20Poly1305` for decryption.
///
/// Expects the input file to follow the layout produced by [`encrypt_file`]:
/// `HEADER_MAGIC | salt_len | nonce_len | salt | nonce | ciphertext`.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use glacier_crypto::decrypt::decrypt_file;
///
/// # fn main() -> anyhow::Result<()> {
/// let input = Path::new("secret.enc");
/// let output = Path::new("secret.txt");
///
/// decrypt_file(input, output, "my_strong_passphrase")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// * Reading the input file fails (e.g., file does not exist or insufficient permissions).
/// * Parsing the file header fails (due to incorrect magic bytes, truncated data, or mismatched lengths).
/// * Key derivation via Argon2 fails.
/// * Converting the nonce slice into a fixed-size array fails.
/// * Decryption fails (e.g., due to an incorrect password or tampered ciphertext).
/// * Writing the decrypted plaintext to the output file fails.
pub fn decrypt_file(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    password: &str,
) -> Result<()> {
    let input_path = input.as_ref();
    let output_path = output.as_ref();

    let data =
        fs::read(input_path).with_context(|| format!("reading input file {:?}", input_path))?;

    let parsed_header = parse_header(&data)?;

    let mut key = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(password.as_bytes(), parsed_header.salt, &mut key)
        .map_err(|e| anyhow!("argon2 key derivation failed: {e}"))?;

    let cipher = XChaCha20Poly1305::new((&key).into());
    let plaintext = cipher
        .decrypt(&parsed_header.nonce.try_into()?, parsed_header.ciphertext)
        .context("decryption failed")?;

    fs::write(output_path, plaintext)
        .with_context(|| format!("writing output file {:?}", output_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{HEADER_MAGIC, NONCE_LEN, SALT_LEN};
    use tempfile::tempdir;

    fn build_encrypted_file(password: &str, plaintext: &[u8]) -> Vec<u8> {
        let salt = [1u8; SALT_LEN];
        let nonce = [2u8; NONCE_LEN];

        let mut key = [0u8; KEY_LEN];
        Argon2::default()
            .hash_password_into(password.as_bytes(), &salt, &mut key)
            .unwrap();

        let cipher = XChaCha20Poly1305::new((&key).into());
        let ciphertext = cipher.encrypt(&nonce.into(), plaintext).unwrap();

        let mut file_data = Vec::new();
        file_data.extend_from_slice(HEADER_MAGIC);
        file_data.push(SALT_LEN as u8);
        file_data.push(NONCE_LEN as u8);
        file_data.extend_from_slice(&salt);
        file_data.extend_from_slice(&nonce);
        file_data.extend_from_slice(&ciphertext);

        file_data
    }

    #[test]
    fn test_decrypt_file_valid() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("encrypted.enc");
        let output_path = dir.path().join("decrypted.txt");

        let password = "correct_password";
        let plaintext_msg = b"hello glacier world";

        fs::write(&input_path, build_encrypted_file(password, plaintext_msg)).unwrap();

        let result = decrypt_file(&input_path, &output_path, password);
        assert!(result.is_ok());
        assert_eq!(fs::read(&output_path).unwrap(), plaintext_msg);
    }

    #[test]
    fn test_decrypt_file_wrong_password() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("encrypted.enc");
        let output_path = dir.path().join("decrypted.txt");

        let password = "correct_password";
        let plaintext_msg = b"secret payload";

        fs::write(&input_path, build_encrypted_file(password, plaintext_msg)).unwrap();

        let result = decrypt_file(&input_path, &output_path, "wrong_password");
        assert!(result.is_err());
        assert!(!output_path.exists());
    }

    #[test]
    fn test_decrypt_file_nonexistent_input() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("missing.enc");
        let output_path = dir.path().join("decrypted.txt");

        let result = decrypt_file(&input_path, &output_path, "password");
        assert!(result.is_err());
        assert!(!output_path.exists());
    }
}
