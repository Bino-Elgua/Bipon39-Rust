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

pub use derivation::{master_from_seed, DerivationMode, MasterKey};
pub use error::BiponError;
pub use ifascript::{dominant_macro, macro_distribution, odu_primary_index, Macro};
pub use mnemonic::{entropy_to_mnemonic, mnemonic_to_entropy, validate_mnemonic};
pub use seed::mnemonic_to_seed;
pub use wordlist::{
    all_encoding_tokens, entry_by_encoding, entry_by_index, verify_wordlist_integrity, WordlistEntry,
};
