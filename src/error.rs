use thiserror::Error;

/// Error type returned by fallible BIPỌ̀N39 operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BiponError {
    /// The embedded wordlist failed a structural integrity invariant.
    #[error("Wordlist integrity failure: {0}")]
    WordlistIntegrity(String),

    /// An encoding token was not found in the wordlist.
    #[error("Token not found: {token:?}")]
    TokenNotFound { token: String },

    /// A 1-based flat index was outside the valid 1–256 range.
    #[error("Index out of range: {index} (valid: 1–256)")]
    IndexOutOfRange { index: usize },

    /// A mnemonic had a word count that cannot map to a supported entropy size.
    #[error("Invalid mnemonic length: {words} words (expected one of: 17, 21, 25, 29, 33)")]
    InvalidMnemonicLength { words: usize },

    /// A mnemonic word at the given zero-based position was not a valid encoding token.
    #[error("Invalid word at position {position}: {word:?}")]
    InvalidWord { position: usize, word: String },

    /// The checksum bits embedded in a mnemonic did not match the decoded entropy.
    #[error("Checksum mismatch")]
    ChecksumMismatch,

    /// Entropy length was not one of 128, 160, 192, 224, or 256 bits.
    #[error("Invalid entropy length: {bits} bits (valid: 128, 160, 192, 224, 256)")]
    InvalidEntropyLength { bits: usize },

    /// Seed or master-key derivation failed.
    #[error("PBKDF2 derivation error: {0}")]
    DerivationError(String),

    /// The computed wordlist Merkle root differed from the pinned root.
    #[error("Merkle root mismatch: computed {computed}, expected {expected}")]
    MerkleRootMismatch { computed: String, expected: String },

    /// A canonical display token could not be found for a supplied token.
    #[error("No canonical form for encoding token: {0}")]
    CanonicalNotFound(String),

    /// The embedded JSON wordlist could not be parsed.
    #[error("JSON parse error: {0}")]
    JsonParseError(String),
}
