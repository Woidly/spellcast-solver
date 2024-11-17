use crate::{
    commandline::SolverSubCommand,
    dictionary::DICTIONARY_CELL,
    quit,
    solver::{Board, Move},
    utils::*,
};

pub fn entry(args: SolverSubCommand) {
    if args.swap_count > 3 {
        quit!("Swap amount can't be higher than 3");
    }
    if let Some(board_str) = args
        .board
        .or_else(|| std::fs::read_to_string("board.txt").ok())
    {
        if let Ok(mut board) = board_str.parse::<Board>() {
            board.gem_bonus = args.gem_value as u16;
            // Just load the dictionary by doing something with it (to time actual search time properly).
            println!(
                "Loaded dictionary - {} entries in lookup table",
                DICTIONARY_CELL.get().unwrap().len()
            );
            let clock = std::time::Instant::now();
            let mut words = board.solve(args.swap_count);
            println!(
                "Found {} unique move sequences in {:.2}ms",
                words.len(),
                clock.elapsed().as_secs_f64() * 1000.
            );
            words.sort_by_key(|x| -(x.score as i32));
            let mut existing_words = vec![];
            let mut counter = 0;
            for word in words {
                if counter >= args.move_count {
                    break;
                }
                if existing_words.contains(&word.word) {
                    continue;
                }
                counter += 1;
                existing_words.push((&word.word).clone());
                let mut letters = vec![];
                let mut swaps = vec![];
                for m in &word.moves {
                    match m {
                        Move::Normal { index } => {
                            letters.push(board.tiles[*index as usize].letter.to_string())
                        }
                        Move::Swap { index, new_letter } => {
                            letters.push(format!("{RED}{new_letter}{RESET}"));
                            swaps.push(format!(
                                "{} -> {} @ {},{}",
                                board.tiles[*index as usize].letter,
                                new_letter,
                                index % 5,
                                index / 5
                            ));
                        }
                    }
                }
                if args.pretty_print {
                    let mut table: [String; 25] = std::array::from_fn(|_| ".".to_string());
                    for m in &word.moves {
                        if let Move::Swap { .. } = m {
                            table[m.index() as usize] = format!("{RED}{}{RESET}", m.letter(&board));
                        } else {
                            table[m.index() as usize] = m.letter(&board).to_string();
                        }
                    }
                    println!("=========[{}]=========", counter);
                    for y in 0..5 {
                        for x in 0..5 {
                            print!("{}", table[y * 5 + x]);
                        }
                        if y == 0 {
                            print!(" {}", letters.join(""));
                        }
                        if y == 1 {
                            print!(" {} points", word.score);
                        }
                        if y >= 2 {
                            let swap_index = y - 2;
                            if swap_index < swaps.len() {
                                print!(" {}", swaps[swap_index as usize]);
                            }
                        }
                        println!();
                    }
                } else {
                    println!(
                        "{}. [{}] {} / {}",
                        counter,
                        word.score,
                        letters.join(""),
                        swaps.join("; ")
                    )
                }
            }
        } else {
            quit!("Failed to parse board string!");
        }
    } else {
        quit!("Failed to read board string from file!");
    }
}
