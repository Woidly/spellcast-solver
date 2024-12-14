use std::{fs::read_to_string, path::PathBuf};

pub enum Node {
    Prefix { next_letters: Vec<(char, Node)> },
    Word,
    Both { next_letters: Vec<(char, Node)> },
}

impl Node {
    pub fn get_next_letters(&mut self) -> &mut Vec<(char, Node)> {
        match self {
            Node::Word => panic!("get_next_letters called on Node::Word"),
            Node::Both { next_letters } | Node::Prefix { next_letters } => next_letters,
        }
    }
}

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
                let (_, sub_node) = &mut parent.get_next_letters()[found_index];
                match sub_node {
                    Node::Word => {
                        *sub_node = Node::Both {
                            next_letters: vec![next_letter_node_def],
                        }
                    }
                    Node::Prefix { .. } | Node::Both { .. } => {
                        let mut sub_found = false;
                        let mut sub_found_index = 0;
                        for (sub_letter, _) in sub_node.get_next_letters() {
                            if *sub_letter == next_letter {
                                sub_found = true;
                                break;
                            }
                            sub_found_index += 1;
                        }
                        if sub_found {
                            let (_, sub_sub_node) =
                                &mut sub_node.get_next_letters()[sub_found_index];
                            match sub_sub_node {
                                Node::Word => {
                                    *sub_sub_node = Node::Both {
                                        next_letters: vec![],
                                    }
                                }
                                _ => (),
                            }
                        } else {
                            sub_node.get_next_letters().push(next_letter_node_def);
                        }
                    }
                }
                parent = sub_node;
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
