use bipon39::mnemonic::{entropy_to_mnemonic, mnemonic_to_entropy};
use bipon39::{master_from_seed, mnemonic_to_seed, DerivationMode};
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
    passphrase: String,
    mnemonic: Vec<String>,
    mnemonic_phrase: Option<String>,
    word_count: usize,
    seed_hex: Option<String>,
    master_key_native_hex: Option<String>,
    master_chain_native_hex: Option<String>,
    master_key_bip32_hex: Option<String>,
    master_chain_bip32_hex: Option<String>,
}

#[test]
fn pinned_mnemonic_vectors_roundtrip() {
    let parsed: TestVectors =
        serde_json::from_str(include_str!("../vectors/test_vectors.json")).unwrap();
    assert_eq!(parsed.vectors.len(), 6);

    for vector in parsed.vectors {
        let entropy = hex::decode(&vector.entropy_hex).unwrap();
        assert_eq!(entropy.len() * 8, vector.entropy_bits, "{}", vector.id);
        assert_eq!(vector.mnemonic.len(), vector.word_count, "{}", vector.id);
        if let Some(phrase) = &vector.mnemonic_phrase {
            assert_eq!(phrase, &vector.mnemonic.join(" "), "{}", vector.id);
        }

        assert_eq!(
            entropy_to_mnemonic(&entropy).unwrap(),
            vector.mnemonic,
            "{}",
            vector.id
        );

        let words = vector
            .mnemonic
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let decoded = mnemonic_to_entropy(&words).unwrap();
        assert_eq!(&decoded[..], entropy.as_slice(), "{}", vector.id);

        if let Some(seed_hex) = &vector.seed_hex {
            let seed = mnemonic_to_seed(&words, &vector.passphrase).unwrap();
            assert_eq!(hex::encode(&seed[..]), *seed_hex, "{}", vector.id);

            if let (Some(master_key), Some(master_chain)) = (
                &vector.master_key_native_hex,
                &vector.master_chain_native_hex,
            ) {
                let master = master_from_seed(&seed, DerivationMode::Native).unwrap();
                assert_eq!(master.key_hex(), *master_key, "{}", vector.id);
                assert_eq!(master.chain_code_hex(), *master_chain, "{}", vector.id);
            }

            if let (Some(master_key), Some(master_chain)) =
                (&vector.master_key_bip32_hex, &vector.master_chain_bip32_hex)
            {
                let master = master_from_seed(&seed, DerivationMode::Bip32).unwrap();
                assert_eq!(master.key_hex(), *master_key, "{}", vector.id);
                assert_eq!(master.chain_code_hex(), *master_chain, "{}", vector.id);
            }
        }
    }
}
