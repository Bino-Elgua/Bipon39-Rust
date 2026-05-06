use bipon39::{entropy_to_mnemonic, mnemonic_to_seed};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestVectors {
    vectors: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
struct TestVector {
    id: String,
    passphrase: String,
    mnemonic: Vec<String>,
    seed_hex: Option<String>,
}

fn all_zero_256_words() -> Vec<String> {
    entropy_to_mnemonic(&[0u8; 32]).unwrap()
}

fn as_word_refs(words: &[String]) -> Vec<&str> {
    words.iter().map(String::as_str).collect()
}

fn load_vectors() -> TestVectors {
    serde_json::from_str(include_str!("../vectors/test_vectors.json")).unwrap()
}

#[test]
fn empty_passphrase_deterministic() {
    let mnemonic = all_zero_256_words();
    let words = as_word_refs(&mnemonic);
    let first = mnemonic_to_seed(&words, "").unwrap();
    let second = mnemonic_to_seed(&words, "").unwrap();
    assert_eq!(&first[..], &second[..]);
}

#[test]
fn passphrase_changes_seed() {
    let mnemonic = all_zero_256_words();
    let words = as_word_refs(&mnemonic);
    let first = mnemonic_to_seed(&words, "àṣẹ").unwrap();
    let second = mnemonic_to_seed(&words, "different").unwrap();
    assert_ne!(&first[..], &second[..]);
}

#[test]
fn empty_vs_nonempty_passphrase_differ() {
    let mnemonic = all_zero_256_words();
    let words = as_word_refs(&mnemonic);
    let empty = mnemonic_to_seed(&words, "").unwrap();
    let nonempty = mnemonic_to_seed(&words, "a").unwrap();
    assert_ne!(&empty[..], &nonempty[..]);
}

#[test]
fn output_is_64_bytes() {
    let mnemonic = all_zero_256_words();
    let words = as_word_refs(&mnemonic);
    let seed = mnemonic_to_seed(&words, "").unwrap();
    assert_eq!(seed.len(), 64);
}

#[test]
fn nfkd_normalization_applied() {
    let mnemonic = all_zero_256_words();
    let words = as_word_refs(&mnemonic);
    let precomposed = mnemonic_to_seed(&words, "á").unwrap();
    let decomposed = mnemonic_to_seed(&words, "a\u{0301}").unwrap();
    assert_eq!(&precomposed[..], &decomposed[..]);
}

#[test]
fn pinned_vector_no_passphrase() {
    let vectors = load_vectors();
    let vector = vectors
        .vectors
        .iter()
        .find(|vector| vector.id == "v005")
        .unwrap();
    assert_eq!(vector.passphrase, "");
    let words = as_word_refs(&vector.mnemonic);
    let seed = mnemonic_to_seed(&words, &vector.passphrase).unwrap();
    assert_eq!(
        hex::encode(&seed[..]),
        vector.seed_hex.as_ref().unwrap().as_str()
    );
}

#[test]
fn pinned_vector_with_passphrase() {
    let vectors = load_vectors();
    let vector = vectors
        .vectors
        .iter()
        .find(|vector| vector.id == "v006")
        .unwrap();
    assert_eq!(vector.passphrase, "àṣẹ");
    let words = as_word_refs(&vector.mnemonic);
    let seed = mnemonic_to_seed(&words, &vector.passphrase).unwrap();
    assert_eq!(
        hex::encode(&seed[..]),
        vector.seed_hex.as_ref().unwrap().as_str()
    );
}
