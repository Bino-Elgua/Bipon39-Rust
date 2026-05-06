use zeroize::ZeroizeOnDrop;

use crate::constants::{MASTER_KEY_BIP32, MASTER_KEY_NATIVE};
use crate::crypto::hmac_sha512;
use crate::error::BiponError;

/// Master key derivation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivationMode {
    /// Native BIPỌ̀N39 derivation.
    Native,
    /// BIP-32 compatible derivation.
    Bip32,
}

impl DerivationMode {
    fn key_string(self) -> &'static str {
        match self {
            DerivationMode::Native => MASTER_KEY_NATIVE,
            DerivationMode::Bip32 => MASTER_KEY_BIP32,
        }
    }
}

/// Master private key and chain code derived from a 64-byte seed.
#[derive(ZeroizeOnDrop)]
pub struct MasterKey {
    /// Private key material (IL).
    pub key: [u8; 32],
    /// Chain code material (IR).
    pub chain_code: [u8; 32],
}

impl MasterKey {
    /// Hex-encode the private key.
    pub fn key_hex(&self) -> String {
        hex::encode(self.key)
    }

    /// Hex-encode the chain code.
    pub fn chain_code_hex(&self) -> String {
        hex::encode(self.chain_code)
    }
}

/// Derive a master key from a 64-byte seed.
pub fn master_from_seed(seed: &[u8], mode: DerivationMode) -> Result<MasterKey, BiponError> {
    if seed.len() != 64 {
        return Err(BiponError::DerivationError(format!(
            "seed must be 64 bytes, got {}",
            seed.len()
        )));
    }
    let digest = hmac_sha512(mode.key_string().as_bytes(), seed);
    let mut key = [0u8; 32];
    let mut chain_code = [0u8; 32];
    key.copy_from_slice(&digest[..32]);
    chain_code.copy_from_slice(&digest[32..]);
    Ok(MasterKey { key, chain_code })
}
