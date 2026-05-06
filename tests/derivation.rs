use bipon39::{master_from_seed, DerivationMode};
use serde::Deserialize;

const V005_SEED_HEX: &str = "68b11e76147d7d3436015103d3a4f6fd5aa87834d4139e57e91d6467391ada62a489201ed50cae2260e39d0ba813600a6dbfa75cac9f31653f15ae739c202f1e";

#[derive(Debug, Deserialize)]
struct TestVectors {
    vectors: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
struct TestVector {
    id: String,
    seed_hex: Option<String>,
    master_key_native_hex: Option<String>,
    master_chain_native_hex: Option<String>,
    master_key_bip32_hex: Option<String>,
    master_chain_bip32_hex: Option<String>,
}

fn v005_seed() -> Vec<u8> {
    hex::decode(V005_SEED_HEX).unwrap()
}

fn v005_vector() -> TestVector {
    let parsed: TestVectors =
        serde_json::from_str(include_str!("../vectors/test_vectors.json")).unwrap();
    parsed
        .vectors
        .into_iter()
        .find(|vector| vector.id == "v005")
        .unwrap()
}

#[test]
fn native_mode_deterministic() {
    let seed = v005_seed();
    let first = master_from_seed(&seed, DerivationMode::Native).unwrap();
    let second = master_from_seed(&seed, DerivationMode::Native).unwrap();
    assert_eq!(first.key_hex(), second.key_hex());
    assert_eq!(first.chain_code_hex(), second.chain_code_hex());
}

#[test]
fn bip32_mode_deterministic() {
    let seed = v005_seed();
    let first = master_from_seed(&seed, DerivationMode::Bip32).unwrap();
    let second = master_from_seed(&seed, DerivationMode::Bip32).unwrap();
    assert_eq!(first.key_hex(), second.key_hex());
    assert_eq!(first.chain_code_hex(), second.chain_code_hex());
}

#[test]
fn native_ne_bip32() {
    let seed = v005_seed();
    let native = master_from_seed(&seed, DerivationMode::Native).unwrap();
    let bip32 = master_from_seed(&seed, DerivationMode::Bip32).unwrap();
    assert_ne!(native.key_hex(), bip32.key_hex());
    assert_ne!(native.chain_code_hex(), bip32.chain_code_hex());
}

#[test]
fn key_is_32_bytes() {
    let seed = v005_seed();
    let master = master_from_seed(&seed, DerivationMode::Native).unwrap();
    assert_eq!(master.key.len(), 32);
}

#[test]
fn chain_code_is_32_bytes() {
    let seed = v005_seed();
    let master = master_from_seed(&seed, DerivationMode::Native).unwrap();
    assert_eq!(master.chain_code.len(), 32);
}

#[test]
fn pinned_native_vector() {
    let vector = v005_vector();
    assert_eq!(vector.seed_hex.as_deref(), Some(V005_SEED_HEX));
    let seed = hex::decode(vector.seed_hex.unwrap()).unwrap();
    let master = master_from_seed(&seed, DerivationMode::Native).unwrap();
    assert_eq!(master.key_hex(), vector.master_key_native_hex.unwrap());
    assert_eq!(
        master.chain_code_hex(),
        vector.master_chain_native_hex.unwrap()
    );
}

#[test]
fn pinned_bip32_vector() {
    let vector = v005_vector();
    assert_eq!(vector.seed_hex.as_deref(), Some(V005_SEED_HEX));
    let seed = hex::decode(vector.seed_hex.unwrap()).unwrap();
    let master = master_from_seed(&seed, DerivationMode::Bip32).unwrap();
    assert_eq!(master.key_hex(), vector.master_key_bip32_hex.unwrap());
    assert_eq!(
        master.chain_code_hex(),
        vector.master_chain_bip32_hex.unwrap()
    );
}
