use bipon39::{
    entropy_to_mnemonic, join_mnemonic, master_from_seed, mnemonic_to_entropy, mnemonic_to_seed,
    validate_mnemonic, BiponError, DerivationMode,
};

fn main() -> Result<(), BiponError> {
    let entropy = [0x42u8; 32];
    let mnemonic = entropy_to_mnemonic(&entropy)?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();

    validate_mnemonic(&words)?;
    let decoded = mnemonic_to_entropy(&words)?;
    assert_eq!(&decoded[..], &entropy[..]);

    let seed = mnemonic_to_seed(&words, "àṣẹ")?;
    let native = master_from_seed(&seed, DerivationMode::Native)?;
    let bip32 = master_from_seed(&seed, DerivationMode::Bip32)?;

    println!("Mnemonic: {}", join_mnemonic(&mnemonic));
    println!("Native key: {}", native.key_hex());
    println!("BIP-32 key: {}", bip32.key_hex());
    Ok(())
}
