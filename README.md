# BIPỌ̀N39 Rust

BIPỌ̀N39 is a sovereign Base-256 mnemonic library with Yorùbá/Ifá roots for the Ọmọ Kọ́dà ecosystem. It maps entropy to a fixed 256-token encoding layer, derives deterministic seeds and master keys, and exposes Ifáscript metadata over the same immutable wordlist.

The cryptographic layer always uses lowercase ASCII encoding tokens. Yorùbá canonical forms are display-only and never enter hashes, PBKDF2 password input, or Merkle leaves.

## Installation

```bash
cargo add bipon39
```

Until the crate is published, use the git repository directly:

```toml
[dependencies]
bipon39 = { git = "https://github.com/Bino-Elgua/Bipon39-Rust" }
```

## Entropy → mnemonic

```rust
use bipon39::{entropy_to_mnemonic, join_mnemonic, BiponError};

fn main() -> Result<(), BiponError> {
    let entropy = [0u8; 32];
    let mnemonic = entropy_to_mnemonic(&entropy)?;

    println!("{}", join_mnemonic(&mnemonic));
    Ok(())
}
```

## Mnemonic → seed

```rust
use bipon39::{entropy_to_mnemonic, mnemonic_to_seed, BiponError};

fn main() -> Result<(), BiponError> {
    let mnemonic = entropy_to_mnemonic(&[0u8; 32])?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    let seed = mnemonic_to_seed(&words, "àṣẹ")?;

    println!("seed bytes: {}", seed.len());
    Ok(())
}
```

## Seed → master key (Native + BIP-32)

```rust
use bipon39::{entropy_to_mnemonic, master_from_seed, mnemonic_to_seed, BiponError, DerivationMode};

fn main() -> Result<(), BiponError> {
    let mnemonic = entropy_to_mnemonic(&[0u8; 32])?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    let seed = mnemonic_to_seed(&words, "")?;

    let native = master_from_seed(&seed, DerivationMode::Native)?;
    let bip32 = master_from_seed(&seed, DerivationMode::Bip32)?;

    println!("native key: {}", native.key_hex());
    println!("native chain: {}", native.chain_code_hex());
    println!("bip32 key: {}", bip32.key_hex());
    println!("bip32 chain: {}", bip32.chain_code_hex());
    Ok(())
}
```

## Ifáscript lookup

```rust
use bipon39::{
    dominant_macro, elemental_signature, entropy_to_mnemonic, macro_distribution,
    odu_primary_index, BiponError,
};

fn main() -> Result<(), BiponError> {
    let mnemonic = entropy_to_mnemonic(&[0u8; 32])?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    let phrase = mnemonic.join(" ");

    let odu = odu_primary_index(&words)?;
    let dominant = dominant_macro(&words)?;
    let distribution = macro_distribution(&words)?;
    let elements = elemental_signature(&phrase);

    println!("Odù primary index: {odu}");
    println!("Dominant macro: {}", dominant.name());
    println!("Word count: {}", distribution.total);
    println!("Fire tokens: {}", elements.fire);
    Ok(())
}
```

## Dual-mode 256 ↔ 2048 conversion

BIPỌ̀N39 keeps its 256-token encoding layer immutable while supporting the
TypeScript reference's 2048-mode expansion. The expanded mode uses the same base
tokens plus eight deterministic subtones: `alpha`, `beta`, `gamma`, `delta`,
`epsilon`, `zeta`, `eta`, and `theta`.

```rust
use bipon39::{decode_2048, encode_2048, entropy_to_mnemonic, BiponError};

fn main() -> Result<(), BiponError> {
    let mnemonic_256 = entropy_to_mnemonic(&[0u8; 16])?.join(" ");
    let mnemonic_2048 = encode_2048(&mnemonic_256)?;
    let recovered_256 = decode_2048(&mnemonic_2048)?;

    assert_eq!(recovered_256, mnemonic_256);
    Ok(())
}
```

## Token metadata lookup

Token metadata is loaded from `data/canonical.json` at compile time alongside the
canonical/encoding wordlist. The lookup API uses the same 0-based array index as
the byte-oriented mnemonic layer.

```rust
use bipon39::lookup_meta;

fn main() {
    let meta = lookup_meta(15).expect("token 15 exists");

    assert_eq!(meta.element, "Ether");
    assert_eq!(meta.ritual_cue, "face sunrise");
    assert_eq!(meta.ethical_tag, "begin");
    assert_eq!(meta.sigil_seed, "east-ray");
}
```

## Personality profile

`personality_profile` combines macro distribution, elemental signature, and a
dominant Orisha/Macro selected with deterministic tie-breaking: highest count,
then the more concentrated Macro, then lower flat-index range.

```rust
use bipon39::{personality_profile, BiponError};

fn main() -> Result<(), BiponError> {
    let profile = personality_profile("esu-elegbara esu-elegba sango")?;

    println!("Dominant Orisha: {}", profile.dominant_orisha.name());
    println!("Earth balance: {}", profile.elemental_signature.earth);
    println!("Macro tokens: {}", profile.macro_distribution.total);
    println!("ÈṢÙ percentage: {:.1}%", profile.macro_percentages[0].1);
    println!("First ritual: {}", profile.ritual_suggestions[0]);
    println!("Summary: {}", profile.personality_summary);
    Ok(())
}
```

## CLI

Install or run the binary with Cargo:

```bash
cargo run -- generate 256
cargo run -- generate 128 --mode 2048
cargo run -- convert --to-2048 "<256-mode mnemonic>"
cargo run -- convert --to-256 "<2048-mode mnemonic>"
cargo run -- inspect "esu-elegbara esu-elegba sango"
cargo run -- seed "<mnemonic>" --passphrase "àṣẹ"
```

After installation, replace `cargo run --` with `bipon39`:

```bash
bipon39 generate 256
bipon39 inspect esu-elegbara esu-elegba sango
bipon39 convert --to-2048 "$(bipon39 generate 128)"
```

## Stable wordlist Merkle root

The 256-token encoding wordlist is pinned by this SHA-256 binary Merkle root:

```text
fd49f820efba401dc2f53a17411517476e20ba2494c5207cbaf1960369e43d14
```

## Test vectors

| ID | Entropy | Passphrase | Words | Seed pinned | Master keys pinned |
|---|---:|---|---:|---|---|
| v001 | 128-bit all-zero | empty | 17 | yes | no |
| v002 | 160-bit all-zero | empty | 21 | yes | no |
| v003 | 192-bit all-zero | empty | 25 | yes | no |
| v004 | 224-bit all-zero | empty | 29 | yes | no |
| v005 | 256-bit all-zero | empty | 33 | yes | Native + BIP-32 |
| v006 | 256-bit all-zero | `àṣẹ` | 33 | yes | no |

The full machine-readable vectors live in [`vectors/test_vectors.json`](vectors/test_vectors.json).
Regenerate them from the current library implementation with:

```bash
cargo run --example regenerate_vectors > vectors/test_vectors.json
```

## Examples

Runnable examples live in [`examples/`](examples/):

- `basic_usage.rs`
- `ifascript_demo.rs`
- `full_roundtrip.rs`
- `metadata_demo.rs`
- `regenerate_vectors.rs`

Run one with:

```bash
cargo run --example basic_usage
```

## Verification

```bash
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt --check
cargo doc
cargo bench
```

## Documentation

- [`SPEC.md`](SPEC.md) — formal algorithm and security specification.
- [`AGENT_CHECKLIST.md`](AGENT_CHECKLIST.md) — phased build checklist and audit session log.

## License

MIT
