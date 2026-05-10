use zeroize::Zeroizing;

use crate::crypto::{ct_eq, sha256};
use crate::error::BiponError;
use crate::mnemonic::{entropy_to_mnemonic, mnemonic_to_entropy};
use crate::wordlist::{entry_by_index, index_of_encoding, WordlistEntry};

/// TypeScript-compatible 2048-mode subtones.
pub const SUBTONES: [&str; 8] = [
    "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
];

/// Convert a valid 256-mode mnemonic phrase into 2048-mode tokens.
pub fn encode_2048(mnemonic_256: &str) -> Result<String, BiponError> {
    let words = mnemonic_256.split_whitespace().collect::<Vec<_>>();
    let entropy = mnemonic_to_entropy(&words)?;
    Ok(entropy_to_mnemonic_2048(&entropy)?.join(" "))
}

/// Convert a valid 2048-mode mnemonic phrase back into 256-mode tokens.
pub fn decode_2048(mnemonic_2048: &str) -> Result<String, BiponError> {
    let words = mnemonic_2048.split_whitespace().collect::<Vec<_>>();
    let entropy = mnemonic_2048_to_entropy(&words)?;
    Ok(entropy_to_mnemonic(&entropy)?.join(" "))
}

/// Convert entropy bytes to TypeScript-compatible 2048-mode tokens.
pub fn entropy_to_mnemonic_2048(entropy: &[u8]) -> Result<Vec<String>, BiponError> {
    let entropy_bits = entropy.len() * 8;
    let (_, word_count, checksum_bits, pad_bits) = params_for_entropy_bits_2048(entropy_bits)?;

    let hash = sha256(entropy);
    let mut bits = Vec::with_capacity(word_count * 11);
    for byte in entropy {
        append_byte_bits(&mut bits, *byte);
    }
    for bit in 0..checksum_bits {
        bits.push(((hash[0] >> (7 - bit)) & 1) == 1);
    }
    bits.extend(std::iter::repeat_n(false, pad_bits));

    let mut words = Vec::with_capacity(word_count);
    for chunk in bits.chunks_exact(11) {
        let mut value = 0usize;
        for bit in chunk {
            value = (value << 1) | usize::from(*bit);
        }
        words.push(token_2048(value)?);
    }
    Ok(words)
}

/// Convert 2048-mode tokens back to entropy bytes.
pub fn mnemonic_2048_to_entropy(words: &[&str]) -> Result<Zeroizing<Vec<u8>>, BiponError> {
    let (entropy_bits, _, checksum_bits, _) = params_for_word_count_2048(words.len())?;
    let mut bits = Vec::with_capacity(words.len() * 11);

    for (position, word) in words.iter().enumerate() {
        let index = index_of_2048_token(word).map_err(|_| BiponError::InvalidWord {
            position,
            word: (*word).to_string(),
        })?;
        append_11_bits(&mut bits, index);
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

/// Resolve either a 256-mode token or a 2048-mode token to its base wordlist entry.
pub fn entry_by_mode_token(token: &str) -> Result<&'static WordlistEntry, BiponError> {
    match token.split_once('~') {
        Some((base, subtone)) => {
            validate_subtone(subtone)?;
            let index = index_of_encoding(base)?;
            Ok(entry_by_index(index + 1)?)
        }
        None => crate::wordlist::entry_by_encoding(token),
    }
}

/// Return the mode-specific index for a 256- or 2048-mode token.
pub fn mode_index_of_token(token: &str) -> Result<usize, BiponError> {
    match token.split_once('~') {
        Some((base, subtone)) => {
            let base_index = index_of_encoding(base)?;
            let subtone_index = validate_subtone(subtone)?;
            Ok((base_index << 3) | subtone_index)
        }
        None => index_of_encoding(token),
    }
}

/// Return true when a token is in 2048-mode `base~subtone` form.
pub fn is_2048_token(token: &str) -> bool {
    token.split_once('~').is_some()
}

fn token_2048(index: usize) -> Result<String, BiponError> {
    if index >= 2048 {
        return Err(BiponError::IndexOutOfRange { index });
    }
    let base = entry_by_index((index >> 3) + 1)?.encoding;
    let subtone = SUBTONES[index & 0b111];
    Ok(format!("{base}~{subtone}"))
}

fn index_of_2048_token(token: &str) -> Result<usize, BiponError> {
    match token.split_once('~') {
        Some((base, subtone)) => {
            let base_index = index_of_encoding(base)?;
            let subtone_index = validate_subtone(subtone)?;
            Ok((base_index << 3) | subtone_index)
        }
        None => Err(BiponError::TokenNotFound {
            token: token.to_string(),
        }),
    }
}

fn validate_subtone(subtone: &str) -> Result<usize, BiponError> {
    SUBTONES
        .iter()
        .position(|candidate| *candidate == subtone)
        .ok_or_else(|| BiponError::TokenNotFound {
            token: subtone.to_string(),
        })
}

fn params_for_entropy_bits_2048(bits: usize) -> Result<(usize, usize, usize, usize), BiponError> {
    match bits {
        128 => Ok((128, 12, 4, 0)),
        160 => Ok((160, 15, 5, 0)),
        192 => Ok((192, 18, 6, 0)),
        224 => Ok((224, 21, 7, 0)),
        256 => Ok((256, 24, 8, 0)),
        _ => Err(BiponError::InvalidEntropyLength { bits }),
    }
}

fn params_for_word_count_2048(words: usize) -> Result<(usize, usize, usize, usize), BiponError> {
    match words {
        12 => Ok((128, 12, 4, 0)),
        15 => Ok((160, 15, 5, 0)),
        18 => Ok((192, 18, 6, 0)),
        21 => Ok((224, 21, 7, 0)),
        24 => Ok((256, 24, 8, 0)),
        _ => Err(BiponError::InvalidMnemonic2048Length { words }),
    }
}

fn append_byte_bits(bits: &mut Vec<bool>, byte: u8) {
    for bit in (0..8).rev() {
        bits.push(((byte >> bit) & 1) == 1);
    }
}

fn append_11_bits(bits: &mut Vec<bool>, value: usize) {
    for bit in (0..11).rev() {
        bits.push(((value >> bit) & 1) == 1);
    }
}
