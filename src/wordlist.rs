use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::constants::{MERKLE_ROOT, WORDLIST_SIZE};
use crate::error::BiponError;

static WORDLIST_JSON: &str = include_str!("../data/canonical.json");

/// A single entry in the BIPỌ̀N39 wordlist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordlistEntry {
    /// 1-based position in the flat_index (1–256).
    pub flat_index: usize,
    /// 0-based array index (flat_index − 1).
    pub array_index: usize,
    /// Macro display name.
    pub macro_name: &'static str,
    /// 1-based position within its Macro.
    pub macro_local_index: usize,
    /// Yorùbá display token.
    pub canonical: &'static str,
    /// ASCII cryptographic token.
    pub encoding: &'static str,
    /// Ritual and elemental metadata loaded from canonical.json.
    pub meta: TokenMeta,
}

/// Ritual and elemental metadata for a single BIPỌ̀N39 token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenMeta {
    /// 0-based token id matching the byte-oriented array index.
    pub id: usize,
    /// ASCII encoding token this metadata describes.
    pub word: String,
    /// Root Orisha/Macro key in ASCII form.
    pub root: String,
    /// TypeScript-compatible ritual affix/subtone family.
    pub affix: String,
    /// Elemental association, e.g. "Fire", "Water", or "Ether".
    pub element: String,
    /// Suggested ritual cue, e.g. "face sunrise" or "beat dundun".
    pub ritual_cue: String,
    /// Ethical or semantic tag, e.g. "truth" or "judgment".
    pub ethical_tag: String,
    /// Deterministic sigil seed phrase, e.g. "east-ray".
    pub sigil_seed: String,
}

#[derive(Deserialize)]
struct CanonicalJson {
    flat_index: Vec<JsonEntry>,
}

#[derive(Deserialize)]
struct JsonEntry {
    index: usize,
    #[serde(rename = "macro")]
    macro_name: String,
    macro_local: usize,
    canonical: String,
    encoding: String,
    token_meta: JsonTokenMeta,
}

#[derive(Deserialize)]
struct JsonTokenMeta {
    id: usize,
    word: String,
    root: String,
    affix: String,
    element: String,
    ritual_cue: String,
    ethical_tag: String,
    sigil_seed: String,
}

static WORDLIST: Lazy<Vec<WordlistEntry>> = Lazy::new(|| {
    let parsed: CanonicalJson = serde_json::from_str(WORDLIST_JSON)
        .expect("embedded canonical.json must parse successfully");

    parsed
        .flat_index
        .into_iter()
        .map(|entry| WordlistEntry {
            flat_index: entry.index,
            array_index: entry.index.saturating_sub(1),
            macro_name: Box::leak(entry.macro_name.into_boxed_str()),
            macro_local_index: entry.macro_local,
            canonical: Box::leak(entry.canonical.into_boxed_str()),
            encoding: Box::leak(entry.encoding.into_boxed_str()),
            meta: TokenMeta {
                id: entry.token_meta.id,
                word: entry.token_meta.word,
                root: entry.token_meta.root,
                affix: entry.token_meta.affix,
                element: entry.token_meta.element,
                ritual_cue: entry.token_meta.ritual_cue,
                ethical_tag: entry.token_meta.ethical_tag,
                sigil_seed: entry.token_meta.sigil_seed,
            },
        })
        .collect()
});

static ENCODING_INDEX: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    WORDLIST
        .iter()
        .map(|entry| (entry.encoding, entry.array_index))
        .collect()
});

static ENCODING_TOKENS: Lazy<Vec<&'static str>> =
    Lazy::new(|| WORDLIST.iter().map(|entry| entry.encoding).collect());

/// Verify all structural invariants of the loaded wordlist.
pub fn verify_wordlist_integrity() -> Result<(), BiponError> {
    if WORDLIST.len() != WORDLIST_SIZE {
        return Err(BiponError::WordlistIntegrity(format!(
            "expected {WORDLIST_SIZE} entries, loaded {}",
            WORDLIST.len()
        )));
    }

    let expected_counts = [
        ("ÈṢÙ", 88usize),
        ("ṢÀNGÓ", 20),
        ("Ọ̀ṢUN", 28),
        ("YEMỌJA", 28),
        ("ỌYA", 32),
        ("ÒGÚN", 32),
        ("ỌBÀTÁLÁ", 28),
    ];
    for (macro_name, expected) in expected_counts {
        let actual = WORDLIST
            .iter()
            .filter(|entry| entry.macro_name == macro_name)
            .count();
        if actual != expected {
            return Err(BiponError::WordlistIntegrity(format!(
                "macro {macro_name} has {actual} entries, expected {expected}"
            )));
        }
    }

    let mut encodings = HashSet::with_capacity(WORDLIST_SIZE);
    let mut canonicals = HashSet::with_capacity(WORDLIST_SIZE);
    for (array_index, entry) in WORDLIST.iter().enumerate() {
        let expected_flat = array_index + 1;
        if entry.flat_index != expected_flat {
            return Err(BiponError::WordlistIntegrity(format!(
                "flat_index sequence mismatch at array index {array_index}: found {}, expected {expected_flat}",
                entry.flat_index
            )));
        }
        if entry.array_index != array_index {
            return Err(BiponError::WordlistIntegrity(format!(
                "array_index mismatch for flat_index {}: found {}, expected {array_index}",
                entry.flat_index, entry.array_index
            )));
        }
        if !encodings.insert(entry.encoding) {
            return Err(BiponError::WordlistIntegrity(format!(
                "duplicate encoding token {}",
                entry.encoding
            )));
        }
        if !canonicals.insert(entry.canonical) {
            return Err(BiponError::WordlistIntegrity(format!(
                "duplicate canonical token {}",
                entry.canonical
            )));
        }
        if !encoding_token_is_valid(entry.encoding) {
            return Err(BiponError::WordlistIntegrity(format!(
                "encoding token {} at flat_index {} violates ^[a-z][a-z0-9\\-]+$",
                entry.encoding, entry.flat_index
            )));
        }
    }

    if !MERKLE_ROOT.is_empty() {
        let computed = crate::crypto::compute_wordlist_merkle_root();
        if computed != MERKLE_ROOT {
            return Err(BiponError::MerkleRootMismatch {
                computed,
                expected: MERKLE_ROOT.to_string(),
            });
        }
    }

    Ok(())
}

fn encoding_token_is_valid(token: &str) -> bool {
    let mut chars = token.chars();
    matches!(chars.next(), Some('a'..='z'))
        && chars.all(|ch| matches!(ch, 'a'..='z' | '0'..='9' | '-'))
        && token.len() >= 2
}

/// Look up an entry by 1-based flat_index.
pub fn entry_by_index(flat_index: usize) -> Result<&'static WordlistEntry, BiponError> {
    if !(1..=WORDLIST_SIZE).contains(&flat_index) {
        return Err(BiponError::IndexOutOfRange { index: flat_index });
    }
    Ok(&WORDLIST[flat_index - 1])
}

/// Look up an entry by encoding token.
pub fn entry_by_encoding(token: &str) -> Result<&'static WordlistEntry, BiponError> {
    let index = index_of_encoding(token)?;
    Ok(&WORDLIST[index])
}

/// Look up an entry by canonical token.
pub fn entry_by_canonical(token: &str) -> Result<&'static WordlistEntry, BiponError> {
    WORDLIST
        .iter()
        .find(|entry| entry.canonical == token)
        .ok_or_else(|| BiponError::CanonicalNotFound(token.to_string()))
}

/// Return all entries belonging to a named Macro.
pub fn entries_for_macro(macro_name: &str) -> Vec<&'static WordlistEntry> {
    WORDLIST
        .iter()
        .filter(|entry| entry.macro_name == macro_name)
        .collect()
}

/// Return the 0-based array_index for an encoding token.
pub fn index_of_encoding(token: &str) -> Result<usize, BiponError> {
    ENCODING_INDEX
        .get(token)
        .copied()
        .ok_or_else(|| BiponError::TokenNotFound {
            token: token.to_string(),
        })
}

/// Look up token metadata by 0-based array index.
pub fn lookup_meta(index: usize) -> Option<TokenMeta> {
    WORDLIST.get(index).map(|entry| entry.meta.clone())
}

/// Return the full ordered slice of encoding tokens.
pub fn all_encoding_tokens() -> &'static [&'static str] {
    ENCODING_TOKENS.as_slice()
}
