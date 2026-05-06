use bipon39::mnemonic::{entropy_to_mnemonic, mnemonic_to_entropy};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestVectors {
    vectors: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
struct TestVector {
    id: String,
    entropy_hex: String,
    entropy_bits: usize,
    mnemonic: Vec<String>,
    mnemonic_phrase: Option<String>,
    word_count: usize,
}

#[test]
fn pinned_mnemonic_vectors_roundtrip() {
    let parsed: TestVectors = serde_json::from_str(include_str!("../vectors/test_vectors.json")).unwrap();
    assert_eq!(parsed.vectors.len(), 5);

    for vector in parsed.vectors {
        let entropy = hex::decode(&vector.entropy_hex).unwrap();
        assert_eq!(entropy.len() * 8, vector.entropy_bits, "{}", vector.id);
        assert_eq!(vector.mnemonic.len(), vector.word_count, "{}", vector.id);
        if let Some(phrase) = &vector.mnemonic_phrase {
            assert_eq!(phrase, &vector.mnemonic.join(" "), "{}", vector.id);
        }

        assert_eq!(entropy_to_mnemonic(&entropy).unwrap(), vector.mnemonic, "{}", vector.id);

        let words = vector.mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
        let decoded = mnemonic_to_entropy(&words).unwrap();
        assert_eq!(&decoded[..], entropy.as_slice(), "{}", vector.id);
    }
}
