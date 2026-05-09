use bipon39::{
    dominant_macro, entropy_to_mnemonic, macro_distribution, odu_primary_index, BiponError,
};

fn main() -> Result<(), BiponError> {
    let mnemonic = entropy_to_mnemonic(&[0u8; 32])?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();

    let odu = odu_primary_index(&words)?;
    let dominant = dominant_macro(&words)?;
    let distribution = macro_distribution(&words)?;

    println!("Odù primary index: {odu}");
    println!("Dominant macro: {}", dominant.name());
    println!("Distribution:");
    for (macro_, count) in distribution.counts {
        println!("  {}: {count}", macro_.name());
    }

    Ok(())
}
