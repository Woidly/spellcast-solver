use crate::{
    spellcast::{Board, Step, Word},
    utils::*,
};

/// Enum used for storing output format.
#[derive(Debug)]
pub enum OutputFormat {
    /// Board output format that prints order of steps on board.
    Board,
    /// JSON output format that is intended for automation purposes.
    JSON,
    /// Simple output format that prints each word compactly on a single line.
    Simple,
}

impl OutputFormat {
    /// Returns whether format is intended for humans.
    /// As of now, it returns `true` for everything other than `JSON`.
    pub fn is_for_humans(&self) -> bool {
        !matches!(self, Self::JSON)
    }
}

/// Board output format that prints order of steps on board.
pub fn board_output(board: &Board, words: Vec<Word>) {
    for (i, word) in words.into_iter().enumerate().rev() {
        let mut order = [None; 25];
        let mut swaps = vec![];
        for (i, step) in (&word.steps).into_iter().enumerate() {
            order[step.index() as usize] = Some((i as i8, step));
            if let Step::Swap { index, new_letter } = step {
                swaps.push(format!(
                    "{}{} -> {new_letter}",
                    (b'A' + (index % 5) as u8) as char,
                    index / 5 + 1
                ));
            }
        }
        println!("===============|{i}|===============");
        let mut buf =
            format!("#   A    B    C    D    E\n  {GREY}+----+----+----+----+----+{RESET}\n");
        for row in 0..5 {
            buf += &format!("{} {GREY}|{RESET}", row + 1);
            for column in 0..5 {
                let index = row * 5 + column;
                if let Some((i, step)) = order[index] {
                    if let Step::Swap { new_letter, .. } = step {
                        buf += &format!("{RED}{new_letter} {GREEN}{i:>2}{GREY}|{RESET}");
                    } else {
                        buf += &format!("{} {GREEN}{i:>2}{GREY}|{RESET}", step.letter(board));
                    }
                } else {
                    buf += &format!("    {GREY}|{RESET}");
                }
            }
            match row {
                0 => {
                    buf += &format!(
                        " {}\n  {GREY}+----+----+----+----+----+{RESET}\n",
                        word.word(&board, true)
                    )
                }
                1 => {
                    buf += &format!(
                        " +{} pts, +{} gems\n  {GREY}+----+----+----+----+----+{RESET}\n",
                        word.score, word.gems_collected
                    )
                }
                2..=4 => {
                    if let Some(swap) = swaps.get(row - 2) {
                        buf += &format!(" {swap}\n  {GREY}+----+----+----+----+----+{RESET}\n");
                    } else {
                        buf += &format!("\n  {GREY}+----+----+----+----+----+{RESET}\n");
                    }
                }
                _ => buf += &format!("\n  {GREY}+----+----+----+----+----+{RESET}\n"),
            }
        }
        println!("{}", buf);
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
                word.word(&board, false)
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
}

/// Simple output format that prints each word compactly on a single line.
pub fn simple_output(board: &Board, words: Vec<Word>) {
    for (i, word) in words.into_iter().enumerate().rev() {
        println!(
            "{i}. {} (+{}pts, +{} gems){}",
            word.word(&board, true),
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
