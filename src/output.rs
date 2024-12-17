use crate::spellcast::{Board, Step, Word};

/// Enum used for storing output format.
#[derive(Debug)]
pub enum OutputFormat {
    /// Simple output format that prints each word compactly on a single line.
    Simple,
    /// JSON output format that is intended for automation purposes.
    JSON,
}

impl OutputFormat {
    /// Returns whether format is intended for humans.
    /// As of now, it returns `true` for everything other than `JSON`.
    pub fn is_for_humans(&self) -> bool {
        match self {
            Self::Simple => true,
            Self::JSON => false,
        }
    }
}

/// JSON output format that is intended for automation purposes.
pub fn json_output(board: &Board, words: Vec<Word>, elapsed_dict: f64, elapsed_solver: f64) {
    // Totally real JSON serialisation!!1!
    // At least it has no dependencies...
    println!(
        r#"{{"elapsed_ms":{{"dict":{elapsed_dict:.1},"solver":{elapsed_solver:.1}}},"words":[{}]}}"#,
        words
            .into_iter()
            .map(|word| format!(
                r#"{{"gems_collected":{},"steps":[{}],"score":{},"swaps_used":{},"word":{:?}}}"#,
                word.gems_collected,
                (&word.steps)
                    .into_iter()
                    .map(|step| match step {
                        Step::Normal { index } => format!(r#"{{"swap":false,"index":{index}}}"#),
                        Step::Swap { index, new_letter } => format!(
                            r#"{{"swap":true,"index":{index},"new_letter":"{new_letter}"}}"#
                        ),
                    })
                    .collect::<Vec<_>>()
                    .join(","),
                word.score,
                word.swaps_used,
                word.word(&board, false, false)
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
}

/// Simple output format that prints each word compactly on a single line.
pub fn simple_output(board: &Board, words: Vec<Word>, no_colour: bool) {
    for (i, word) in words.into_iter().enumerate().rev() {
        println!(
            "{i}. {} (+{}pts, +{} gems){}",
            word.word(&board, true, !no_colour),
            word.score,
            word.gems_collected,
            if word.swaps_used == 0 {
                String::new()
            } else {
                format!(
                    " / {}",
                    word.steps
                        .into_iter()
                        .filter_map(|step| {
                            match step {
                                Step::Normal { .. } => None,
                                Step::Swap { index, new_letter } => Some(format!(
                                    "{}{} -> {new_letter}",
                                    (b'A' + (index % 5) as u8) as char,
                                    index / 5 + 1
                                )),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        );
    }
}
