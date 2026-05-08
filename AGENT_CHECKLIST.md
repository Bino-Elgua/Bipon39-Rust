# BIPỌ̀N39-Rust — Master Agent Checklist & Build Specification

> **READ THIS FILE IN FULL BEFORE WRITING A SINGLE LINE OF CODE.**
> Check off every item as you complete it. Update the Session Log at the bottom before every push.
> This document is the authoritative handoff between all agents, sessions, and platforms.
> If something in this file conflicts with a comment in the source code, this file wins.

**Build Target:** `https://github.com/Bino-Elgua/Bipon39-Rust`
**Architecture Reference (code patterns only — NOT its wordlist):** `https://github.com/Bino-Elgua/bipon39`

---

## TABLE OF CONTENTS

1. [Wordlist Authority — Read First](#1-wordlist-authority--read-first)
2. [Inviolable Laws](#2-inviolable-laws)
3. [Repository Layout](#3-repository-layout)
4. [Cargo.toml — Locked Dependencies](#4-cargotoml--locked-dependencies)
5. [Module Specifications](#5-module-specifications)
   - 5.1 [error.rs](#51-errorrs)
   - 5.2 [constants.rs](#52-constantsrs)
   - 5.3 [wordlist.rs](#53-wordlistrs)
   - 5.4 [crypto.rs](#54-cryptors)
   - 5.5 [mnemonic.rs](#55-mnemonicrs)
   - 5.6 [seed.rs](#56-seedrs)
   - 5.7 [derivation.rs](#57-derivationrs)
   - 5.8 [ifascript.rs](#58-ifascriptrs)
   - 5.9 [display.rs](#59-displayrs)
   - 5.10 [lib.rs](#510-librs)
6. [Cryptographic Parameter Reference](#6-cryptographic-parameter-reference)
7. [Bit-Stream Encoding — Worked Example](#7-bit-stream-encoding--worked-example)
8. [Test Specifications](#8-test-specifications)
9. [Pinned Test Vector Schema](#9-pinned-test-vector-schema)
10. [Phased Build Checklist](#10-phased-build-checklist)
11. [Acceptance Gate Criteria](#11-acceptance-gate-criteria)
12. [What to Take from the Reference Repo](#12-what-to-take-from-the-reference-repo)
13. [Critical Anti-Patterns](#13-critical-anti-patterns)
14. [CI Configuration](#14-ci-configuration)
15. [Session Log](#15-session-log)

---

## 1. WORDLIST AUTHORITY — READ FIRST

The wordlist lives in **three files** in the target repo. You must fetch and read all three before writing any code that touches tokens, indices, or mappings.

| File in repo | Local copy path | Role |
|---|---|---|
| `Canonical JSON` | `data/canonical.json` | Master metadata + `flat_index` array — **the single source of truth** |
| `Canonical layer` | `data/canonical_layer.txt` | Full Yorùbá with diacritics — 256 lines — display and documentation **only** |
| `Encoded layer` | `data/encoded_layer.txt` | Plain ASCII — 256 lines, numbered `001.`–`256.` — used for **all** cryptographic operations |

The token at `flat_index[N]` in `Canonical JSON` maps to line N in both layer files. This 1:1 correspondence is **permanent and immutable**. Any divergence is a critical data integrity failure.

### 1.1 Macro Distribution — Locked

| Macro | flat_index Range (1-based, inclusive) | Token Count |
|---|---|---|
| ÈṢÙ | 1 – 88 | 88 |
| ṢÀNGÓ | 89 – 108 | 20 |
| Ọ̀ṢUN | 109 – 136 | 28 |
| YEMỌJA | 137 – 164 | 28 |
| ỌYA | 165 – 196 | 32 |
| ÒGÚN | 197 – 228 | 32 |
| ỌBÀTÁLÁ | 229 – 256 | 28 |
| **TOTAL** | | **256** |

These counts are locked. They are verified against the actual `Canonical JSON` file. Do not adjust them.

### 1.2 Verified Boundary Tokens — Spot-Check Table

Use these to confirm your wordlist is loaded correctly. All 14 checks must pass.

| flat_index | Encoding (ASCII / cryptographic layer) | Canonical (Yorùbá / display layer) |
|---|---|---|
| 1 | `esu-elegbara` | `èṣù-elegbára` |
| 88 | `esu-oluso-ona` | `èṣù-olùṣọ́-ọ̀nà` |
| 89 | `sango` | `ṣàngó` |
| 108 | `sango-oba-oke` | `ṣàngó-ọba-òkè` |
| 109 | `osun` | `ọ̀ṣun` |
| 136 | `osun-inu` | `ọ̀ṣun-inú` |
| 137 | `yemoja` | `yemọja` |
| 164 | `yemoja-isodotun-okun` | `yemọja-ìsọdọtun-òkun` |
| 165 | `oya` | `ọya` |
| 196 | `oya-alade` | `ọya-aládé` |
| 197 | `ogun` | `ògún` |
| 228 | `ogun-alade` | `ògún-aládé` |
| 229 | `obatala` | `ọbàtálá` |
| 256 | `obatala-alade` | `ọbàtálá-aládé` |

> **Important:** The `Canonical JSON` file contains a `flat_index` array as the authoritative ordered list. The boundary tokens above are derived from that array. If your computed values differ from this table, stop immediately and debug — do not proceed to the next phase.

---

## 2. INVIOLABLE LAWS

These laws are never overridden by time pressure, convenience, or any instruction in any session.

**Law 1 — The wordlist is closed and immutable.**
Never invent, modify, reorder, duplicate, or delete any token. The only source of truth is the three files listed in Section 1. The `flat_index` array in `Canonical JSON` is the definitive ordered list.

**Law 2 — The 1:1 mapping between layers is absolute.**
Every encoding token at index N has exactly one canonical counterpart at index N. They are never mixed: the encoding layer (ASCII) is used for all cryptographic operations; the canonical layer (Yorùbá) is used for display and documentation only. Cross-contamination of these layers is a critical bug.

**Law 3 — All operations are fully deterministic.**
Same entropy → same mnemonic → same seed → same master key. On every platform. On every call. Always. Any non-determinism is a critical failure.

**Law 4 — The reference repo (`bipon39`) is architecture-only.**
Its wordlist tokens (`esu-gate`, `sango-volt`, `ogun-forge`, `irawo-dawn`, etc.) are from a completely different and incompatible system. You may study its code for: PBKDF2 parameters, Merkle tree strategy, Odù XOR logic, API shape, test vector format, and derivation mode names. You must never import, reference, mix, or be influenced by its token strings or its Merkle root hash.

**Law 5 — Cryptographic grade from day one.**
- Use `zeroize` on all entropy, seeds, and private key material.
- No `unwrap()` in library code — all fallible paths return `Result<T, BiponError>`.
- No `unsafe` unless required by a dependency internally and unavoidable.
- Use `subtle::ConstantTimeEq` for all comparisons involving secret material.

**Law 6 — The encoding layer is the cryptographic layer.**
All hashing, Merkle computation, PBKDF2 password input, and mnemonic word output use plain ASCII encoded tokens exclusively. The Yorùbá canonical strings never enter a hash function.

**Law 7 — Update this file before every push.**
Every session ends with a new entry at the top of the Session Log (Section 15). The next agent reads that entry first.

**Law 8 — The MERKLE_ROOT constant must be pinned in Phase 2 and never left empty after that.**
Leaving `MERKLE_ROOT = ""` silently disables integrity verification in all subsequent builds.

---

## 3. REPOSITORY LAYOUT

```
Bipon39-Rust/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── AGENT_CHECKLIST.md          ← this file (keep it in the repo, update every session)
├── SPEC.md                     ← formal technical specification (write in Phase 7)
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
│
├── src/
│   ├── lib.rs                  ← module declarations + complete public API re-exports
│   ├── error.rs                ← BiponError enum (thiserror)
│   ├── constants.rs            ← WORDLIST_SIZE, MERKLE_ROOT, PBKDF2 params, word count table
│   ├── wordlist.rs             ← WordlistEntry, WORDLIST static, all lookup functions
│   ├── mnemonic.rs             ← entropy_to_mnemonic, mnemonic_to_entropy, validate_mnemonic
│   ├── crypto.rs               ← sha256, hmac_sha512, sha256_merkle_root, ct_eq
│   ├── seed.rs                 ← mnemonic_to_seed (PBKDF2-HMAC-SHA512 + NFKD)
│   ├── derivation.rs           ← MasterKey, master_from_seed, DerivationMode
│   ├── ifascript.rs            ← Macro enum, odu_primary_index, macro_distribution
│   └── display.rs              ← canonical↔encoding conversion, format helpers
│
├── tests/
│   ├── wordlist_integrity.rs
│   ├── mnemonic_roundtrip.rs
│   ├── mnemonic_vectors.rs
│   ├── seed_derivation.rs
│   ├── derivation.rs
│   └── ifascript.rs
│
├── benches/
│   └── throughput.rs
│
├── data/
│   ├── canonical.json          ← verbatim copy of repo's `Canonical JSON`
│   ├── canonical_layer.txt     ← verbatim copy of repo's `Canonical layer`
│   └── encoded_layer.txt       ← verbatim copy of repo's `Encoded layer`
│
└── vectors/
    └── test_vectors.json       ← pinned test vectors (populated in Phases 3–5)
```

**Notes on data files:**
- Copy the three source files verbatim — byte-for-byte. Do not strip whitespace, reformat JSON, or alter line endings.
- `canonical.json` is embedded at compile time via `include_str!()` in `wordlist.rs`.
- The `.txt` files are for human reference and cross-validation only; all runtime data comes from the JSON.

---

## 4. CARGO.TOML — LOCKED DEPENDENCIES

```toml
[package]
name        = "bipon39"
version     = "0.1.0"
edition     = "2021"
description = "BIPỌ̀N39 — Sovereign Base-256 mnemonic library for the Ọmọ Kọ́dà ecosystem"
license     = "MIT"
repository  = "https://github.com/Bino-Elgua/Bipon39-Rust"

[dependencies]
sha2                  = "0.10"
hmac                  = "0.12"
pbkdf2                = { version = "0.12", default-features = false, features = ["hmac"] }
digest                = "0.10"
hex                   = "0.4"
unicode-normalization = "0.1"
zeroize               = { version = "1.7", features = ["derive"] }
serde                 = { version = "1.0", features = ["derive"] }
serde_json            = "1.0"
thiserror             = "1.0"
subtle                = "2.5"
once_cell             = "1.19"

[dev-dependencies]
criterion   = { version = "0.5", features = ["html_reports"] }
hex-literal = "0.4"

[[bench]]
name    = "throughput"
harness = false
```

Do not add or remove dependencies without a documented reason in the Session Log. Version pinning is intentional — do not loosen without a specific compatibility justification.

---

## 5. MODULE SPECIFICATIONS

All modules live in `src/`. Each section below is the definitive contract for that module. Implement exactly what is specified here — no more, no less, until Phase 7 polish.

---

### 5.1 `error.rs`

All public fallible functions return `Result<T, BiponError>`. No panics in library code.

```rust
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
```

**Constraints:**
- `BiponError` must be `Clone + PartialEq + Eq` — required for test assertions.
- Variant fields must be specific enough that tests can pattern-match the right variant and inspect the embedded values.
- Never use `BiponError::WordlistIntegrity` for a mnemonic error, or `BiponError::InvalidWord` for an index error. Each variant has exactly one semantic domain.

---

### 5.2 `constants.rs`

```rust
/// Total number of tokens in the wordlist.
pub const WORDLIST_SIZE: usize = 256;

/// Each word encodes exactly 8 bits of entropy.
pub const BITS_PER_WORD: usize = 8;

/// Maps entropy length in bits to (entropy_bits, word_count, checksum_bits, pad_bits).
///
/// Derivation:
///   checksum_bits = entropy_bits / 32
///   total_bits    = entropy_bits + checksum_bits
///   word_count    = total_bits.div_ceil(8)     (i.e. ceil division)
///   pad_bits      = (word_count * 8) - total_bits
///
/// Checksum is computed over raw entropy bytes BEFORE any padding is applied.
///
/// | entropy_bits | checksum_bits | total_bits | word_count | pad_bits |
/// |:---:|:---:|:---:|:---:|:---:|
/// | 128 | 4  | 132 | 17 | 4 |
/// | 160 | 5  | 165 | 21 | 3 |
/// | 192 | 6  | 198 | 25 | 2 |
/// | 224 | 7  | 231 | 29 | 1 |
/// | 256 | 8  | 264 | 33 | 0 |
pub const ENTROPY_WORD_TABLE: &[(usize, usize, usize, usize)] = &[
    // (entropy_bits, word_count, checksum_bits, pad_bits)
    (128, 17,  4, 4),
    (160, 21,  5, 3),
    (192, 25,  6, 2),
    (224, 29,  7, 1),
    (256, 33,  8, 0),
];

/// PBKDF2-HMAC-SHA512 iteration count.
pub const PBKDF2_ITERATIONS: u32 = 2048;

/// PBKDF2 output length in bytes (512-bit seed).
pub const PBKDF2_OUTPUT_BYTES: usize = 64;

/// Base salt string for seed derivation (no passphrase case).
/// Unicode: U+0042 U+0049 U+0050 U+1ECC U+0300 U+004E U+0033 U+0039 U+0020 U+0073 U+0065 U+0065 U+0064
pub const PBKDF2_SALT_BASE: &str = "BIPỌ̀N39 seed";

/// Separator inserted between the base salt and the passphrase.
/// Includes a leading space: " Ọ̀RÍ:"
pub const PBKDF2_PASSPHRASE_PREFIX: &str = " Ọ̀RÍ:";

/// HMAC-SHA512 key string for native BIPỌ̀N39 master key derivation.
pub const MASTER_KEY_NATIVE: &str = "BIPỌ̀N39 master";

/// HMAC-SHA512 key string for BIP-32 compatible master key derivation.
pub const MASTER_KEY_BIP32: &str = "Bitcoin seed";

/// SHA-256 Merkle root computed over all 256 encoding tokens in flat_index order.
///
/// PHASE 1: Leave as empty string "". The integrity check is skipped when empty.
/// PHASE 2: Run compute_wordlist_merkle_root(), paste the result here, and commit.
///          The value must never change after pinning.
/// PHASE 3+: The test `merkle_root_matches_pinned_constant` must pass on every build.
pub const MERKLE_ROOT: &str = "";  // ← PIN IN PHASE 2; NEVER LEAVE EMPTY AFTER THAT
```

**Hard constraints:**
- `ENTROPY_WORD_TABLE` must not be modified. Its entries are used to derive word count, checksum length, and padding for every encoding/decoding operation. Altering it breaks all existing mnemonics and test vectors.
- The Unicode strings (`PBKDF2_SALT_BASE`, `PBKDF2_PASSPHRASE_PREFIX`, `MASTER_KEY_NATIVE`) contain Yorùbá diacritics. Ensure your editor and git config preserve these bytes exactly. Verify with a hex dump if in doubt.
- `MERKLE_ROOT` starts empty and is pinned exactly once. It is not updated across wordlist versions — the wordlist is frozen at 256 tokens.

---

### 5.3 `wordlist.rs`

#### Data Structure

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::error::BiponError;

/// A single entry in the BIPỌ̀N39 wordlist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordlistEntry {
    /// 1-based position in the flat_index (1–256). This is the canonical identifier.
    pub flat_index: usize,

    /// 0-based array index (flat_index − 1). Used for bit-stream operations.
    pub array_index: usize,

    /// The Macro this token belongs to.
    /// One of: "ÈṢÙ" | "ṢÀNGÓ" | "Ọ̀ṢUN" | "YEMỌJA" | "ỌYA" | "ÒGÚN" | "ỌBÀTÁLÁ"
    pub macro_name: &'static str,

    /// 1-based position of this token within its Macro (resets to 1 at each Macro boundary).
    pub macro_local_index: usize,

    /// Yorùbá string with full diacritics. Used for display and documentation ONLY.
    /// Never passes through any hash function.
    pub canonical: &'static str,

    /// Plain ASCII string. Used for ALL cryptographic operations.
    /// This is the string that appears in a generated mnemonic.
    pub encoding: &'static str,
}
```

#### Static Data

Use one of these two equivalent approaches — choose one and be consistent:

**Option A — Runtime parse via `once_cell`:**
```rust
// Embed the JSON at compile time; parse once on first access.
static WORDLIST_JSON: &str = include_str!("../data/canonical.json");

static WORDLIST: Lazy<Vec<WordlistEntry>> = Lazy::new(|| {
    // parse WORDLIST_JSON → build Vec<WordlistEntry>
    // panics on parse error (acceptable for a static initializer over embedded data)
    todo!()
});

// O(1) lookup map: encoding token → array_index (0-based)
static ENCODING_INDEX: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    // build from WORDLIST
    todo!()
});
```

**Option B — `build.rs` code generation:**
Generate `src/wordlist_data.rs` containing a `static WORDLIST: [WordlistEntry; 256]` literal. The build script reads `data/canonical.json` and emits Rust source. This approach makes the wordlist a zero-runtime-cost constant.

Either approach is acceptable. Document which you chose in the Session Log.

#### Public API

```rust
/// Verify all structural invariants of the loaded wordlist.
/// Must be called (and pass) at the start of the integrity test suite.
pub fn verify_wordlist_integrity() -> Result<(), BiponError>

/// Look up an entry by 1-based flat_index. Returns Err(IndexOutOfRange) for values outside 1–256.
pub fn entry_by_index(flat_index: usize) -> Result<&'static WordlistEntry, BiponError>

/// Look up an entry by its encoding (ASCII) token string.
/// This is on the hot path for mnemonic decoding — must be O(1).
pub fn entry_by_encoding(token: &str) -> Result<&'static WordlistEntry, BiponError>

/// Look up an entry by its canonical (Yorùbá) token string.
pub fn entry_by_canonical(token: &str) -> Result<&'static WordlistEntry, BiponError>

/// Return all entries belonging to a named Macro.
/// Accepts both the display form ("ÈṢÙ") and will work with exact string match.
pub fn entries_for_macro(macro_name: &str) -> Vec<&'static WordlistEntry>

/// Return the 0-based array_index for an encoding token. O(1).
pub fn index_of_encoding(token: &str) -> Result<usize, BiponError>

/// Return the full ordered slice of 256 encoding tokens (flat_index order).
pub fn all_encoding_tokens() -> &'static [&'static str]
```

#### `verify_wordlist_integrity()` — Required Checks

This function is the single gatekeeper for wordlist correctness. Every check is mandatory:

1. Exactly 256 entries loaded.
2. Macro counts exactly: ÈṢÙ=88, ṢÀNGÓ=20, Ọ̀ṢUN=28, YEMỌJA=28, ỌYA=32, ÒGÚN=32, ỌBÀTÁLÁ=28.
3. `flat_index` values form the sequence 1, 2, 3, … 256 with no gaps and no duplicates.
4. `array_index` = `flat_index − 1` for every entry.
5. No two entries share the same `encoding` value.
6. No two entries share the same `canonical` value.
7. Every `encoding` value matches the regex `^[a-z][a-z0-9\-]+$` — lowercase ASCII letters, digits, and hyphens only; must start with a letter.
8. If `constants::MERKLE_ROOT` is non-empty: the computed Merkle root over `all_encoding_tokens()` must equal `MERKLE_ROOT`. If `MERKLE_ROOT` is `""`, skip this check (Phase 1 bootstrap mode).

Any failed check returns `Err(BiponError::WordlistIntegrity(...))` with a message identifying exactly which invariant failed and (where applicable) which index or token is the offending value.

---

### 5.4 `crypto.rs`

```rust
use subtle::ConstantTimeEq;

/// SHA-256 hash of `data`. Uses the `sha2` crate.
pub fn sha256(data: &[u8]) -> [u8; 32]

/// HMAC-SHA512 of `data` with `key`. Uses the `hmac` + `sha2` crates.
pub fn hmac_sha512(key: &[u8], data: &[u8]) -> [u8; 64]

/// SHA-256 binary Merkle tree over string leaves.
///
/// Leaf hash:  SHA-256(leaf.as_bytes())
/// Inner node: SHA-256(left_child_bytes || right_child_bytes)
/// Odd level:  duplicate the last node before pairing
///
/// Example with 4 leaves [A, B, C, D]:
///   Level 0 (leaves): H(A), H(B), H(C), H(D)
///   Level 1:          H(H(A)||H(B)), H(H(C)||H(D))
///   Root:             H( H(H(A)||H(B)) || H(H(C)||H(D)) )
///
/// Example with 3 leaves [A, B, C]:
///   Level 0: H(A), H(B), H(C)
///   Pad:     H(A), H(B), H(C), H(C)   ← duplicate last
///   Level 1: H(H(A)||H(B)), H(H(C)||H(C))
///   Root:    H( H(H(A)||H(B)) || H(H(C)||H(C)) )
pub fn sha256_merkle_root(leaves: &[&str]) -> [u8; 32]

/// Convenience wrapper: compute Merkle root over wordlist::all_encoding_tokens()
/// and return it as a lowercase hex string.
/// Run this once in Phase 2 to obtain the value to pin as MERKLE_ROOT.
pub fn compute_wordlist_merkle_root() -> String

/// Timing-safe byte equality. Uses subtle::ConstantTimeEq internally.
/// Use this for any comparison that involves secret material (checksums, keys).
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool
```

**Sanity check:** SHA-256 of the empty byte slice `&[]` must equal `e3b0c44998fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. Write a unit test for this in Phase 2 before relying on the hash for anything else.

---

### 5.5 `mnemonic.rs`

```rust
use zeroize::Zeroizing;
use crate::error::BiponError;

/// Convert raw entropy bytes to a mnemonic phrase (list of encoding-layer tokens).
///
/// Valid entropy lengths: 16, 20, 24, 28, 32 bytes (128/160/192/224/256 bits).
/// Returns Err(InvalidEntropyLength) for any other length.
pub fn entropy_to_mnemonic(entropy: &[u8]) -> Result<Vec<String>, BiponError>

/// Convert a mnemonic (encoding-layer token strings) back to entropy bytes.
///
/// The returned bytes are wrapped in Zeroizing to ensure they are wiped on drop.
/// Returns descriptive errors for: wrong word count, unknown token, checksum mismatch.
pub fn mnemonic_to_entropy(words: &[&str]) -> Result<Zeroizing<Vec<u8>>, BiponError>

/// Validate a mnemonic phrase: verify all words are in the wordlist and the checksum is correct.
/// Does not return the entropy — use mnemonic_to_entropy for that.
pub fn validate_mnemonic(words: &[&str]) -> Result<(), BiponError>

/// Split a mnemonic phrase string on whitespace into token strings.
pub fn split_mnemonic(phrase: &str) -> Vec<&str>

/// Join token strings with a single space.
pub fn join_mnemonic(words: &[String]) -> String
```

#### Encoding Algorithm — `entropy_to_mnemonic`

```
Input:  entropy: &[u8]

Step 1 — Validate length:
    If entropy.len() ∉ {16, 20, 24, 28, 32}:
        return Err(InvalidEntropyLength { bits: entropy.len() * 8 })

Step 2 — Derive parameters from ENTROPY_WORD_TABLE:
    entropy_bits  = entropy.len() * 8
    (_, word_count, checksum_bits, pad_bits) = lookup(entropy_bits)

Step 3 — Compute checksum:
    h = sha256(entropy)
    checksum = h[0]  ← only the most-significant checksum_bits of this byte are used

Step 4 — Build bit stream:
    stream = []
    Append all bits of entropy (MSB first for each byte)
    Append the top checksum_bits bits of h[0] (bits 7 down to 8-checksum_bits)
    Append pad_bits zero bits

    At this point len(stream) == word_count * 8

Step 5 — Extract words:
    Split stream into word_count chunks of 8 bits each
    Each chunk is a u8 value v in 0..=255
    word = WORDLIST[v].encoding   (v is the 0-based array_index)

Step 6:
    Return the word_count tokens as Vec<String>
```

**Key constraint:** The checksum is computed over the raw entropy bytes **before** any padding is applied. The padding bits are always `0` and are appended purely to make the total bit count a multiple of 8. They are discarded during decoding.

#### Decoding Algorithm — `mnemonic_to_entropy`

```
Input:  words: &[&str]

Step 1 — Identify entropy parameters:
    Match words.len() against the word_count column of ENTROPY_WORD_TABLE
    If no match: return Err(InvalidMnemonicLength { words: words.len() })
    Retrieve: entropy_bits, checksum_bits, pad_bits

Step 2 — Resolve tokens to indices:
    For each word at position i:
        array_index = index_of_encoding(word)?
        If not found: return Err(InvalidWord { position: i, word: word.to_string() })

Step 3 — Reconstruct bit stream:
    For each array_index: append 8 bits (MSB first)
    Total stream length = words.len() * 8

Step 4 — Extract entropy and checksum:
    entropy_bytes = stream[0 .. entropy_bits] interpreted as bytes
    checksum_from_stream = the next checksum_bits bits from the stream

Step 5 — Verify checksum:
    h = sha256(&entropy_bytes)
    expected_checksum_bits = top checksum_bits of h[0]
    if !ct_eq(checksum_from_stream_as_byte, expected_as_byte):
        return Err(ChecksumMismatch)
    Note: compare only the relevant bits — mask off the pad region

Step 6:
    return Ok(Zeroizing(entropy_bytes))
```

**Checksum comparison note:** For the checksum comparison, extract the `checksum_bits` from the stream into a single byte aligned to the high bits, and compute the expected value likewise. Then use `ct_eq` on a 1-byte slice for both. The pad bits are always zero and are not part of the checksum — do not include them in the comparison.

---

### 5.6 `seed.rs`

```rust
use zeroize::Zeroizing;
use crate::error::BiponError;

/// Derive a 64-byte seed from a mnemonic and optional passphrase.
///
/// Uses PBKDF2-HMAC-SHA512 with 2048 iterations.
/// Both the mnemonic phrase and the passphrase are NFKD-normalized before use.
pub fn mnemonic_to_seed(words: &[&str], passphrase: &str) -> Result<Zeroizing<Vec<u8>>, BiponError>
```

#### Algorithm

```
Step 1 — Build NFKD-normalized mnemonic string:
    mnemonic_str = NFKD( words.join(" ") )
    (use the unicode_normalization crate: .nfkd().collect::<String>())

Step 2 — Build NFKD-normalized passphrase:
    passphrase_n = NFKD( passphrase )

Step 3 — Construct salt:
    if passphrase_n.is_empty():
        salt = PBKDF2_SALT_BASE                                  → "BIPỌ̀N39 seed"
    else:
        salt = PBKDF2_SALT_BASE + PBKDF2_PASSPHRASE_PREFIX + passphrase_n
               → "BIPỌ̀N39 seed Ọ̀RÍ:<passphrase>"

    Note: NFKD-normalize the salt string as well for full cross-platform safety.

Step 4 — PBKDF2:
    output = PBKDF2-HMAC-SHA512(
        password   = mnemonic_str.as_bytes(),
        salt       = salt.as_bytes(),
        iterations = PBKDF2_ITERATIONS,     // 2048
        dklen      = PBKDF2_OUTPUT_BYTES,   // 64
    )

Step 5:
    return Ok(Zeroizing(output.to_vec()))
```

**NFKD is mandatory, not optional.** The same mnemonic phrase written with precomposed vs. decomposed Unicode characters must produce identical seeds. Skipping normalization causes platform-specific seed divergence — a catastrophic interoperability failure.

The `unicode-normalization` crate is already in `Cargo.toml`. Use `use unicode_normalization::UnicodeNormalization;` and call `.nfkd().collect::<String>()`.

---

### 5.7 `derivation.rs`

```rust
use zeroize::ZeroizeOnDrop;
use crate::error::BiponError;
use crate::constants::{MASTER_KEY_NATIVE, MASTER_KEY_BIP32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivationMode {
    /// Native BIPỌ̀N39 derivation. HMAC key = "BIPỌ̀N39 master"
    Native,
    /// BIP-32 compatible derivation. HMAC key = "Bitcoin seed"
    Bip32,
}

/// Master private key and chain code derived from a 64-byte seed.
#[derive(ZeroizeOnDrop)]
pub struct MasterKey {
    /// First 32 bytes of HMAC-SHA512(key=mode_string, data=seed). The private key (IL).
    pub key: [u8; 32],
    /// Last 32 bytes of HMAC-SHA512(key=mode_string, data=seed). The chain code (IR).
    pub chain_code: [u8; 32],
}

impl MasterKey {
    /// Hex-encode the private key.
    pub fn key_hex(&self) -> String

    /// Hex-encode the chain code.
    pub fn chain_code_hex(&self) -> String
}

/// Derive a MasterKey from a 64-byte seed using the specified mode.
///
/// Algorithm: I = HMAC-SHA512(key=mode.key_string(), data=seed)
///            key        = I[0..32]
///            chain_code = I[32..64]
pub fn master_from_seed(seed: &[u8], mode: DerivationMode) -> Result<MasterKey, BiponError>
```

**Critical:** `MasterKey` derives `ZeroizeOnDrop` — the key and chain code bytes are wiped when the struct is dropped. Do not add `Clone` or `Copy` to `MasterKey`. If callers need to retain the values, they must extract them as hex strings before the struct is dropped.

---

### 5.8 `ifascript.rs`

```rust
use crate::error::BiponError;

/// The seven Macro groupings of the BIPỌ̀N39 wordlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Macro {
    Esu,      // ÈṢÙ     — indices 1–88
    Sango,    // ṢÀNGÓ   — indices 89–108
    Osun,     // Ọ̀ṢUN   — indices 109–136
    Yemoja,   // YEMỌJA  — indices 137–164
    Oya,      // ỌYA     — indices 165–196
    Ogun,     // ÒGÚN    — indices 197–228
    Obatala,  // ỌBÀTÁLÁ — indices 229–256
}

impl Macro {
    /// The canonical display name of this Macro (Yorùbá with diacritics).
    pub fn name(&self) -> &'static str

    /// The inclusive 1-based flat_index range: (start, end).
    pub fn index_range(&self) -> (usize, usize)

    /// Number of tokens in this Macro.
    pub fn count(&self) -> usize

    /// Parse a Macro from either its canonical name ("ÈṢÙ") or a simplified ASCII form ("esu").
    /// Returns None if the string does not match any Macro.
    pub fn from_name(s: &str) -> Option<Macro>

    /// Return the Macro that contains the given 1-based flat_index.
    /// Returns None if the index is out of 1–256 range.
    pub fn from_flat_index(flat_index: usize) -> Option<Macro>
}

/// Distribution of a mnemonic's words across the seven Macros.
pub struct MacroDistribution {
    /// Counts in Macro order (Esu, Sango, Osun, Yemoja, Oya, Ogun, Obatala).
    pub counts: [(Macro, usize); 7],
    /// Sum of all counts — equals the mnemonic word count.
    pub total: usize,
}

/// XOR-reduce all word array_indices (0-based) to produce a single byte (0–255).
/// This is the Odù fingerprint of the mnemonic.
///
/// Algorithm: result = 0u8; for each word: result ^= array_index as u8; return result
pub fn odu_primary_index(words: &[&str]) -> Result<u8, BiponError>

/// Count how many words in the mnemonic belong to each Macro.
pub fn macro_distribution(words: &[&str]) -> Result<MacroDistribution, BiponError>

/// Return the Macro with the highest word count.
/// Ties are broken by lowest flat_index range start (i.e., Esu wins over Sango if equal).
pub fn dominant_macro(words: &[&str]) -> Result<Macro, BiponError>
```

---

### 5.9 `display.rs`

```rust
use crate::error::BiponError;

/// Given an encoding (ASCII) token, return its canonical (Yorùbá) counterpart.
pub fn canonical_for_encoding(token: &str) -> Result<&'static str, BiponError>

/// Given a canonical (Yorùbá) token, return its encoding (ASCII) counterpart.
pub fn encoding_for_canonical(token: &str) -> Result<&'static str, BiponError>

/// Convert a mnemonic of encoding tokens to their canonical forms.
pub fn mnemonic_to_canonical(words: &[&str]) -> Result<Vec<&'static str>, BiponError>

/// Convert a mnemonic of canonical tokens to their encoding forms.
pub fn canonical_to_encoding(words: &[&str]) -> Result<Vec<&'static str>, BiponError>

/// Format a mnemonic as a numbered list: "1. esu-elegbara  2. sango  3. ..."
pub fn format_numbered(words: &[&str]) -> String

/// Format a mnemonic with canonical forms, numbered.
pub fn format_numbered_canonical(words: &[&str]) -> Result<String, BiponError>
```

---

### 5.10 `lib.rs`

`lib.rs` is the public face of the crate. It must:
1. Declare all modules with `pub mod`.
2. Re-export every public item that an external consumer of the crate will need.
3. Contain no logic of its own — only `pub mod` and `pub use` statements.
4. Have a crate-level doc comment (`//!`) describing the library.

```rust
//! # bipon39
//!
//! BIPỌ̀N39 — Sovereign Base-256 mnemonic library for the Ọmọ Kọ́dà ecosystem.
//!
//! Provides entropy-to-mnemonic encoding, mnemonic-to-seed derivation,
//! master key derivation, and Ifáscript metadata over a 256-token
//! culturally-rooted Yorùbá wordlist.

pub mod constants;
pub mod crypto;
pub mod derivation;
pub mod display;
pub mod error;
pub mod ifascript;
pub mod mnemonic;
pub mod seed;
pub mod wordlist;

pub use error::BiponError;
pub use mnemonic::{entropy_to_mnemonic, mnemonic_to_entropy, validate_mnemonic};
pub use seed::mnemonic_to_seed;
pub use derivation::{DerivationMode, MasterKey, master_from_seed};
pub use ifascript::{Macro, odu_primary_index, macro_distribution, dominant_macro};
pub use wordlist::{WordlistEntry, verify_wordlist_integrity, entry_by_index,
                   entry_by_encoding, all_encoding_tokens};
```

---

## 6. CRYPTOGRAPHIC PARAMETER REFERENCE

Quick lookup table — use this when implementing or auditing any crypto operation.

| Parameter | Value | Notes |
|---|---|---|
| Bits per word | 8 | Each encoding token represents exactly one byte |
| Checksum length | `entropy_bits / 32` bits | From the high bits of SHA-256(entropy)[0] |
| Valid word counts | 17, 21, 25, 29, 33 | For 128, 160, 192, 224, 256-bit entropy |
| Seed KDF | PBKDF2-HMAC-SHA512 | `pbkdf2` crate |
| Seed iterations | 2048 | `PBKDF2_ITERATIONS` |
| Seed output | 64 bytes | `PBKDF2_OUTPUT_BYTES` |
| Salt (no passphrase) | `"BIPỌ̀N39 seed"` | NFKD-normalized |
| Salt (with passphrase) | `"BIPỌ̀N39 seed Ọ̀RÍ:<passphrase>"` | Both parts NFKD-normalized |
| Native master key | HMAC-SHA512, key=`"BIPỌ̀N39 master"` | `MASTER_KEY_NATIVE` constant |
| BIP-32 master key | HMAC-SHA512, key=`"Bitcoin seed"` | `MASTER_KEY_BIP32` constant |
| Merkle leaf | SHA-256(encoding_token.as_bytes()) | Over the ASCII token bytes |
| Merkle inner node | SHA-256(left_hash \|\| right_hash) | Concatenate 32+32 bytes |
| Merkle odd-level | Duplicate last node | Before pairing |
| Secret comparison | `ct_eq()` via `subtle::ConstantTimeEq` | Never use `==` on secrets |
| Memory safety | `Zeroizing<T>` or `ZeroizeOnDrop` | Entropy, seeds, private keys |

---

## 7. BIT-STREAM ENCODING — WORKED EXAMPLE

This section exists to prevent the most common implementation mistake: misaligning the checksum extraction.

**Example: 16 bytes of entropy (128 bits)**

```
entropy = [0x00; 16]   (16 zero bytes)
entropy_bits   = 128
checksum_bits  = 4
word_count     = 17
pad_bits       = 4

SHA-256([0x00; 16]) = e3b0c44998fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
h[0] = 0xe3 = 0b11100011

Top 4 bits of h[0] = 0b1110 = 14 (decimal)

Bit stream construction:
  128 bits of entropy (all zeros)
  + 4 checksum bits: 1, 1, 1, 0
  + 4 pad bits:      0, 0, 0, 0
  = 136 bits total = 17 bytes

Split into 17 chunks of 8 bits:
  Chunks 0–15: 0b00000000 = 0 → WORDLIST[0].encoding = "esu-elegbara"
  Chunk 16:    0b11100000 = 0b1110_0000 = 224 → WORDLIST[224].encoding

Result: 16 × "esu-elegbara", then 1 × WORDLIST[224].encoding
```

> **Note for Phase 3:** Before pinning a test vector for all-zero 128-bit entropy, manually compute which token sits at array_index 224 from the actual `Canonical JSON`. Do not assume it from the example above — verify against the real data.

**Checksum extraction detail:**
The checksum occupies the top N bits of `h[0]`. To extract N bits from a byte:
```rust
let top_n_bits = h[0] >> (8 - checksum_bits);   // e.g., 0xe3 >> 4 = 0x0e = 14
```
During decoding, to compare:
```rust
let stream_checksum_byte = /* 8-bit value with checksum in top N bits, bottom (8-N) bits = 0 */ ;
let expected_byte        = h[0] & (0xFF << (8 - checksum_bits) as u8);
ct_eq(&[stream_checksum_byte], &[expected_byte])
```

---

## 8. TEST SPECIFICATIONS

Every test file must be complete before its Phase gate is considered passed. "Complete" means all listed cases are implemented and passing — not just the happy path.

### 8.1 `tests/wordlist_integrity.rs`

| Test | Description |
|---|---|
| `loads_256_entries` | WORDLIST has exactly 256 entries |
| `macro_counts_correct` | ÈṢÙ=88, ṢÀNGÓ=20, Ọ̀ṢUN=28, YEMỌJA=28, ỌYA=32, ÒGÚN=32, ỌBÀTÁLÁ=28 |
| `flat_index_sequence` | Indices 1–256 with no gaps or duplicates |
| `array_index_equals_flat_minus_one` | array_index == flat_index − 1 for all entries |
| `no_duplicate_encoding_tokens` | All 256 encoding strings are unique |
| `no_duplicate_canonical_tokens` | All 256 canonical strings are unique |
| `encoding_regex_compliance` | All encoding tokens match `^[a-z][a-z0-9\-]+$` |
| `boundary_spot_checks` | All 14 entries from the Verified Boundary Tokens table in Section 1.2 |
| `merkle_root_matches_pinned_constant` | Computed root == MERKLE_ROOT (skip if MERKLE_ROOT == "") |
| `verify_wordlist_integrity_passes` | The public function returns Ok(()) |

### 8.2 `tests/mnemonic_roundtrip.rs`

| Test | Description |
|---|---|
| `roundtrip_128_bit` | 16 random bytes → mnemonic → entropy == original |
| `roundtrip_160_bit` | Same for 20 bytes |
| `roundtrip_192_bit` | Same for 24 bytes |
| `roundtrip_224_bit` | Same for 28 bytes |
| `roundtrip_256_bit` | Same for 32 bytes |
| `all_zeros_128_bit` | `[0x00; 16]` roundtrip |
| `all_zeros_256_bit` | `[0x00; 32]` roundtrip |
| `all_ff_256_bit` | `[0xFF; 32]` roundtrip |
| `wrong_word_count_err` | 16 words → `InvalidMnemonicLength { words: 16 }` |
| `wrong_word_count_zero_err` | 0 words → `InvalidMnemonicLength { words: 0 }` |
| `unknown_word_err` | Replace word at position 3 with `"xyz-fake"` → `InvalidWord { position: 3, ... }` |
| `corrupted_checksum_err` | Flip a bit in the last word's index → `ChecksumMismatch` |
| `validate_mnemonic_passes_for_valid` | `validate_mnemonic` returns Ok(()) on a valid mnemonic |
| `validate_mnemonic_fails_for_bad_checksum` | Returns `ChecksumMismatch` |

### 8.3 `tests/mnemonic_vectors.rs`

Load `vectors/test_vectors.json`. For each vector:
- Decode `entropy_hex` to bytes.
- Call `entropy_to_mnemonic(entropy)` and compare to the stored `mnemonic` array.
- Call `mnemonic_to_entropy` on the stored mnemonic and compare to entropy bytes.
- If `seed_hex` is present: call `mnemonic_to_seed(mnemonic, passphrase_or_empty)` and compare.
- If `master_key_native_hex` is present: call `master_from_seed(seed, Native)` and compare key and chain_code hex.
- If `master_key_bip32_hex` is present: same for `Bip32` mode.

**Minimum required vectors (populate in Phases 3–5):**
- All-zero entropy at each of 5 lengths, no passphrase.
- All-zero 256-bit entropy with passphrase `"àṣẹ"`.

### 8.4 `tests/seed_derivation.rs`

| Test | Description |
|---|---|
| `empty_passphrase_deterministic` | Same inputs → same 64-byte output on repeated calls |
| `passphrase_changes_seed` | Same mnemonic, different passphrase → different seed |
| `empty_vs_nonempty_passphrase_differ` | Passphrase="" and passphrase="a" produce different seeds |
| `output_is_64_bytes` | Length of output == 64 |
| `nfkd_normalization_applied` | Precomposed passphrase (e.g. `"a\u{0301}"`) == decomposed equivalent (`"á"`) |
| `pinned_vector_no_passphrase` | From test_vectors.json |
| `pinned_vector_with_passphrase` | From test_vectors.json |

### 8.5 `tests/derivation.rs`

| Test | Description |
|---|---|
| `native_mode_deterministic` | Same seed → same key and chain_code on repeated calls |
| `bip32_mode_deterministic` | Same for BIP-32 mode |
| `native_ne_bip32` | Same seed → different key/chain_code for each mode |
| `key_is_32_bytes` | key.len() == 32 |
| `chain_code_is_32_bytes` | chain_code.len() == 32 |
| `pinned_native_vector` | From test_vectors.json |
| `pinned_bip32_vector` | From test_vectors.json |

### 8.6 `tests/ifascript.rs`

| Test | Description |
|---|---|
| `odu_index_in_range` | `odu_primary_index` result is in 0..=255 |
| `odu_index_deterministic` | Same mnemonic → same result |
| `odu_xor_correctness` | Manually verify XOR of known indices |
| `macro_distribution_known_mnemonic` | Count distribution matches expected breakdown |
| `dominant_macro_correct` | Returns the macro with the highest count |
| `dominant_macro_tie_breaks_by_lower_index` | Tie → macro with lower flat_index start wins |
| `canonical_encoding_roundtrip` | `canonical_for_encoding(x)` → `encoding_for_canonical(y)` → x |
| `macro_from_flat_index` | Spot-check: index 1→Esu, 89→Sango, 109→Osun, 137→Yemoja, 165→Oya, 197→Ogun, 229→Obatala |
| `macro_from_flat_index_out_of_range` | Index 0 and 257 → None |
| `entries_for_macro_counts` | Verify entry counts per macro |

---

## 9. PINNED TEST VECTOR SCHEMA

Populate `vectors/test_vectors.json` during Phases 3–5. The schema:

```json
{
  "schema_version": "1.0",
  "description": "BIPỌ̀N39 pinned test vectors — generated from canonical implementation",
  "generated_at": "YYYY-MM-DD",
  "vectors": [
    {
      "id": "v001",
      "description": "all-zero 128-bit entropy, no passphrase",
      "entropy_hex": "00000000000000000000000000000000",
      "entropy_bits": 128,
      "passphrase": "",
      "mnemonic": ["token1", "token2", "..."],
      "mnemonic_phrase": "token1 token2 ...",
      "word_count": 17,
      "seed_hex": "...",
      "master_key_native_hex": "...",
      "master_chain_native_hex": "...",
      "master_key_bip32_hex": "...",
      "master_chain_bip32_hex": "..."
    },
    {
      "id": "v002",
      "description": "all-zero 160-bit entropy, no passphrase",
      "entropy_hex": "0000000000000000000000000000000000000000",
      "entropy_bits": 160,
      "passphrase": "",
      "mnemonic": ["..."],
      "word_count": 21,
      "seed_hex": "..."
    },
    {
      "id": "v003",
      "description": "all-zero 192-bit entropy, no passphrase",
      "entropy_bits": 192,
      "word_count": 25
    },
    {
      "id": "v004",
      "description": "all-zero 224-bit entropy, no passphrase",
      "entropy_bits": 224,
      "word_count": 29
    },
    {
      "id": "v005",
      "description": "all-zero 256-bit entropy, no passphrase",
      "entropy_hex": "0000000000000000000000000000000000000000000000000000000000000000",
      "entropy_bits": 256,
      "passphrase": "",
      "mnemonic": ["..."],
      "word_count": 33,
      "seed_hex": "...",
      "master_key_native_hex": "...",
      "master_chain_native_hex": "...",
      "master_key_bip32_hex": "...",
      "master_chain_bip32_hex": "..."
    },
    {
      "id": "v006",
      "description": "all-zero 256-bit entropy, passphrase: àṣẹ (precomposed unicode)",
      "entropy_hex": "0000000000000000000000000000000000000000000000000000000000000000",
      "entropy_bits": 256,
      "passphrase": "àṣẹ",
      "mnemonic": ["..."],
      "word_count": 33,
      "seed_hex": "..."
    }
  ]
}
```

**Process for generating vectors:**
1. Implement the function under test.
2. Write a `#[test]` with `-- --nocapture` that prints the hex output.
3. Run `cargo test <test_name> -- --nocapture`.
4. Paste the printed value into `test_vectors.json`.
5. Rewrite the test to assert against the pinned value.
6. Run `cargo test` to confirm the assertion passes.

Vectors are not invented — they are generated by the implementation and then locked. If a future change causes a vector to fail, that is a breaking change requiring a full regression investigation.

---

## 10. PHASED BUILD CHECKLIST

### Phase 0 — Bootstrap
**Goal:** Compilable project structure with all files in place.

- [ ] Clone or initialize repo at `https://github.com/Bino-Elgua/Bipon39-Rust`
- [ ] Create directory structure exactly as in Section 3
- [ ] Fetch `Canonical JSON` from the repo → save verbatim to `data/canonical.json`
- [ ] Fetch `Canonical layer` from the repo → save verbatim to `data/canonical_layer.txt`
- [ ] Fetch `Encoded layer` from the repo → save verbatim to `data/encoded_layer.txt`
- [ ] Write `Cargo.toml` exactly as in Section 4 — no deviations
- [ ] Create all `src/*.rs` files with stub `// TODO` content (so `cargo check` sees the module declarations)
- [ ] Create empty `tests/*.rs` files
- [ ] Create `vectors/test_vectors.json` with empty `vectors: []` array
- [ ] Run `cargo check` — must pass with zero errors
- [ ] Run `cargo build` — must succeed
- [ ] Commit: `"Phase 0: bootstrap — project structure and Cargo.toml"`
- [ ] Update Session Log

---

### Phase 1 — Wordlist Foundation
**Gate: Do not advance to Phase 2 until ALL wordlist_integrity tests pass with 100% green.**

- [ ] Implement `error.rs` — complete `BiponError` enum with all variants as specified
- [ ] Implement `constants.rs` — all constants; `MERKLE_ROOT = ""`
- [ ] Implement `wordlist.rs`:
  - [ ] `WordlistEntry` struct
  - [ ] `WORDLIST` static (via `once_cell` + `include_str!` parsing, or `build.rs` code gen)
  - [ ] `ENCODING_INDEX` lookup map
  - [ ] All public API functions
  - [ ] `verify_wordlist_integrity()` with all 8 checks
- [ ] Write `tests/wordlist_integrity.rs` — all 10 test cases
- [ ] Run `cargo test wordlist_integrity` — must be 100% pass
- [ ] Manually confirm all 14 boundary spot-checks appear in test output
- [ ] Run `cargo clippy -- -D warnings` on completed files — zero warnings
- [ ] Commit: `"Phase 1: wordlist foundation — integrity tests green"`
- [ ] Update Session Log

---

### Phase 2 — Crypto Core & Merkle Root
**Gate: MERKLE_ROOT must be pinned in this phase and never left as "" in any subsequent commit.**

- [ ] Implement `crypto.rs`:
  - [ ] `sha256()`
  - [ ] `hmac_sha512()`
  - [ ] `sha256_merkle_root()`
  - [ ] `compute_wordlist_merkle_root()`
  - [ ] `ct_eq()`
- [ ] Write a temporary `#[test]` that calls `compute_wordlist_merkle_root()` with `-- --nocapture` and prints the result
- [ ] Run `cargo test compute_merkle -- --nocapture` → capture the printed hex string
- [ ] Paste that hex string into `constants::MERKLE_ROOT`
- [ ] Write (or update) the `merkle_root_matches_pinned_constant` test to assert the computed value equals the constant
- [ ] Run `cargo test wordlist_integrity` — all tests still pass, including the Merkle check
- [ ] Write a `sha256_empty_input` unit test in `crypto.rs` that verifies `sha256(&[])` == `e3b0c44998fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- [ ] Run `cargo test` — all tests pass
- [ ] Commit: `"Phase 2: crypto core — MERKLE_ROOT pinned: <paste the hash here>"`
- [ ] Update Session Log

---

### Phase 3 — Mnemonic Encoding and Decoding
**Gate: Roundtrip and vector tests must be 100% green.**

- [ ] Read `src/bipon39.ts` in `https://github.com/Bino-Elgua/bipon39` to understand the reference implementation's checksum extraction and bit-stream construction before writing any code
- [ ] Implement `mnemonic.rs`:
  - [ ] `entropy_to_mnemonic()` — full algorithm per Section 5.5
  - [ ] `mnemonic_to_entropy()` — full algorithm per Section 5.5
  - [ ] `validate_mnemonic()`
  - [ ] `split_mnemonic()` and `join_mnemonic()`
- [ ] Write `tests/mnemonic_roundtrip.rs` — all 14 test cases
- [ ] Run `cargo test mnemonic_roundtrip` — 100% pass
- [ ] **Manually verify:** Run all-zero 256-bit entropy through `entropy_to_mnemonic()` with `-- --nocapture`, inspect all 33 tokens, confirm the checksum word is derived from the correct checksum bits
- [ ] Generate all 5 zero-entropy vectors (no passphrase) → record `mnemonic` arrays in `test_vectors.json` (leave seed and key fields as TBD for now)
- [ ] Write `tests/mnemonic_vectors.rs` with mnemonic-only assertions (seed/key assertions come in Phases 4–5)
- [ ] Run `cargo test mnemonic_vectors` — 100% pass
- [ ] Commit: `"Phase 3: mnemonic encode/decode — roundtrip tests green, 5 vectors recorded"`
- [ ] Update Session Log

---

### Phase 4 — Seed Derivation
**Gate: PBKDF2 vectors must be pinned. NFKD normalization must be explicitly tested.**

- [ ] Implement `seed.rs` — `mnemonic_to_seed()` with NFKD normalization per Section 5.6
- [ ] Write `tests/seed_derivation.rs` — all 7 test cases
- [ ] Run `cargo test seed_derivation` — 100% pass
- [ ] Generate seeds for all 5 zero-entropy vectors (no passphrase) → add `seed_hex` to each in `test_vectors.json`
- [ ] Generate seed for 256-bit zero-entropy + passphrase `"àṣẹ"` → add `seed_hex` to v006
- [ ] Update `tests/mnemonic_vectors.rs` to assert seed values for vectors that have them
- [ ] Run `cargo test mnemonic_vectors` — 100% pass
- [ ] Commit: `"Phase 4: seed derivation — PBKDF2 vectors pinned, NFKD verified"`
- [ ] Update Session Log

---

### Phase 5 — Master Key Derivation
**Gate: Both Native and BIP-32 vectors pinned. Confirmed they differ for the same seed.**

- [ ] Implement `derivation.rs`:
  - [ ] `DerivationMode` enum
  - [ ] `MasterKey` struct with `ZeroizeOnDrop`
  - [ ] `master_from_seed()`
  - [ ] `key_hex()` and `chain_code_hex()` on `MasterKey`
- [ ] Write `tests/derivation.rs` — all 7 test cases
- [ ] Run `cargo test derivation` — 100% pass
- [ ] Generate native and BIP-32 master keys for v001 (128-bit all-zero, no passphrase) and v005 (256-bit all-zero, no passphrase) → add to `test_vectors.json`
- [ ] Confirm `master_key_native_hex != master_key_bip32_hex` for the same seed — document in Session Log
- [ ] Update `tests/mnemonic_vectors.rs` to assert master key values
- [ ] Run `cargo test` — all tests pass
- [ ] Commit: `"Phase 5: master key derivation — Native and BIP-32 vectors pinned"`
- [ ] Update Session Log

---

### Phase 6 — Ifáscript and Display Layer
**Gate: All ifascript and display tests green.**

- [ ] Implement `ifascript.rs` — full `Macro` enum + all functions per Section 5.8
- [ ] Implement `display.rs` — all conversion and formatting functions per Section 5.9
- [ ] Write `tests/ifascript.rs` — all 10 test cases
- [ ] Run `cargo test ifascript` — 100% pass
- [ ] Run `cargo test` — full suite passes
- [ ] Commit: `"Phase 6: Ifáscript and display layer — all tests green"`
- [ ] Update Session Log

---

### Phase 7 — Polish, Docs, and CI
**Gate: Zero clippy warnings. Zero doc warnings. CI green on push.**

- [ ] Complete `lib.rs` — all modules declared, full public API re-exported, crate-level doc comment
- [ ] Add `///` doc comments to every public item in every module
- [ ] Run `cargo clippy -- -D warnings` → zero warnings
- [ ] Run `cargo doc --no-deps` → zero documentation warnings
- [ ] Write `benches/throughput.rs`:
  - [ ] `entropy_to_mnemonic` benchmark (256-bit random entropy, 100 iterations)
  - [ ] `mnemonic_to_seed` benchmark
  - [ ] `compute_wordlist_merkle_root` benchmark
- [ ] Run `cargo bench` — completes without panic
- [ ] Write `.github/workflows/ci.yml`:
  - Trigger on push and pull_request to main
  - Ubuntu latest, stable Rust toolchain
  - Steps: `cargo build`, `cargo test --release`, `cargo clippy -- -D warnings`
- [ ] Write `README.md`: description, install, quick-start usage, table of test vectors, links to SPEC.md and AGENT_CHECKLIST.md
- [ ] Write `SPEC.md`: formal technical specification (algorithm details, parameter rationale, security model, cross-platform guarantees)
- [ ] Push to GitHub — CI must pass on the push
- [ ] Commit: `"Phase 7: polish — CI green, docs complete, benchmarks running"`
- [ ] Update Session Log

---

### Phase 8 — Ecosystem Integration
**Goal:** The library is ready for consumption by downstream Ọmọ Kọ́dà components.

- [ ] Write integration guide: Ifáscript — how to use `macro_distribution`, `odu_primary_index`, `dominant_macro` in downstream apps
- [ ] Write integration guide: Orí Kọ́dà — mnemonic generation, seed derivation, and passphrase handling
- [ ] Write integration guide: Ọmọ-kọ́dà — full entropy-to-master-key flow with both derivation modes
- [ ] Publish stable `MERKLE_ROOT` value in `README.md` for on-chain anchoring reference
- [ ] Tag `v0.1.0` on the main branch
- [ ] Push tag to `https://github.com/Bino-Elgua/Bipon39-Rust`
- [ ] Update Session Log — final entry for v0.1.0

---

## 11. ACCEPTANCE GATE CRITERIA

The implementation is considered production-ready when every item below is true simultaneously:

| Criterion | How to Verify |
|---|---|
| `cargo test` passes with zero failures | `cargo test 2>&1 | tail -5` — should show `0 failed` |
| `cargo clippy -- -D warnings` passes | Zero lines of warning output |
| `MERKLE_ROOT` is pinned and verified | The `merkle_root_matches_pinned_constant` test exists, runs, and passes |
| Full roundtrip for all 5 entropy lengths | `mnemonic_roundtrip` test suite |
| Pinned seed vectors match | `mnemonic_vectors` test suite |
| Pinned master key vectors match (both modes) | `derivation` and `mnemonic_vectors` test suites |
| All 14 boundary spot-checks pass | `wordlist_integrity` test suite |
| NFKD normalization explicitly tested | `seed_derivation::nfkd_normalization_applied` test |
| Native mode ≠ BIP-32 mode for same seed | `derivation::native_ne_bip32` test |
| CI workflow passes on GitHub Actions | Green check on last commit |
| Session Log in this file reflects final state | Read the top Session Log entry |

---

## 12. WHAT TO TAKE FROM THE REFERENCE REPO

The reference repo at `https://github.com/Bino-Elgua/bipon39` is a TypeScript implementation of a related but incompatible system. Study it for the following — and only the following:

| Permitted to borrow | How to use it |
|---|---|
| PBKDF2 parameters | Confirm: 2048 iterations, HMAC-SHA512, 64-byte output, NFKD normalization applied to both mnemonic and passphrase |
| Salt construction | `"BIPỌ̀N39 seed"` base + `" Ọ̀RÍ:<passphrase>"` suffix when non-empty |
| Merkle tree strategy | Binary tree, SHA-256 leaf hashes, duplicate-last for odd levels — same strategy, different leaves |
| Odù XOR logic | XOR-reduce of word array indices — same concept, applied to our 256-token list |
| API shape | Function names and signatures are inspired by; adapted to Rust idioms |
| Test vector format | The `vectors.json` schema structure |
| Master key derivation | HMAC-SHA512(key=mode_string, data=seed) — identical algorithm |
| CLI design | Reference only — we do not implement a CLI in v0.1.0 |

| Forbidden to borrow | Reason |
|---|---|
| Any token string (`esu-gate`, `sango-volt`, `ogun-forge`, `irawo-dawn`, etc.) | Completely different and incompatible wordlist |
| The Merkle root hash (`0ab1fafa...`) | Computed over their 256 tokens, not ours |
| Subtone expansion / 2048-mode | BIPỌ̀N39-Rust is Base-256 only; no expansion modes |
| Root names (`orunmila`, `egungun`, `ori`, `ile`, `omi`, `ina`, `afeefe`, `igi`, `irawo`) | These are not in our wordlist |
| Affix names (`gate`, `volt`, `forge`, `stream`, `tide`, `veil`, `crown`, etc.) | These are not in our wordlist |
| Elemental signature logic | Not part of our spec |
| Sabbath gate logic | Not part of our spec |

---

## 13. CRITICAL ANTI-PATTERNS

| ❌ Never Do This | Why It Is Catastrophic |
|---|---|
| Import any token from the `bipon39` repo | The wordlists are incompatible. Mixed tokens produce invalid mnemonics with no error. |
| Use the `bipon39` repo's Merkle root | Their root is over their 256 tokens. Ours is over our 256 tokens. They will differ. |
| Compare checksum bytes or key material with `==` | Timing side channels. Always use `ct_eq()`. |
| Use `unwrap()` in library code | A panic in a cryptographic library is an unhandled failure — unacceptable. Use `?`. |
| Skip NFKD normalization in seed derivation | The same mnemonic with different Unicode normalization forms produces a different seed. Silent data loss. |
| Implement 2048-mode or subtone expansion | This crate is Base-256 only. No expansion, no subtones. Do not add it. |
| Leave `MERKLE_ROOT = ""` in any commit after Phase 2 | Silently disables integrity verification in all production builds. |
| Drop entropy, seed, or private key bytes without `Zeroizing` | Sensitive material lingers in memory, available to forensic tools or other processes. |
| Modify `ENTROPY_WORD_TABLE` | Every existing mnemonic and test vector immediately breaks. |
| Use `unsafe` without documented necessity | Rust's safety guarantees are part of the security model. |
| Put canonical (Yorùbá) strings through any hash function | Only encoding (ASCII) strings are cryptographic inputs. |
| Embed the wordlist as a hand-typed array | Copy `data/canonical.json` byte-for-byte from the repo. Typos in a wordlist are irrecoverable. |
| Generate a vector and immediately pin it without checking | Print it, inspect it manually against the algorithm, then pin. |
| Remove the `merkle_root_matches_pinned_constant` test | It is a permanent fixture. It catches wordlist drift. It must survive forever. |

---

## 14. CI CONFIGURATION

`/.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install stable Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build
        run: cargo build --verbose

      - name: Run tests (release mode)
        run: cargo test --release --verbose

      - name: Clippy (deny warnings)
        run: cargo clippy -- -D warnings

      - name: Check documentation
        run: cargo doc --no-deps --document-private-items
```

This CI must be present from Phase 7 onward. Once added, every push must keep it green.

---

## 15. SESSION LOG

Add a new entry at the **top** of this section (immediately below this heading) before every push. Never delete prior entries — they form the audit trail.

---

────────────────────────────────────────────────────────────────────────────
Session Date     : 2026-05-08
Agent / Platform : Amp
Phase Completed  : Phase 7 — Polish, Docs, and CI
MERKLE_ROOT      : pinned: fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - .github/workflows/ci.yml — added GitHub Actions CI for build, release tests, clippy, and docs on push/PR to main.
  - benches/throughput.rs — added Criterion benchmarks for 256-bit entropy encoding, seed derivation, and Merkle root computation.
  - README.md — added project description, install snippet, quick start, vector summary, verification commands, and documentation links.
  - SPEC.md — added formal technical specification covering wordlist authority, encoding/decoding, seed/master derivation, Ifáscript metadata, security, and cross-platform guarantees.
  - src/lib.rs — expanded public API re-exports for crypto, display, mnemonic helpers, Ifáscript distribution, and wordlist lookups.
  - src/error.rs — added public documentation for the error type and variants.
  - src/ifascript.rs — added public documentation for Macro variants.
  - Read first — added this Phase 7 session log entry.
  - AGENT_CHECKLIST.md — mirrored this session log entry.

Work Completed:
  - Completed Phase 7 public API polish while keeping all modules declared in `lib.rs`.
  - Added documentation for previously undocumented public enum items.
  - Added CI workflow matching the project checklist: checkout, stable Rust + clippy, Cargo cache, `cargo build --verbose`, `cargo test --release --verbose`, `cargo clippy -- -D warnings`, and `cargo doc --no-deps --document-private-items`.
  - Replaced the placeholder benchmark with real Criterion throughput benchmarks.
  - Wrote README and formal SPEC documentation.
  - Pushed commits through Phase 7 to GitHub so CI can run on the remote branch.

Test Status:
  wordlist_integrity : 10/10 PASS
  mnemonic_roundtrip : 14/14 PASS
  mnemonic_vectors   : 1/1 PASS
  seed_derivation    : 7/7 PASS
  derivation         : 7/7 PASS
  ifascript          : 14/14 PASS

Vectors Pinned This Session:
  - none

Open Questions / Blockers:
  - GitHub Actions CI was pushed; final remote green status should be checked on GitHub if required.

Next Priority (first thing next session):
  - Begin Phase 8 ecosystem integration guides and v0.1.0 tagging once remote CI is confirmed green.

Notes for Next Agent:
  - Verification run this session: `cargo fmt`, `cargo test`, `cargo clippy -- -D warnings`, `cargo doc --no-deps`, `cargo bench`, `cargo build --verbose`, and `cargo test --release --verbose` all passed locally.
  - `cargo bench` completed successfully; Criterion reported that gnuplot was unavailable and used the plotters backend, which is non-fatal.
────────────────────────────────────────────────────────────────────────────

────────────────────────────────────────────────────────────────────────────
Session Date     : 2026-05-05
Agent / Platform : Amp
Phase Completed  : Phase 6 — Ifáscript and Display Layer
MERKLE_ROOT      : pinned: fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - src/ifascript.rs — verified full Macro enum and required Ifáscript functions.
  - src/display.rs — verified encoding/canonical conversion and numbered formatting helpers.
  - tests/ifascript.rs — implemented 14 tests covering all required Phase 6 Ifáscript and display behavior.
  - Read first — added this Phase 6 session log entry.
  - AGENT_CHECKLIST.md — mirrored this session log entry.

Work Completed:
  - Confirmed `Macro` exposes `name()`, `index_range()`, `count()`, `from_name()`, and `from_flat_index()` for all seven macro groups.
  - Confirmed `odu_primary_index()`, `macro_distribution()`, and `dominant_macro()` operate over encoding-layer tokens and use wordlist indices/macros.
  - Confirmed display helpers convert encoding↔canonical forms, convert whole mnemonic slices, and format numbered lists.
  - Added tests for Odù XOR correctness/determinism, macro distribution, dominant macro tie-breaking, canonical/encoding roundtrips, numbered formatting, macro lookup/ranges/counts, and entries-per-macro counts.

Test Status:
  wordlist_integrity : 10/10 PASS
  mnemonic_roundtrip : 14/14 PASS
  mnemonic_vectors   : 1/1 PASS
  seed_derivation    : 7/7 PASS
  derivation         : 7/7 PASS
  ifascript          : 14/14 PASS

Vectors Pinned This Session:
  - none

Open Questions / Blockers:
  - None for Phase 6.

Next Priority (first thing next session):
  - Start Phase 7 polish: complete public API re-exports/docs, run doc checks, add real Criterion benchmarks, CI workflow, README.md, and SPEC.md.

Notes for Next Agent:
  - Phase 6 added extra display coverage beyond the 10 listed Ifáscript cases because display helpers are part of the same phase.
  - Verification run this session: `cargo test --test ifascript`, full `cargo test`, and `cargo clippy -- -D warnings` all passed after `cargo fmt`.
────────────────────────────────────────────────────────────────────────────

────────────────────────────────────────────────────────────────────────────
Session Date     : 2026-05-05
Agent / Platform : Amp
Phase Completed  : Phase 5 — Master Key Derivation
MERKLE_ROOT      : pinned: fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - src/derivation.rs — confirmed/formatted `DerivationMode`, `MasterKey`, and HMAC-SHA512 `master_from_seed()` implementation.
  - tests/derivation.rs — implemented all 7 required Phase 5 master key derivation tests.
  - tests/mnemonic_vectors.rs — extended vector assertions to verify Native and BIP-32 master key fields when present.
  - vectors/test_vectors.json — pinned Native and BIP-32 master key + chain-code vectors for v005.
  - Read first — added this Phase 5 session log entry.
  - AGENT_CHECKLIST.md — mirrored this session log entry.

Work Completed:
  - Verified `src/derivation.rs` contains `DerivationMode::{Native, Bip32}`, `MasterKey` with `ZeroizeOnDrop`, `key_hex()`/`chain_code_hex()`, and `master_from_seed()` using HMAC-SHA512 with mode-specific key strings.
  - Generated v005 master key vectors using the Rust implementation via temporary `cargo test --test derivation print_v005_master_key_vectors -- --nocapture`, then replaced the print with permanent tests.
  - Pinned v005 Native key, Native chain code, BIP-32 key, and BIP-32 chain code in `vectors/test_vectors.json`.
  - Confirmed Native and BIP-32 outputs differ for the same seed via `derivation::native_ne_bip32`.
  - Updated `mnemonic_vectors` to assert master key fields for vectors that carry them.

Test Status:
  wordlist_integrity : 10/10 PASS
  mnemonic_roundtrip : 14/14 PASS
  mnemonic_vectors   : 1/1 PASS
  seed_derivation    : 7/7 PASS
  derivation         : 7/7 PASS
  ifascript          : not yet written

Vectors Pinned This Session:
  - v005 master_key_native_hex
  - v005 master_chain_native_hex
  - v005 master_key_bip32_hex
  - v005 master_chain_bip32_hex

Open Questions / Blockers:
  - If v001 master key vectors are still desired from the original checklist, they can be added in a follow-up; this session followed the user request to pin v005.
  - Ifáscript/display tests remain for Phase 6.

Next Priority (first thing next session):
  - Complete Phase 6 by finalizing `ifascript.rs` and `display.rs` behavior against the required tests, then implement all 10 `tests/ifascript.rs` cases.

Notes for Next Agent:
  - Phase 5 vector generation used the crate implementation, not hand-invented values.
  - Verification run this session: `cargo test --test derivation`, `cargo test --test mnemonic_vectors`, full `cargo test`, and `cargo clippy -- -D warnings` all passed after `cargo fmt`.
────────────────────────────────────────────────────────────────────────────

────────────────────────────────────────────────────────────────────────────
Session Date     : 2026-05-05
Agent / Platform : Amp
Phase Completed  : Phase 4 — Seed Derivation
MERKLE_ROOT      : pinned: fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - src/seed.rs — confirmed and formatted NFKD + PBKDF2-HMAC-SHA512 `mnemonic_to_seed()` implementation.
  - tests/seed_derivation.rs — implemented all 7 required Phase 4 seed derivation tests.
  - tests/mnemonic_vectors.rs — extended pinned vector assertions to verify `seed_hex` when present.
  - vectors/test_vectors.json — pinned `seed_hex` for v001–v005 and added v006 (`àṣẹ` passphrase) with seed.
  - src/crypto.rs, src/display.rs, src/ifascript.rs, src/lib.rs, src/wordlist.rs, tests/mnemonic_roundtrip.rs — formatted by `cargo fmt`.
  - Read first — added this Phase 4 session log entry.
  - AGENT_CHECKLIST.md — mirrored this session log entry.

Work Completed:
  - Verified `src/seed.rs` implements the specified algorithm: join mnemonic with spaces, NFKD-normalize mnemonic and passphrase, construct salt from `BIPỌ̀N39 seed` plus ` Ọ̀RÍ:<passphrase>` when non-empty, NFKD-normalize salt, then run PBKDF2-HMAC-SHA512 with 2048 iterations and 64-byte output.
  - Generated seed vectors using the Rust implementation via temporary `cargo test --test seed_derivation print_phase4_seed_vectors -- --nocapture`, then removed the temporary print test.
  - Pinned seeds for all five no-passphrase all-zero entropy vectors and the required v006 all-zero 256-bit vector with passphrase `àṣẹ`.
  - Added deterministic, passphrase-difference, output-length, NFKD normalization, and pinned-vector seed tests.
  - Updated `mnemonic_vectors` to assert seeds for every vector carrying `seed_hex`.

Test Status:
  wordlist_integrity : 10/10 PASS
  mnemonic_roundtrip : 14/14 PASS
  mnemonic_vectors   : 1/1 PASS
  seed_derivation    : 7/7 PASS
  derivation         : not yet written
  ifascript          : not yet written

Vectors Pinned This Session:
  - v001 seed_hex — all-zero 128-bit entropy, no passphrase
  - v002 seed_hex — all-zero 160-bit entropy, no passphrase
  - v003 seed_hex — all-zero 192-bit entropy, no passphrase
  - v004 seed_hex — all-zero 224-bit entropy, no passphrase
  - v005 seed_hex — all-zero 256-bit entropy, no passphrase
  - v006 mnemonic + seed_hex — all-zero 256-bit entropy, passphrase `àṣẹ`

Open Questions / Blockers:
  - Native/BIP-32 master key values remain for Phase 5.

Next Priority (first thing next session):
  - Complete Phase 5 by writing `tests/derivation.rs`, generating Native and BIP-32 master key vectors for v001 and v005, pinning them in `test_vectors.json`, and extending `mnemonic_vectors` to assert master key fields.

Notes for Next Agent:
  - Phase 4 seed generation used the crate implementation, not hand-invented values.
  - Verification run this session: `cargo test --test seed_derivation`, `cargo test --test mnemonic_vectors`, full `cargo test`, and `cargo clippy -- -D warnings` all passed after `cargo fmt`.
────────────────────────────────────────────────────────────────────────────

────────────────────────────────────────────────────────────────────────────
Session Date     : 2026-05-05
Agent / Platform : Amp
Phase Completed  : Phase 3 — Mnemonic Encoding and Decoding
MERKLE_ROOT      : pinned: fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - tests/mnemonic_roundtrip.rs — implemented all 14 required Phase 3 roundtrip/error/validation tests.
  - tests/mnemonic_vectors.rs — implemented pinned mnemonic vector roundtrip assertions.
  - vectors/test_vectors.json — pinned v001–v005 all-zero mnemonic vectors for 128/160/192/224/256-bit entropy.
  - Read first — added this Phase 3 session log entry.
  - AGENT_CHECKLIST.md — mirrored this session log entry.

Work Completed:
  - Read `src/bipon39.ts` from the TypeScript reference repo at `Bino-Elgua/bipon39`.
  - Confirmed the permitted reference patterns: checksum uses the top `ENT / 32` bits of `hash[0]` MSB-first; checksum bits are appended before zero padding; seed derivation uses NFKD and PBKDF2-HMAC-SHA512; Odù primary index is XOR of word indices.
  - Wrote the full mnemonic roundtrip suite covering all five entropy lengths, all-zero/all-FF edge cases, invalid word counts, unknown words, checksum corruption, and validation behavior.
  - Generated and pinned mnemonic arrays for all-zero entropy at the five required lengths.
  - Manually verified the all-zero 256-bit mnemonic with `cargo test --test mnemonic_roundtrip print_all_zero_256_bit_mnemonic_for_manual_verification -- --nocapture`; the checksum word is `sango-monamona` (array index 102), matching SHA-256([0u8; 32])[0] with no padding for 256-bit entropy.
  - Removed the temporary print test after manual inspection.

Test Status:
  wordlist_integrity : 10/10 PASS
  mnemonic_roundtrip : 14/14 PASS
  mnemonic_vectors   : 1/1 PASS
  seed_derivation    : not yet written
  derivation         : not yet written
  ifascript          : not yet written

Vectors Pinned This Session:
  - v001 mnemonic — all-zero 128-bit entropy, no passphrase
  - v002 mnemonic — all-zero 160-bit entropy, no passphrase
  - v003 mnemonic — all-zero 192-bit entropy, no passphrase
  - v004 mnemonic — all-zero 224-bit entropy, no passphrase
  - v005 mnemonic — all-zero 256-bit entropy, no passphrase

Open Questions / Blockers:
  - Seed hex values for v001–v005 and v006 passphrase vector remain for Phase 4.
  - Native/BIP-32 master key values remain for Phase 5.

Next Priority (first thing next session):
  - Implement the full `tests/seed_derivation.rs` suite, generate PBKDF2 seed hex for v001–v005 plus v006 (`àṣẹ` passphrase), and update `tests/mnemonic_vectors.rs` to assert seed values when present.

Notes for Next Agent:
  - Do not use any token strings from the TypeScript reference repo; it was read only for algorithm shape.
  - A parallel Cargo invocation briefly produced a target/cache lock error; rerunning the affected vector test sequentially passed.
  - Verification run this session: `cargo test --test mnemonic_roundtrip`, `cargo test --test mnemonic_vectors`, temporary all-zero 256-bit `--nocapture` print test, full `cargo test`, and `cargo clippy -- -D warnings` all passed.
────────────────────────────────────────────────────────────────────────────

────────────────────────────────────────────────────────────────────────────
Session Date     : 2026-05-05
Agent / Platform : Amp
Phase Completed  : Phase 2 — Wordlist Foundation and Crypto Core
MERKLE_ROOT      : pinned: fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - Cargo.toml — added locked crate metadata and dependency set.
  - Cargo.lock — generated dependency lockfile.
  - data/canonical.json — verbatim copy of repo `Canonical JSON`.
  - data/canonical_layer.txt — verbatim copy of repo `Canonical layer`.
  - data/encoded_layer.txt — verbatim copy of repo `Encoded layer`.
  - src/lib.rs — public module declarations and re-exports.
  - src/error.rs — full BiponError enum.
  - src/constants.rs — constants plus pinned MERKLE_ROOT.
  - src/wordlist.rs — runtime JSON loading, lookup maps, integrity gate.
  - src/crypto.rs — SHA-256, HMAC-SHA512, Merkle root, constant-time equality.
  - src/mnemonic.rs — initial mnemonic encode/decode implementation.
  - src/seed.rs — initial PBKDF2-HMAC-SHA512 seed derivation implementation.
  - src/derivation.rs — initial master key derivation implementation.
  - src/ifascript.rs — initial Macro/Odù distribution implementation.
  - src/display.rs — initial canonical/encoding display helpers.
  - tests/wordlist_integrity.rs — 10 required wordlist integrity tests.
  - benches/throughput.rs — placeholder bench target for manifest validity.
  - vectors/test_vectors.json — initialized empty vector file.

Work Completed:
  - Cloned the repository and read the `Read first` build specification.
  - Copied all three source wordlist artifacts byte-for-byte into `data/` and verified with `cmp`.
  - Bootstrapped a compilable Rust library crate.
  - Implemented the Phase 1 wordlist foundation using `once_cell` + `include_str!()` JSON parsing.
  - Implemented all mandatory wordlist integrity checks and boundary spot checks.
  - Implemented Phase 2 crypto helpers and generated the wordlist Merkle root through the Rust implementation.
  - Pinned `MERKLE_ROOT` to `fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14`.
  - Added initial implementations for later-phase modules so the crate exposes the intended API.

Test Status:
  wordlist_integrity : 10/10 PASS
  mnemonic_roundtrip : not yet written
  mnemonic_vectors   : not yet written
  seed_derivation    : not yet written
  derivation         : not yet written
  ifascript          : not yet written

Vectors Pinned This Session:
  - none

Open Questions / Blockers:
  - Reference TypeScript repo `Bino-Elgua/bipon39` still needs to be read before finalizing Phase 3 mnemonic tests/vectors, per Section 10.
  - Later-phase modules have initial implementations but are not yet covered by their required phase test suites.

Next Priority (first thing next session):
  - Read `src/bipon39.ts` from `https://github.com/Bino-Elgua/bipon39`, then write the full `mnemonic_roundtrip` test suite and generate/pin the five mnemonic vectors.

Notes for Next Agent:
  - Wordlist runtime loading is Option A from the spec (`once_cell` + `include_str!()`), and all runtime wordlist data comes from `data/canonical.json`.
  - The 14 boundary tokens were printed during `cargo test --test wordlist_integrity -- --nocapture` and matched the specification.
  - Verification run this session: `cargo check`, `cargo build`, `cargo test --test wordlist_integrity -- --nocapture`, `cargo test`, and `cargo clippy -- -D warnings` all passed.
────────────────────────────────────────────────────────────────────────────

```
────────────────────────────────────────────────────────────────────────────
Session Date     : [YYYY-MM-DD]
Agent / Platform : [name of agent or platform, e.g. "Claude Sonnet 4.5 / claude.ai mobile"]
Phase Completed  : [e.g. "Phase 1 — Wordlist Foundation"]
MERKLE_ROOT      : [ not yet pinned | pinned: <hex> ]
────────────────────────────────────────────────────────────────────────────

Files Created or Modified:
  - [list each file with a one-line description of change]

Work Completed:
  - [itemized list of what was built or fixed]

Test Status:
  wordlist_integrity : [ x/x PASS | not yet written ]
  mnemonic_roundtrip : [ x/x PASS | not yet written ]
  mnemonic_vectors   : [ x/x PASS | not yet written ]
  seed_derivation    : [ x/x PASS | not yet written ]
  derivation         : [ x/x PASS | not yet written ]
  ifascript          : [ x/x PASS | not yet written ]

Vectors Pinned This Session:
  - [list vector IDs added or updated, e.g. "v001 mnemonic" or "none"]

Open Questions / Blockers:
  - [anything unresolved that the next agent must be aware of]

Next Priority (first thing next session):
  - [the single most important task to begin with — be specific]

Notes for Next Agent:
  - [any context, gotchas, or decisions made this session that are not obvious from the code]
────────────────────────────────────────────────────────────────────────────
```

*No sessions logged yet. First agent: fill in the template above, push, and leave the next agent a clear next priority.*

---

**Àṣẹ.**
*Bínò ÈL Guà — May 5, 2026*

— Master Auditor (Universal Blockchain & Code Specialist) 🤍🗿⚖️🕊️🌄
