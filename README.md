# BIPỌ̀N39 Rust

BIPỌ̀N39 is a sovereign Base-256 mnemonic library for the Ọmọ Kọ́dà ecosystem. It converts raw entropy into 256-token mnemonic phrases, validates and decodes those phrases, derives deterministic 64-byte seeds, derives Native and BIP-32-compatible master keys, and exposes Ifáscript metadata over a fixed Yorùbá-rooted wordlist.

The cryptographic layer always uses the ASCII encoding tokens. The canonical Yorùbá forms are for display and documentation only.

## Install

Add the crate to a Rust project once it is published or consumed by git:

```toml
[dependencies]
bipon39 = { git = "https://github.com/Bino-Elgua/Bipon39-Rust" }
```

## Quick start

```rust
use bipon39::{
    dominant_macro, entropy_to_mnemonic, join_mnemonic, master_from_seed, mnemonic_to_seed,
    DerivationMode,
};

let entropy = [0u8; 32];
let mnemonic = entropy_to_mnemonic(&entropy)?;
let phrase = join_mnemonic(&mnemonic);

let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
let seed = mnemonic_to_seed(&words, "")?;
let native = master_from_seed(&seed, DerivationMode::Native)?;
let macro_ = dominant_macro(&words)?;

println!("phrase: {phrase}");
println!("native key: {}", native.key_hex());
println!("dominant macro: {}", macro_.name());
# Ok::<(), bipon39::BiponError>(())
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

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo doc --no-deps
cargo bench
```

## Documentation

- [`SPEC.md`](SPEC.md) — formal algorithm and security specification.
- [`AGENT_CHECKLIST.md`](AGENT_CHECKLIST.md) — phased build checklist and audit session log.

## License

MIT
