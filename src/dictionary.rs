use std::{fs::read_to_string, path::PathBuf};

/// A node in dictionary tree.
pub enum Node {
    /// Node is a prefix. It can only be followed by child nodes (`self.next_letters`) and is useless on its own.
    Prefix { next_letters: Vec<(char, Node)> },
    /// Node is a word. It has no child nodes.
    Word,
    /// Node is both a prefix and a word. It can be followed by child nodes (`self.next_letters`), but can also be a standalone word.
    Both { next_letters: Vec<(char, Node)> },
}

impl Node {
    /// Returns `&mut self.next_letters` for `Both`/`Prefix`.
    /// It panics when called on `Word`, however dictionary loading logic ensures it is never called on `Word`.
    /// Just a convenience function that handles matching and panic.
    fn get_next_letters(&mut self) -> &mut Vec<(char, Node)> {
        match self {
            Node::Both { next_letters } | Node::Prefix { next_letters } => next_letters,
            Node::Word => unreachable!(),
        }
    }
}

/// Parses string of words separated by newlines into tree-like structure.
/// Because of how dictionary works, words shorter than 3 characters are ignored.
/// Words longer than 25 characters are also ignored, because it is impossible to play them in Spellcast.
/// Despite code having lot of things that in theory can panic, it does not panic under normal circumstances.
/// Each `Both`/`Prefix` node is guaranteed to have at least one child node, and each branch is guaranteed to eventually end in `Word` node.
pub fn load_dictionary_tree(string: String) -> Vec<(char, Node)> {
    let mut root = Node::Prefix {
        next_letters: vec![],
    };
    for word in string.lines().filter(|x| x.len() >= 3 && x.len() <= 25) {
        let mut parent = &mut root;
        for i in 0..(word.len() - 1) {
            let letter = word.chars().nth(i).unwrap();
            let next_letter = word.chars().nth(i + 1).unwrap();
            let is_next_letter_last_letter = i + 1 == word.len() - 1;
            let next_letter_node_def = (
                next_letter,
                if is_next_letter_last_letter {
                    Node::Word
                } else {
                    Node::Prefix {
                        next_letters: vec![],
                    }
                },
            );
            let mut found = false;
            let mut found_index = 0;
            for (sub_letter, _) in parent.get_next_letters() {
                if *sub_letter == letter {
                    found = true;
                    break;
                }
                found_index += 1;
            }
            if found {
                let (_, current_letter_node) = &mut parent.get_next_letters()[found_index];
                match current_letter_node {
                    Node::Both { .. } | Node::Prefix { .. } => {
                        let mut next_found = false;
                        for (sub_letter, _) in current_letter_node.get_next_letters() {
                            if *sub_letter == next_letter {
                                next_found = true;
                                break;
                            }
                        }
                        if !next_found {
                            current_letter_node
                                .get_next_letters()
                                .push(next_letter_node_def);
                        }
                    }
                    Node::Word => {
                        *current_letter_node = Node::Both {
                            next_letters: vec![next_letter_node_def],
                        }
                    }
                }
                parent = current_letter_node;
            } else {
                parent.get_next_letters().push((
                    letter,
                    Node::Prefix {
                        next_letters: vec![next_letter_node_def],
                    },
                ));
                parent = &mut parent.get_next_letters().last_mut().unwrap().1;
            }
        }
    }

    match root {
        Node::Prefix { next_letters } => next_letters,
        _ => unreachable!(),
    }
}

/// Loads dictionary from file.
/// Basically a wrapper for [load_dictionary_tree] that handles file access.
pub fn load_dictionary_file(path: &String) -> Result<Vec<(char, Node)>, String> {
    let path = PathBuf::from(path);
    if !path.is_file() {
        return Err("File not found".into());
    }
    let content = read_to_string(path).map_err(|e| format!("Failed to read the file: {e}"))?;
    Ok(load_dictionary_tree(content))
}
