# BIPỌ̀N39 Technical Specification

## Scope

BIPỌ̀N39-Rust is a Base-256 mnemonic system. Each encoding-layer token maps to exactly one byte value (`array_index` 0–255). The crate does not implement 2048-word BIP39 mode, subtone expansion, or CLI commands. It does expose the TypeScript reference project's elemental/ritual metadata through the canonical JSON wordlist.

## Wordlist authority

Runtime wordlist data is embedded from `data/canonical.json` with `include_str!()`. The source JSON contains 256 entries in `flat_index` order. Each entry has:

- `flat_index`: 1-based stable index.
- `array_index`: derived as `flat_index - 1`.
- `macro_name`: one of seven Ifáscript macro group names.
- `macro_local_index`: 1-based index within its macro.
- `canonical`: Yorùbá display token.
- `encoding`: ASCII cryptographic token.
- `token_meta`: elemental and ritual metadata (`element`, `ritual_cue`, `ethical_tag`, `sigil_seed`).

Only `encoding` tokens are used in hashing, mnemonic phrases, PBKDF2 password input, and Merkle leaves. Canonical tokens are display-only.

The pinned encoding-token Merkle root is:

```text
fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
```

## Entropy and checksum parameters

Supported entropy lengths are 128, 160, 192, 224, and 256 bits.

| Entropy bits | Checksum bits | Word count | Pad bits |
|---:|---:|---:|---:|
| 128 | 4 | 17 | 4 |
| 160 | 5 | 21 | 3 |
| 192 | 6 | 25 | 2 |
| 224 | 7 | 29 | 1 |
| 256 | 8 | 33 | 0 |

Checksum bits are the most-significant `entropy_bits / 32` bits of `SHA-256(entropy)[0]`. Padding bits are zero and are appended only after checksum bits to align the stream to 8-bit word boundaries.

## Entropy to mnemonic

1. Validate entropy length.
2. Compute `SHA-256(entropy)`.
3. Emit entropy bits MSB-first for each byte.
4. Append the top checksum bits of the first hash byte, MSB-first.
5. Append zero padding bits.
6. Split the stream into 8-bit chunks.
7. Interpret each chunk as an unsigned byte and emit the matching encoding token by `array_index`.

## Mnemonic to entropy

1. Match the word count to the entropy/checksum/padding table.
2. Resolve each encoding token to its `array_index`.
3. Reconstruct the bit stream MSB-first.
4. Extract entropy bytes and checksum bits.
5. Recompute the expected checksum from the entropy.
6. Compare only the relevant checksum bits using constant-time equality.
7. Return entropy in `Zeroizing<Vec<u8>>`.

## Seed derivation

Seed derivation uses PBKDF2-HMAC-SHA512 with:

- iterations: `2048`
- output length: `64` bytes
- PBKDF2 password input: NFKD-normalized mnemonic phrase joined by single spaces
- salt without passphrase: NFKD(`"BIPỌ̀N39 seed"`)
- salt with passphrase: NFKD(`"BIPỌ̀N39 seed Ọ̀RÍ:<passphrase>"`), where passphrase is NFKD-normalized before insertion

The returned seed is `Zeroizing<Vec<u8>>`.

## Master key derivation

`master_from_seed(seed, mode)` requires a 64-byte seed and computes:

```text
I = HMAC-SHA512(key = mode_key, data = seed)
master_private_key = I[0..32]
chain_code         = I[32..64]
```

Mode keys:

- Native: `"BIPỌ̀N39 master"`
- BIP-32: `"Bitcoin seed"`

`MasterKey` uses `ZeroizeOnDrop` and is intentionally not `Clone` or `Copy`.

## Ifáscript metadata

The seven macro groups are fixed:

| Macro | Range | Count |
|---|---:|---:|
| ÈṢÙ | 1–88 | 88 |
| ṢÀNGÓ | 89–108 | 20 |
| Ọ̀ṢUN | 109–136 | 28 |
| YEMỌJA | 137–164 | 28 |
| ỌYA | 165–196 | 32 |
| ÒGÚN | 197–228 | 32 |
| ỌBÀTÁLÁ | 229–256 | 28 |

`lookup_meta(index)` returns `token_meta` for a 0-based `array_index`. `elemental_signature(mnemonic)` counts Fire, Water, Earth, Air, and Ether associations across a whitespace-separated mnemonic, ignoring unknown tokens for parity with the TypeScript helper. `personality_profile(mnemonic)` validates tokens and returns macro distribution, elemental signature, and dominant Orisha/Macro.

`odu_primary_index(words)` XOR-reduces all word `array_index` values into a single byte. `macro_distribution(words)` counts words per macro. `dominant_macro(words)` returns the highest-count macro, breaking ties by the more concentrated macro (smaller macro size) and then the lowest flat-index range start.

## Security model

- Entropy, seeds, and master-key material are returned or stored in zeroizing containers where applicable.
- Checksum comparisons use constant-time equality.
- Public functions return `Result<_, BiponError>` rather than panicking for user-controlled invalid input.
- Unicode-sensitive seed material is NFKD-normalized for cross-platform determinism.
- The encoding wordlist is protected by a pinned Merkle root and integrity tests.

## Cross-platform guarantees

For the same entropy, mnemonic, passphrase, and derivation mode, compliant implementations must produce identical mnemonic tokens, seeds, master keys, chain codes, Ifáscript macro distributions, and Odù primary indices on every platform.
