use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BiponError {
    #[error("Wordlist integrity failure: {0}")]
    WordlistIntegrity(String),

    #[error("Token not found: {token:?}")]
    TokenNotFound { token: String },

    #[error("Index out of range: {index} (valid: 1–256)")]
    IndexOutOfRange { index: usize },

    #[error("Invalid mnemonic length: {words} words (expected one of: 17, 21, 25, 29, 33)")]
    InvalidMnemonicLength { words: usize },

    #[error("Invalid word at position {position}: {word:?}")]
    InvalidWord { position: usize, word: String },

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Invalid entropy length: {bits} bits (valid: 128, 160, 192, 224, 256)")]
    InvalidEntropyLength { bits: usize },

    #[error("PBKDF2 derivation error: {0}")]
    DerivationError(String),

    #[error("Merkle root mismatch: computed {computed}, expected {expected}")]
    MerkleRootMismatch { computed: String, expected: String },

    #[error("No canonical form for encoding token: {0}")]
    CanonicalNotFound(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(String),
}
