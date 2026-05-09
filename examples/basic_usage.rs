use bipon39::{entropy_to_mnemonic, join_mnemonic, mnemonic_to_seed, BiponError};

fn main() -> Result<(), BiponError> {
    let entropy = [0u8; 32];
    let mnemonic = entropy_to_mnemonic(&entropy)?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    let seed = mnemonic_to_seed(&words, "")?;

    println!("Mnemonic: {}", join_mnemonic(&mnemonic));
    println!("Seed length: {} bytes", seed.len());
    Ok(())
}
