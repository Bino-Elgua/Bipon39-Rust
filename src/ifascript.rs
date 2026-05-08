use crate::error::BiponError;
use crate::wordlist::{entries_for_macro, entry_by_encoding};

/// The seven Macro groupings of the BIPỌ̀N39 wordlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Macro {
    /// ÈṢÙ — flat indices 1–88.
    Esu,
    /// ṢÀNGÓ — flat indices 89–108.
    Sango,
    /// Ọ̀ṢUN — flat indices 109–136.
    Osun,
    /// YEMỌJA — flat indices 137–164.
    Yemoja,
    /// ỌYA — flat indices 165–196.
    Oya,
    /// ÒGÚN — flat indices 197–228.
    Ogun,
    /// ỌBÀTÁLÁ — flat indices 229–256.
    Obatala,
}

impl Macro {
    /// Canonical display name.
    pub fn name(&self) -> &'static str {
        match self {
            Macro::Esu => "ÈṢÙ",
            Macro::Sango => "ṢÀNGÓ",
            Macro::Osun => "Ọ̀ṢUN",
            Macro::Yemoja => "YEMỌJA",
            Macro::Oya => "ỌYA",
            Macro::Ogun => "ÒGÚN",
            Macro::Obatala => "ỌBÀTÁLÁ",
        }
    }

    /// Inclusive 1-based flat_index range.
    pub fn index_range(&self) -> (usize, usize) {
        match self {
            Macro::Esu => (1, 88),
            Macro::Sango => (89, 108),
            Macro::Osun => (109, 136),
            Macro::Yemoja => (137, 164),
            Macro::Oya => (165, 196),
            Macro::Ogun => (197, 228),
            Macro::Obatala => (229, 256),
        }
    }

    /// Number of tokens in this Macro.
    pub fn count(&self) -> usize {
        let (start, end) = self.index_range();
        end - start + 1
    }

    /// Parse a Macro from canonical or simplified ASCII form.
    pub fn from_name(s: &str) -> Option<Macro> {
        match s {
            "ÈṢÙ" | "esu" => Some(Macro::Esu),
            "ṢÀNGÓ" | "sango" => Some(Macro::Sango),
            "Ọ̀ṢUN" | "osun" => Some(Macro::Osun),
            "YEMỌJA" | "yemoja" => Some(Macro::Yemoja),
            "ỌYA" | "oya" => Some(Macro::Oya),
            "ÒGÚN" | "ogun" => Some(Macro::Ogun),
            "ỌBÀTÁLÁ" | "obatala" => Some(Macro::Obatala),
            _ => None,
        }
    }

    /// Return the Macro containing a 1-based flat_index.
    pub fn from_flat_index(flat_index: usize) -> Option<Macro> {
        Self::all().into_iter().find(|macro_| {
            let (start, end) = macro_.index_range();
            (start..=end).contains(&flat_index)
        })
    }

    fn all() -> [Macro; 7] {
        [
            Macro::Esu,
            Macro::Sango,
            Macro::Osun,
            Macro::Yemoja,
            Macro::Oya,
            Macro::Ogun,
            Macro::Obatala,
        ]
    }
}

/// Distribution of mnemonic words across Macros.
pub struct MacroDistribution {
    /// Counts in Macro order.
    pub counts: [(Macro, usize); 7],
    /// Sum of all counts.
    pub total: usize,
}

/// XOR-reduce all word array_indices.
pub fn odu_primary_index(words: &[&str]) -> Result<u8, BiponError> {
    let mut result = 0u8;
    for word in words {
        result ^= entry_by_encoding(word)?.array_index as u8;
    }
    Ok(result)
}

/// Count how many words belong to each Macro.
pub fn macro_distribution(words: &[&str]) -> Result<MacroDistribution, BiponError> {
    let mut counts = Macro::all().map(|macro_| (macro_, 0usize));
    for word in words {
        let entry = entry_by_encoding(word)?;
        let macro_ = Macro::from_name(entry.macro_name).ok_or_else(|| {
            BiponError::WordlistIntegrity(format!("unknown macro {}", entry.macro_name))
        })?;
        let (_, count) = counts
            .iter_mut()
            .find(|(candidate, _)| *candidate == macro_)
            .expect("all macros are present in counts");
        *count += 1;
    }
    Ok(MacroDistribution {
        counts,
        total: words.len(),
    })
}

/// Return the Macro with the highest word count.
pub fn dominant_macro(words: &[&str]) -> Result<Macro, BiponError> {
    let distribution = macro_distribution(words)?;
    Ok(distribution
        .counts
        .into_iter()
        .max_by_key(|(macro_, count)| (*count, usize::MAX - macro_.index_range().0))
        .map(|(macro_, _)| macro_)
        .unwrap_or(Macro::Esu))
}

/// Entries for a Macro.
pub fn entries_for(macro_: Macro) -> Vec<&'static crate::wordlist::WordlistEntry> {
    entries_for_macro(macro_.name())
}
