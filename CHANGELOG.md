# Changelog

## v0.1.1 — 2026-05-10

- Regenerated pinned mnemonic/seed/master-key test vectors from the current canonical implementation.
- Added `examples/regenerate_vectors.rs` so vectors can be reproduced with a single Cargo command.
- Added explicit zero-padding validation when decoding 256-mode mnemonics.
- Stored PBKDF2 salt constants in NFKD form and documented the runtime normalization contract.
- Added regression coverage for non-zero padding rejection.

## v0.1.0 — 2026-05-09

- Initial production-ready BIPỌ̀N39 Rust library.
- Added immutable 256-token Base-256 encoding wordlist with pinned Merkle root.
- Added entropy-to-mnemonic encoding and mnemonic-to-entropy validation/decoding.
- Added PBKDF2-HMAC-SHA512 seed derivation with mandatory NFKD normalization.
- Added Native and BIP-32-compatible master key derivation.
- Added Ifáscript macro distribution, Odù primary index, and display-layer conversion helpers.
- Added pinned mnemonic, seed, and master-key test vectors.
- Added Criterion benchmarks, runnable examples, CI workflow, README, and formal SPEC documentation.
