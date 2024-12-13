use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

/// Dictionary lookup result for a sequence of letters.
#[derive(Debug)]
pub enum LookupResult {
    // next_letters never gets longer than 26, so just doing O(n) with Vec<char> is cheaper than having thousands of HashSets.
    /// Sequence of letters is a prefix. To form valid word, next letter must be one of those listed in next_letters.
    Prefix { next_letters: Vec<char> },
    /// Sequence of letters is a word. Add it as a word and terminate search, because no next letter may form another valid word.
    Word,
    /// Sequence of letters is both a prefix and a word. Add it as a word and continue search as if it was a prefix.
    Both { next_letters: Vec<char> },
}

pub type Dictionary = HashMap<&'static str, LookupResult>;

/// Loads dictionary from string.
/// String should contain lowercase words separated by newlines.
/// Only words 3-25 characters long are included.
///
/// Note that it uses Box::leak to give dictionary keys 'static lifetime.
fn load_dictionary(string: String) -> Dictionary {
    let mut dictionary: Dictionary = HashMap::new();
    for word in Box::leak(string.into_boxed_str())
        .lines()
        .filter(|x| x.len() >= 3 && x.len() <= 25)
    {
        if let Some(x) = dictionary.get_mut(word) {
            // It was included as prefix before, so its prefixes are also here and we should just mark it as both and skip prefix routine.
            if let LookupResult::Prefix { next_letters } = x {
                *x = LookupResult::Both {
                    next_letters: std::mem::take(next_letters),
                };
            }
            // Otherwise (Word/Both) it's a duplicate, skip.
        } else {
            // It's a new word, insert it and add prefixes.
            dictionary.insert(word, LookupResult::Word);
            for i in 0..(word.len() - 1) {
                let prefix = &word[0..=i];
                let next_letter = word
                    .chars()
                    .nth(i + 1)
                    .expect("All words should be 3+ in length");
                if let Some(old) = dictionary.get_mut(prefix) {
                    match old {
                        // It's already a Prefix/Both, just append a new letter to existing entry.
                        LookupResult::Prefix { next_letters }
                        | LookupResult::Both { next_letters } => {
                            if !next_letters.contains(&next_letter) {
                                next_letters.push(next_letter);
                            }
                        }
                        // It's a Word, turn it into Both.
                        LookupResult::Word => {
                            dictionary.insert(
                                prefix,
                                LookupResult::Both {
                                    next_letters: vec![next_letter],
                                },
                            );
                        }
                    }
                } else {
                    // It's a completely new prefix.
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

    dictionary
}

/// Loads dictionary from file.
/// Basically a wrapper for [load_dictionary] that handles file access.
pub fn load_dictionary_file(path: &String) -> Result<Dictionary, String> {
    let path = PathBuf::from(path);
    if !path.is_file() {
        return Err("File not found".into());
    }
    let content = read_to_string(path).map_err(|e| format!("Failed to read the file: {e}"))?;
    Ok(load_dictionary(content))
}
