use bipon39::display::{
    canonical_for_encoding, canonical_to_encoding, encoding_for_canonical, format_numbered,
    format_numbered_canonical, mnemonic_to_canonical,
};
use bipon39::ifascript::{dominant_macro, macro_distribution, odu_primary_index, Macro};
use bipon39::wordlist::entries_for_macro;

#[test]
fn odu_index_in_range() {
    let words = ["esu-elegbara", "sango", "osun", "yemoja"];
    let index: u8 = odu_primary_index(&words).unwrap();
    assert_eq!(index, 188);
}

#[test]
fn odu_index_deterministic() {
    let words = ["esu-elegbara", "sango", "osun", "yemoja"];
    assert_eq!(
        odu_primary_index(&words).unwrap(),
        odu_primary_index(&words).unwrap()
    );
}

#[test]
fn odu_xor_correctness() {
    let words = ["esu-elegbara", "sango", "osun"];
    assert_eq!(odu_primary_index(&words).unwrap(), 0 ^ 88 ^ 108);
}

#[test]
fn macro_distribution_known_mnemonic() {
    let words = [
        "esu-elegbara",
        "esu-elegba",
        "sango",
        "osun",
        "yemoja",
        "oya",
        "ogun",
        "obatala",
    ];
    let distribution = macro_distribution(&words).unwrap();
    assert_eq!(distribution.total, words.len());
    assert_eq!(
        distribution.counts,
        [
            (Macro::Esu, 2),
            (Macro::Sango, 1),
            (Macro::Osun, 1),
            (Macro::Yemoja, 1),
            (Macro::Oya, 1),
            (Macro::Ogun, 1),
            (Macro::Obatala, 1),
        ]
    );
}

#[test]
fn dominant_macro_correct() {
    let words = ["sango", "sango-oba-oke", "esu-elegbara"];
    assert_eq!(dominant_macro(&words).unwrap(), Macro::Sango);
}

#[test]
fn dominant_macro_tie_breaks_by_lower_index() {
    let words = ["sango", "osun"];
    assert_eq!(dominant_macro(&words).unwrap(), Macro::Sango);
}

#[test]
fn canonical_encoding_roundtrip() {
    let canonical = canonical_for_encoding("esu-elegbara").unwrap();
    assert_eq!(canonical, "èṣù-elegbára");
    assert_eq!(encoding_for_canonical(canonical).unwrap(), "esu-elegbara");
}

#[test]
fn mnemonic_canonical_conversion_roundtrip() {
    let encoding = ["esu-elegbara", "sango", "osun"];
    let canonical = mnemonic_to_canonical(&encoding).unwrap();
    assert_eq!(canonical, vec!["èṣù-elegbára", "ṣàngó", "ọ̀ṣun"]);
    assert_eq!(canonical_to_encoding(&canonical).unwrap(), encoding);
}

#[test]
fn numbered_format_helpers() {
    let words = ["esu-elegbara", "sango"];
    assert_eq!(format_numbered(&words), "1. esu-elegbara  2. sango");
    assert_eq!(
        format_numbered_canonical(&words).unwrap(),
        "1. èṣù-elegbára  2. ṣàngó"
    );
}

#[test]
fn macro_from_flat_index() {
    assert_eq!(Macro::from_flat_index(1), Some(Macro::Esu));
    assert_eq!(Macro::from_flat_index(89), Some(Macro::Sango));
    assert_eq!(Macro::from_flat_index(109), Some(Macro::Osun));
    assert_eq!(Macro::from_flat_index(137), Some(Macro::Yemoja));
    assert_eq!(Macro::from_flat_index(165), Some(Macro::Oya));
    assert_eq!(Macro::from_flat_index(197), Some(Macro::Ogun));
    assert_eq!(Macro::from_flat_index(229), Some(Macro::Obatala));
}

#[test]
fn macro_from_flat_index_out_of_range() {
    assert_eq!(Macro::from_flat_index(0), None);
    assert_eq!(Macro::from_flat_index(257), None);
}

#[test]
fn macro_from_name_accepts_canonical_and_ascii() {
    assert_eq!(Macro::from_name("ÈṢÙ"), Some(Macro::Esu));
    assert_eq!(Macro::from_name("esu"), Some(Macro::Esu));
    assert_eq!(Macro::from_name("ṢÀNGÓ"), Some(Macro::Sango));
    assert_eq!(Macro::from_name("sango"), Some(Macro::Sango));
    assert_eq!(Macro::from_name("not-a-macro"), None);
}

#[test]
fn macro_ranges_and_counts() {
    assert_eq!(Macro::Esu.name(), "ÈṢÙ");
    assert_eq!(Macro::Esu.index_range(), (1, 88));
    assert_eq!(Macro::Esu.count(), 88);
    assert_eq!(Macro::Obatala.name(), "ỌBÀTÁLÁ");
    assert_eq!(Macro::Obatala.index_range(), (229, 256));
    assert_eq!(Macro::Obatala.count(), 28);
}

#[test]
fn entries_for_macro_counts() {
    let expected = [
        (Macro::Esu, 88usize),
        (Macro::Sango, 20),
        (Macro::Osun, 28),
        (Macro::Yemoja, 28),
        (Macro::Oya, 32),
        (Macro::Ogun, 32),
        (Macro::Obatala, 28),
    ];
    for (macro_, count) in expected {
        assert_eq!(
            entries_for_macro(macro_.name()).len(),
            count,
            "{:?}",
            macro_
        );
    }
}
