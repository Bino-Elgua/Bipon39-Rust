use crate::error::BiponError;
use crate::wordlist::{entry_by_canonical, entry_by_encoding};

/// Given an encoding token, return its canonical counterpart.
pub fn canonical_for_encoding(token: &str) -> Result<&'static str, BiponError> {
    Ok(entry_by_encoding(token)?.canonical)
}

/// Given a canonical token, return its encoding counterpart.
pub fn encoding_for_canonical(token: &str) -> Result<&'static str, BiponError> {
    Ok(entry_by_canonical(token)?.encoding)
}

/// Convert encoding tokens to canonical forms.
pub fn mnemonic_to_canonical(words: &[&str]) -> Result<Vec<&'static str>, BiponError> {
    words.iter().map(|word| canonical_for_encoding(word)).collect()
}

/// Convert canonical tokens to encoding forms.
pub fn canonical_to_encoding(words: &[&str]) -> Result<Vec<&'static str>, BiponError> {
    words.iter().map(|word| encoding_for_canonical(word)).collect()
}

/// Format a mnemonic as a numbered list.
pub fn format_numbered(words: &[&str]) -> String {
    words
        .iter()
        .enumerate()
        .map(|(index, word)| format!("{}. {}", index + 1, word))
        .collect::<Vec<_>>()
        .join("  ")
}

/// Format canonical forms as a numbered list.
pub fn format_numbered_canonical(words: &[&str]) -> Result<String, BiponError> {
    let canonical = mnemonic_to_canonical(words)?;
    Ok(format_numbered(&canonical))
}
