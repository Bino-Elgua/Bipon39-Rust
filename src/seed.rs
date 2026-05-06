use pbkdf2::pbkdf2_hmac;
use sha2::Sha512;
use unicode_normalization::UnicodeNormalization;
use zeroize::Zeroizing;

use crate::constants::{PBKDF2_ITERATIONS, PBKDF2_OUTPUT_BYTES, PBKDF2_PASSPHRASE_PREFIX, PBKDF2_SALT_BASE};
use crate::error::BiponError;

/// Derive a 64-byte seed from a mnemonic and optional passphrase.
pub fn mnemonic_to_seed(words: &[&str], passphrase: &str) -> Result<Zeroizing<Vec<u8>>, BiponError> {
    let mnemonic = words.join(" ").nfkd().collect::<String>();
    let passphrase_n = passphrase.nfkd().collect::<String>();
    let salt = if passphrase_n.is_empty() {
        PBKDF2_SALT_BASE.to_string()
    } else {
        format!("{PBKDF2_SALT_BASE}{PBKDF2_PASSPHRASE_PREFIX}{passphrase_n}")
    }
    .nfkd()
    .collect::<String>();

    let mut output = vec![0u8; PBKDF2_OUTPUT_BYTES];
    pbkdf2_hmac::<Sha512>(
        mnemonic.as_bytes(),
        salt.as_bytes(),
        PBKDF2_ITERATIONS,
        &mut output,
    );
    Ok(Zeroizing::new(output))
}
