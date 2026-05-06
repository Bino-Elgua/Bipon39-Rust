use std::collections::HashSet;

use bipon39::constants::{MERKLE_ROOT, WORDLIST_SIZE};
use bipon39::crypto::compute_wordlist_merkle_root;
use bipon39::wordlist::{
    all_encoding_tokens, entries_for_macro, entry_by_index, verify_wordlist_integrity,
};

#[test]
fn loads_256_entries() {
    assert_eq!(all_encoding_tokens().len(), WORDLIST_SIZE);
}

#[test]
fn macro_counts_correct() {
    let expected = [
        ("ÈṢÙ", 88usize),
        ("ṢÀNGÓ", 20),
        ("Ọ̀ṢUN", 28),
        ("YEMỌJA", 28),
        ("ỌYA", 32),
        ("ÒGÚN", 32),
        ("ỌBÀTÁLÁ", 28),
    ];
    for (macro_name, count) in expected {
        assert_eq!(entries_for_macro(macro_name).len(), count, "{macro_name}");
    }
}

#[test]
fn flat_index_sequence() {
    for index in 1..=WORDLIST_SIZE {
        assert_eq!(entry_by_index(index).unwrap().flat_index, index);
    }
}

#[test]
fn array_index_equals_flat_minus_one() {
    for index in 1..=WORDLIST_SIZE {
        let entry = entry_by_index(index).unwrap();
        assert_eq!(entry.array_index, entry.flat_index - 1);
    }
}

#[test]
fn no_duplicate_encoding_tokens() {
    let mut seen = HashSet::new();
    for token in all_encoding_tokens() {
        assert!(seen.insert(*token), "duplicate encoding token {token}");
    }
}

#[test]
fn no_duplicate_canonical_tokens() {
    let mut seen = HashSet::new();
    for index in 1..=WORDLIST_SIZE {
        let token = entry_by_index(index).unwrap().canonical;
        assert!(seen.insert(token), "duplicate canonical token {token}");
    }
}

#[test]
fn encoding_regex_compliance() {
    for token in all_encoding_tokens() {
        let mut chars = token.chars();
        assert!(matches!(chars.next(), Some('a'..='z')), "{token}");
        assert!(
            chars.all(|ch| matches!(ch, 'a'..='z' | '0'..='9' | '-')),
            "{token}"
        );
    }
}

#[test]
fn boundary_spot_checks() {
    let expected = [
        (1, "esu-elegbara", "èṣù-elegbára"),
        (88, "esu-oluso-ona", "èṣù-olùṣọ́-ọ̀nà"),
        (89, "sango", "ṣàngó"),
        (108, "sango-oba-oke", "ṣàngó-ọba-òkè"),
        (109, "osun", "ọ̀ṣun"),
        (136, "osun-inu", "ọ̀ṣun-inú"),
        (137, "yemoja", "yemọja"),
        (164, "yemoja-isodotun-okun", "yemọja-ìsọdọtun-òkun"),
        (165, "oya", "ọya"),
        (196, "oya-alade", "ọya-aládé"),
        (197, "ogun", "ògún"),
        (228, "ogun-alade", "ògún-aládé"),
        (229, "obatala", "ọbàtálá"),
        (256, "obatala-alade", "ọbàtálá-aládé"),
    ];

    for (index, encoding, canonical) in expected {
        let entry = entry_by_index(index).unwrap();
        println!("{index}: {} / {}", entry.encoding, entry.canonical);
        assert_eq!(entry.encoding, encoding);
        assert_eq!(entry.canonical, canonical);
    }
}

#[test]
fn merkle_root_matches_pinned_constant() {
    assert!(!MERKLE_ROOT.is_empty(), "MERKLE_ROOT must be pinned");
    assert_eq!(compute_wordlist_merkle_root(), MERKLE_ROOT);
}

#[test]
fn verify_wordlist_integrity_passes() {
    verify_wordlist_integrity().unwrap();
}
