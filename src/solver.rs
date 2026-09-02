//! Countdown letters-round solver.
//!
//! Given a rack of letters, find every dictionary word that can be spelled
//! using a subset of those letters (each letter used at most as many times as
//! it appears in the rack), grouped by word length.

// use wordfreq_model::load_wordfreq;
// use wordfreq_model::ModelKind;


use hashbag::HashBag;
use std::collections::BTreeMap;




    // Embed the file contents into a static string slice at compile time
const WORDLIST_RAW: &str = include_str!("words.txt");


static WORD_LIST: std::sync::LazyLock<Vec<(String, HashBag<u8>)>> =
    std::sync::LazyLock::new(|| {
        //let words: Vec<&str> = top_english_words::get_words();
        let words: Vec<&str> = WORDLIST_RAW
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
        let mut word_list_with_hashbag = Vec::new();

        for word in words {
            word_list_with_hashbag.push((
                word.to_string(),
                make_hashbag(&word.to_lowercase()),
            ));
        }
        //println!("Our wordlist is {} words long", word_list_with_hashbag.len());
        word_list_with_hashbag
    });

fn make_hashbag(word: &str) -> HashBag<u8> {
    word.bytes().collect()
}

fn is_subset<T: std::hash::Hash + Eq>(bag_a: &HashBag<T>, bag_b: &HashBag<T>) -> bool {
    // A cannot be a subset if it has more total elements than B
    if bag_a.len() > bag_b.len() {
        return false;
    }

    // Every item in A must exist in B with a greater or equal count
    bag_a
        .set_iter()
        .all(|(element, count_a)| bag_b.contains(element) >= count_a)
}




//.word_frequency_map.keys().map(|s| s.as_str()).collect::<Vec<_>>();

/// Search for all words that can be built from `letters`.
///
/// `letters` is the raw rack as typed by the player: case is ignored and any
/// non-alphabetic characters are ignored.
///
/// Returns a map from word length to the list of words of that length that can
/// be made from the rack. Lengths with no matches are omitted.
pub fn find_words(letters: &str) -> BTreeMap<usize, Vec<String>> {
    // Multiset of available letters, indexed by `0 == 'a'` .. `25 == 'z'`.
    let rack: HashBag<u8> = make_hashbag(&letters.to_lowercase());

    let mut results: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for (word,hb) in &*WORD_LIST {
        if is_subset(hb,&rack) {
            results
                .entry(word.chars().count())
                .or_default()
                .push(word.to_owned());
        }
    }

    results
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
