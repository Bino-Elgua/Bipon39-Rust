/// Total number of tokens in the wordlist.
pub const WORDLIST_SIZE: usize = 256;

/// Each word encodes exactly 8 bits of entropy.
pub const BITS_PER_WORD: usize = 8;

/// Maps entropy length in bits to (entropy_bits, word_count, checksum_bits, pad_bits).
pub const ENTROPY_WORD_TABLE: &[(usize, usize, usize, usize)] = &[
    (128, 17, 4, 4),
    (160, 21, 5, 3),
    (192, 25, 6, 2),
    (224, 29, 7, 1),
    (256, 33, 8, 0),
];

/// PBKDF2-HMAC-SHA512 iteration count.
pub const PBKDF2_ITERATIONS: u32 = 2048;

/// PBKDF2 output length in bytes (512-bit seed).
pub const PBKDF2_OUTPUT_BYTES: usize = 64;

/// Base salt string for seed derivation (no passphrase case).
///
/// Stored in NFKD form (`Ọ̀` = `O` + dot-below + grave) and normalized again
/// at runtime before PBKDF2 to keep cross-platform behavior deterministic.
pub const PBKDF2_SALT_BASE: &str = "BIPO\u{0323}\u{0300}N39 seed";

/// Separator inserted between the base salt and the passphrase.
///
/// Stored in NFKD form (`Ọ̀RÍ` = `O` + dot-below + grave, `I` + acute).
pub const PBKDF2_PASSPHRASE_PREFIX: &str = " O\u{0323}\u{0300}RI\u{0301}:";

/// HMAC-SHA512 key string for native BIPỌ̀N39 master key derivation.
pub const MASTER_KEY_NATIVE: &str = "BIPỌ̀N39 master";

/// HMAC-SHA512 key string for BIP-32 compatible master key derivation.
pub const MASTER_KEY_BIP32: &str = "Bitcoin seed";

/// SHA-256 Merkle root computed over all 256 encoding tokens in flat_index order.
pub const MERKLE_ROOT: &str = "fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14";
