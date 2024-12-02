use crate::{
    commandline::{OutputFormat, SolverSubCommand},
    dictionary::DICTIONARY_CELL,
    quit,
    solver::{Board, Move},
    utils::*,
};

pub fn entry(args: SolverSubCommand, num_threads: u8) {
    if args.swap_count > 3 {
        quit!("Swap amount can't be higher than 3!");
    }
    if let Some(board_str) = args
        .board
        .or_else(|| std::fs::read_to_string("board.txt").ok())
    {
        if let Ok(mut board) = board_str.parse::<Board>() {
            board.gem_bonus = args.gem_value as u16;
            let is_json;
            if let OutputFormat::JSON = args.format {
                is_json = true;
            } else {
                is_json = false;
            }
            if !is_json {
                println!(
                    "Loaded dictionary - {} entries in lookup table",
                    DICTIONARY_CELL.get().unwrap().len()
                );
            }
            let clock = std::time::Instant::now();
            let (mut words, board) = board.solve(args.swap_count, num_threads);
            let elapsed = clock.elapsed().as_secs_f64() * 1000.;
            if !is_json {
                println!("{elapsed:.2}ms elapsed");
            }
            let mut existing_words = vec![];
            let mut counter = 0;
            let mut new_words = vec![];
            for word in words {
                if counter >= args.move_count {
                    break;
                }
                if existing_words.contains(&word.word) {
                    continue;
                }
                counter += 1;
                existing_words.push((&word.word).clone());
                new_words.push(word);
            }
            words = new_words;
            if is_json {
                // Not very good practice, but in this case it's pretty much safe (and it does not have any dependencies)
                println!(
                    r#"{{"elapsed":{elapsed:.2},"solutions":[{}]}}"#,
                    words
                        .into_iter()
                        .map(|w| format!(
                            r#"{{"gems":{},"moves":[{}],"score":{},"sorting_score":{},"swap_count":{},"word":{:?}}}"#, 
                            w.gems, 
                            w.moves.into_iter().map(|m| match m {
                                Move::Normal { index } => format!(r#"{{"swap":false,"index":{index}}}"#),
                                Move::Swap { index, new_letter } => format!(r#"{{"swap":true,"index":{index},"new_letter":"{new_letter}"}}"#)
                            }).collect::<Vec<_>>().join(","), 
                            w.score, 
                            w.sorting_score, 
                            w.swap_count, 
                            w.word
                        ))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            } else {
                // Reverse it, because most terminal emulators have auto-scrolling, and most likely only last table(s) will fit into view.
                for word in words.into_iter().rev() {
                    let mut swaps = vec![];
                    for m in &word.moves {
                        match m {
                            Move::Swap { index, new_letter } => {
                                swaps.push(format!(
                                    "{} -> {} @ {}",
                                    board.tiles[*index as usize].letter,
                                    new_letter,
                                    i2c(*index)
                                ));
                            }
                            _ => (),
                        }
                    }
                    match args.format {
                        OutputFormat::Simple => {
                            println!(
                                "{}. {} (+{}) / {}",
                                counter,
                                word.formatted(&board),
                                word.score,
                                swaps.join("; ")
                            )
                        }
                        OutputFormat::Table => {
                            let mut tiles: [Option<(u8, &Move)>; 25] =
                                std::array::from_fn(|_| None);
                            let mut move_counter: u8 = 0;
                            for m in &word.moves {
                                tiles[m.index() as usize] = Some((move_counter, m));
                                move_counter += 1;
                            }
                            let mut buf = String::from("# ");
                            for (letter, _) in BOARD_COLUMNS {
                                buf = buf + letter + " ";
                            }
                            buf += &format!(" {}\n", word.formatted(&board));
                            for row in 0..5 {
                                buf += &format!("{} ", row + 1);
                                for index in (row * 5)..(row * 5 + 5) {
                                    if let Some((step, m)) = tiles[index] {
                                        let converted_step;
                                        if step < 10 {
                                            converted_step = step.to_string();
                                        } else {
                                            converted_step =
                                                (('a' as u8 + step - 10) as char).to_string();
                                        }
                                        let letter = m.letter(&board);
                                        if let Move::Swap { .. } = m {
                                            buf += &format!(
                                                "{RED}{letter}{GREY}{converted_step}{RESET}"
                                            )
                                        } else {
                                            buf += &format!("{letter}{GREY}{converted_step}{RESET}")
                                        }
                                    } else {
                                        buf += &format!("{BLACK}* {RESET}");
                                    }
                                }
                                if row == 0 {
                                    buf += &format!(" +{} points\n", word.score);
                                } else if row - 1 < swaps.len() {
                                    buf += &format!(" {}\n", swaps[row - 1 as usize]);
                                } else {
                                    buf += "\n";
                                }
                            }
                            println!("=========[{counter}]=========\n{buf}");
                        }
                        _ => (),
                    }
                    counter -= 1;
                }
            }
        } else {
            quit!("Failed to parse board string!");
        }
    } else {
        quit!("Failed to read board string from file!");
    }
}
