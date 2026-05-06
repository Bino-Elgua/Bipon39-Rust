use zeroize::Zeroizing;

use crate::constants::ENTROPY_WORD_TABLE;
use crate::crypto::{ct_eq, sha256};
use crate::error::BiponError;
use crate::wordlist::{entry_by_index, index_of_encoding};

/// Convert raw entropy bytes to a mnemonic phrase.
pub fn entropy_to_mnemonic(entropy: &[u8]) -> Result<Vec<String>, BiponError> {
    let entropy_bits = entropy.len() * 8;
    let (_, word_count, checksum_bits, pad_bits) = params_for_entropy_bits(entropy_bits)?;

    let hash = sha256(entropy);
    let mut bits = Vec::with_capacity(word_count * 8);
    for byte in entropy {
        append_byte_bits(&mut bits, *byte);
    }
    for bit in 0..checksum_bits {
        bits.push(((hash[0] >> (7 - bit)) & 1) == 1);
    }
    bits.extend(std::iter::repeat_n(false, pad_bits));

    let mut words = Vec::with_capacity(word_count);
    for chunk in bits.chunks_exact(8) {
        let mut value = 0u8;
        for bit in chunk {
            value = (value << 1) | u8::from(*bit);
        }
        words.push(entry_by_index(value as usize + 1)?.encoding.to_string());
    }
    Ok(words)
}

/// Convert mnemonic words back to entropy bytes.
pub fn mnemonic_to_entropy(words: &[&str]) -> Result<Zeroizing<Vec<u8>>, BiponError> {
    let (entropy_bits, _, checksum_bits, _) = params_for_word_count(words.len())?;
    let mut bits = Vec::with_capacity(words.len() * 8);

    for (position, word) in words.iter().enumerate() {
        let index = index_of_encoding(word).map_err(|_| BiponError::InvalidWord {
            position,
            word: (*word).to_string(),
        })?;
        append_byte_bits(&mut bits, index as u8);
    }

    let mut entropy = Vec::with_capacity(entropy_bits / 8);
    for chunk in bits[..entropy_bits].chunks_exact(8) {
        let mut value = 0u8;
        for bit in chunk {
            value = (value << 1) | u8::from(*bit);
        }
        entropy.push(value);
    }

    let checksum_start = entropy_bits;
    let checksum_end = checksum_start + checksum_bits;
    let mut stream_checksum = 0u8;
    for bit in &bits[checksum_start..checksum_end] {
        stream_checksum = (stream_checksum << 1) | u8::from(*bit);
    }
    stream_checksum <<= 8 - checksum_bits;

    let hash = sha256(&entropy);
    let expected = hash[0] & (0xFFu8 << (8 - checksum_bits));
    if !ct_eq(&[stream_checksum], &[expected]) {
        return Err(BiponError::ChecksumMismatch);
    }

    Ok(Zeroizing::new(entropy))
}

/// Validate a mnemonic phrase without returning entropy.
pub fn validate_mnemonic(words: &[&str]) -> Result<(), BiponError> {
    mnemonic_to_entropy(words).map(|_| ())
}

/// Split a mnemonic phrase string on whitespace.
pub fn split_mnemonic(phrase: &str) -> Vec<&str> {
    phrase.split_whitespace().collect()
}

/// Join token strings with a single space.
pub fn join_mnemonic(words: &[String]) -> String {
    words.join(" ")
}

fn params_for_entropy_bits(bits: usize) -> Result<(usize, usize, usize, usize), BiponError> {
    ENTROPY_WORD_TABLE
        .iter()
        .copied()
        .find(|(entropy_bits, _, _, _)| *entropy_bits == bits)
        .ok_or(BiponError::InvalidEntropyLength { bits })
}

fn params_for_word_count(words: usize) -> Result<(usize, usize, usize, usize), BiponError> {
    ENTROPY_WORD_TABLE
        .iter()
        .copied()
        .find(|(_, word_count, _, _)| *word_count == words)
        .ok_or(BiponError::InvalidMnemonicLength { words })
}

fn append_byte_bits(bits: &mut Vec<bool>, byte: u8) {
    for bit in (0..8).rev() {
        bits.push(((byte >> bit) & 1) == 1);
    }
}
