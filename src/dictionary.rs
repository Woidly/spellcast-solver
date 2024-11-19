use std::{collections::HashMap, fs};

use once_cell::sync::OnceCell;

// It never gets longer than 26, so just doing O(n) with Vec<char> is cheaper than having thousands of HashSets.
#[derive(Debug)]
pub enum LookupResult {
    /// Sequence of letters is a prefix. To form valid word, next letter must be one of those listed in next_letters.
    Prefix { next_letters: Vec<char> },
    /// Sequence of letters is a word. Terminate search, because no next letter may form another valid word.
    Word,
    /// Sequence of letters is both a prefix and a word. Push this sequence of moves and continue search as if it was a prefix.
    Both { next_letters: Vec<char> },
}

pub type Dictionary = HashMap<&'static str, LookupResult>;

fn load_dictionary(path: String) -> Option<Dictionary> {
    // Only save 3+ letter words because of how dictionary generation works.
    let file = fs::read_to_string(path).ok()?;
    let words = file.lines().filter(|x| x.len() >= 3 && x.len() <= 25);
    let mut dictionary: Dictionary = HashMap::new();
    for word in words {
        let word = Box::leak(word.to_owned().into_boxed_str());
        if let Some(x) = dictionary.get_mut(word) {
            if let LookupResult::Prefix { next_letters } = x {
                // It was included as prefix before, so possible prefixes are also here and we should just mark word as both and skip prefix routine.
                *x = LookupResult::Both {
                    next_letters: std::mem::take(next_letters),
                };
            }
            // Otherwise (word/both) it's a duplicate, skip.
        } else {
            // New word, insert it and add prefixes.
            dictionary.insert(word, LookupResult::Word);
            for i in 0..(word.len() - 1) {
                let prefix = &word[0..=i];
                let next_letter = word
                    .chars()
                    .nth(i + 1)
                    .expect("all words should be 3+ in length");
                if let Some(old) = dictionary.get_mut(prefix) {
                    match old {
                        LookupResult::Prefix { next_letters } => {
                            if !next_letters.contains(&next_letter) {
                                next_letters.push(next_letter);
                            }
                        }
                        LookupResult::Word => {
                            dictionary.insert(
                                prefix,
                                LookupResult::Both {
                                    next_letters: vec![next_letter],
                                },
                            );
                        }
                        LookupResult::Both { next_letters } => {
                            if !next_letters.contains(&next_letter) {
                                next_letters.push(next_letter);
                            }
                        }
                    }
                } else {
                    dictionary.insert(
                        prefix,
                        LookupResult::Prefix {
                            next_letters: vec![next_letter],
                        },
                    );
                }
            }
        }
    }

    Some(dictionary)
}

pub static DICTIONARY_CELL: OnceCell<Dictionary> = OnceCell::new();

pub fn load_dictionary_wrapper(path: Option<String>) -> Option<()> {
    let dictionary = load_dictionary(path.unwrap_or("dictionary.txt".to_string()))?;
    DICTIONARY_CELL.set(dictionary).ok()
}
