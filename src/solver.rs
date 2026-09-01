//! Countdown letters-round solver.
//!
//! Given a rack of letters, find every dictionary word that can be spelled
//! using a subset of those letters (each letter used at most as many times as
//! it appears in the rack), grouped by word length.

use wordfreq_model::load_wordfreq;
use wordfreq_model::ModelKind;



use std::collections::BTreeMap;

/// The list of valid words to search against.
///
/// 

//const WORD_LIST: &[&str] = &[];
const WORD_LIST: &[&str] = &load_wordfreq(ModelKind::English).words;

/// Search for all words that can be built from `letters`.
///
/// `letters` is the raw rack as typed by the player: case is ignored and any
/// non-alphabetic characters are ignored.
///
/// Returns a map from word length to the list of words of that length that can
/// be made from the rack. Lengths with no matches are omitted.
pub fn find_words(letters: &str) -> BTreeMap<usize, Vec<String>> {
    // Multiset of available letters, indexed by `0 == 'a'` .. `25 == 'z'`.
    let rack: [u8; 26] = letter_counts(letters);

    let mut results: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for &word in WORD_LIST {
        if can_spell(word, &rack) {
            results
                .entry(word.chars().count())
                .or_default()
                .push(word.to_owned());
        }
    }

    results
}

/// Count how many times each ASCII letter (`a`-`z`, case-insensitive) appears in
/// `s`. Non-letters are ignored.
fn letter_counts(s: &str) -> [u8; 26] {
    let mut counts = [0u8; 26];
    for ch in s.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_lowercase() {
            counts[(lower as u8 - b'a') as usize] += 1;
        }
    }
    counts
}

/// Whether `word` can be spelled using only the letters available in `rack`.
///
/// TODO: implement. Compare the per-letter counts of `word` against `rack`
/// (drop the `_` prefixes on the parameters once you use them).
fn can_spell(_word: &str, _rack: &[u8; 26]) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_counts_is_case_insensitive_and_ignores_symbols() {
        let counts = letter_counts("aA b!");
        assert_eq!(counts[0], 2, "two 'a's");
        assert_eq!(counts[1], 1, "one 'b'");
    }

    #[test]
    fn find_words_runs() {
        let found = find_words("pumpkin");
        assert!(found.is_empty(), "no word list wired up yet");
    }
}
