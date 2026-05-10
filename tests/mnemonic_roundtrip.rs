use bipon39::error::BiponError;
use bipon39::mnemonic::{entropy_to_mnemonic, mnemonic_to_entropy, validate_mnemonic};
use bipon39::wordlist::{entry_by_index, index_of_encoding};

fn deterministic_entropy(bytes: usize) -> Vec<u8> {
    (0..bytes)
        .map(|index| (index as u8).wrapping_mul(37).wrapping_add(11))
        .collect()
}

fn roundtrip(entropy: &[u8]) {
    let mnemonic = entropy_to_mnemonic(entropy).unwrap();
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    let decoded = mnemonic_to_entropy(&words).unwrap();
    assert_eq!(&decoded[..], entropy);
}

fn checksum_corrupted_words() -> Vec<String> {
    let mut words = entropy_to_mnemonic(&[0u8; 16]).unwrap();
    let last = words.last_mut().unwrap();
    let index = index_of_encoding(last).unwrap();
    let corrupted_index = index ^ 0x10;
    *last = entry_by_index(corrupted_index + 1)
        .unwrap()
        .encoding
        .to_string();
    words
}

#[test]
fn roundtrip_128_bit() {
    roundtrip(&deterministic_entropy(16));
}

#[test]
fn roundtrip_160_bit() {
    roundtrip(&deterministic_entropy(20));
}

#[test]
fn roundtrip_192_bit() {
    roundtrip(&deterministic_entropy(24));
}

#[test]
fn roundtrip_224_bit() {
    roundtrip(&deterministic_entropy(28));
}

#[test]
fn roundtrip_256_bit() {
    roundtrip(&deterministic_entropy(32));
}

#[test]
fn all_zeros_128_bit() {
    roundtrip(&[0u8; 16]);
}

#[test]
fn all_zeros_256_bit() {
    roundtrip(&[0u8; 32]);
}

#[test]
fn all_ff_256_bit() {
    roundtrip(&[0xFFu8; 32]);
}

#[test]
fn wrong_word_count_err() {
    let words = vec!["esu-elegbara"; 16];
    assert_eq!(
        mnemonic_to_entropy(&words).unwrap_err(),
        BiponError::InvalidMnemonicLength { words: 16 }
    );
}

#[test]
fn wrong_word_count_zero_err() {
    assert_eq!(
        mnemonic_to_entropy(&[]).unwrap_err(),
        BiponError::InvalidMnemonicLength { words: 0 }
    );
}

#[test]
fn unknown_word_err() {
    let mut mnemonic = entropy_to_mnemonic(&[0u8; 16]).unwrap();
    mnemonic[3] = "xyz-fake".to_string();
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        mnemonic_to_entropy(&words).unwrap_err(),
        BiponError::InvalidWord {
            position: 3,
            word: "xyz-fake".to_string()
        }
    );
}

#[test]
fn corrupted_checksum_err() {
    let mnemonic = checksum_corrupted_words();
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        mnemonic_to_entropy(&words).unwrap_err(),
        BiponError::ChecksumMismatch
    );
}

#[test]
fn nonzero_pad_bits_err() {
    let mut mnemonic = entropy_to_mnemonic(&[0u8; 20]).unwrap();
    let last = mnemonic.last_mut().unwrap();
    let index = index_of_encoding(last).unwrap();
    *last = entry_by_index(index + 2).unwrap().encoding.to_string();

    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        mnemonic_to_entropy(&words).unwrap_err(),
        BiponError::NonZeroPadding
    );
}

#[test]
fn validate_mnemonic_passes_for_valid() {
    let mnemonic = entropy_to_mnemonic(&deterministic_entropy(32)).unwrap();
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    validate_mnemonic(&words).unwrap();
}

#[test]
fn validate_mnemonic_fails_for_bad_checksum() {
    let mnemonic = checksum_corrupted_words();
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        validate_mnemonic(&words).unwrap_err(),
        BiponError::ChecksumMismatch
    );
}
