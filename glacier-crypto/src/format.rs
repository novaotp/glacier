pub const HEADER_MAGIC: &[u8; 8] = b"GLACIER1";
pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 24;
pub const KEY_LEN: usize = 32;

pub const HEADER_PREFIX_LEN: usize = 10;
pub const HEADER_LEN: usize = HEADER_PREFIX_LEN + SALT_LEN + NONCE_LEN;

/// Parsed `GLACIER1` file header and encrypted payload.
#[derive(Debug)]
pub struct ParsedHeader<'a> {
    pub salt: &'a [u8],
    pub nonce: &'a [u8],
    pub ciphertext: &'a [u8],
}

/// Parses a `GLACIER1`-encrypted file.
///
/// Layout: `HEADER_MAGIC | salt_len | nonce_len | salt | nonce | ciphertext`.
///
/// # Errors
///
/// Returns an error if:
/// * `data` is smaller than `HEADER_PREFIX_LEN` (10 bytes).
/// * The first 8 bytes do not match `HEADER_MAGIC` (`GLACIER1`).
/// * The encoded salt length or nonce length does not match `SALT_LEN` or `NONCE_LEN`.
/// * `data` is shorter than the complete `HEADER_LEN` (due to a truncated file).
pub fn parse_header(data: &[u8]) -> anyhow::Result<ParsedHeader<'_>> {
    if data.len() < HEADER_PREFIX_LEN {
        anyhow::bail!("file too small");
    }

    if &data[0..8] != HEADER_MAGIC {
        anyhow::bail!("invalid file format");
    }

    if data[8] as usize != SALT_LEN || data[9] as usize != NONCE_LEN {
        anyhow::bail!("invalid salt or nonce length");
    }

    if data.len() < HEADER_LEN {
        anyhow::bail!("truncated file");
    }

    Ok(ParsedHeader {
        salt: &data[HEADER_PREFIX_LEN..HEADER_PREFIX_LEN + SALT_LEN],
        nonce: &data[HEADER_PREFIX_LEN + SALT_LEN..HEADER_LEN],
        ciphertext: &data[HEADER_LEN..],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a valid baseline byte [Vec] for testing.
    fn create_valid_data() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(HEADER_MAGIC);
        data.push(SALT_LEN as u8);
        data.push(NONCE_LEN as u8);

        // dummy salt
        data.extend_from_slice(&[0xAA; SALT_LEN]);
        // dummy nonce
        data.extend_from_slice(&[0xBB; NONCE_LEN]);

        data.extend_from_slice(b"secret ciphertext data");
        data
    }

    #[test]
    fn test_parse_header_valid() {
        let data = create_valid_data();
        let result = parse_header(&data);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.salt.len(), SALT_LEN);
        assert_eq!(parsed.nonce.len(), NONCE_LEN);
        assert_eq!(parsed.ciphertext, b"secret ciphertext data");
    }

    #[test]
    fn test_parse_header_data_too_small() {
        // Less than HEADER_PREFIX_LEN (10 bytes)
        let data = b"test".to_vec();
        let result = parse_header(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "file too small");
    }

    #[test]
    fn test_parse_header_magic_mismatch() {
        let mut data = create_valid_data();
        // Wrong magic header
        data[0..8].copy_from_slice(b"BADMAGIC");

        let result = parse_header(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "invalid file format");
    }

    #[test]
    fn test_parse_header_salt_len_different() {
        let mut data = create_valid_data();
        // Wrong salt length
        data[8] = (SALT_LEN + 1) as u8;

        let result = parse_header(&data);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid salt or nonce length"
        );
    }

    #[test]
    fn test_parse_header_nonce_len_different() {
        let mut data = create_valid_data();
        // Wrong nonce length
        data[9] = (NONCE_LEN + 1) as u8;

        let result = parse_header(&data);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid salt or nonce length"
        );
    }

    #[test]
    fn test_parse_header_total_header_length_truncated() {
        let mut data = create_valid_data();
        // Shortens the data so it's >= HEADER_PREFIX_LEN but < HEADER_LEN
        data.truncate(HEADER_PREFIX_LEN + 5);

        let result = parse_header(&data);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "truncated file");
    }

    #[test]
    fn test_parse_header_empty_ciphertext_valid() {
        // Valid header and lengths, but zero ciphertext bytes following the header
        let mut data = Vec::new();
        data.extend_from_slice(HEADER_MAGIC);
        data.push(SALT_LEN as u8);
        data.push(NONCE_LEN as u8);
        data.extend_from_slice(&[0x11; SALT_LEN]);
        data.extend_from_slice(&[0x22; NONCE_LEN]);

        let result = parse_header(&data);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(parsed.ciphertext.is_empty());
    }
}
