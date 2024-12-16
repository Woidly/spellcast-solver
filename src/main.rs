mod args;
mod dictionary;
mod spellcast;
mod utils;

fn main() {
    let args = args::parse();
    if args.swaps > 3 {
        quit!("Swap count can't be higher than 3!")
    }
    let clock = std::time::Instant::now();
    // TODO: Maybe make dictionary loading a part of argument parsing (similar to board argument)?
    let dictionary = match dictionary::load_dictionary_file(&args.dictionary) {
        Ok(dictionary) => Box::leak(Box::new(dictionary)),
        Err(e) => quit!("Failed to load dictionary: {e}"),
    };
    let elapsed_dict = clock.elapsed().as_secs_f64() * 1000.;
    if args.format.is_for_humans() {
        println!("Loaded the dictionary in {elapsed_dict:.1}ms",)
    }
    let clock = std::time::Instant::now();
    let (words, board) =
        spellcast::solver_wrapper(args.board, args.swaps, args.threads, dictionary);
    let elapsed_solver = clock.elapsed().as_secs_f64() * 1000.;
    if args.format.is_for_humans() {
        println!("Solved the board in {elapsed_solver:.1}ms",);
    }
    let mut existing_words = vec![];
    let mut counter = 0;
    let mut final_words = vec![];
    for word in words {
        if counter >= args.move_count {
            break;
        }
        let word_str = word.word(&board, false, false);
        if existing_words.contains(&word_str) {
            continue;
        }
        counter += 1;
        existing_words.push(word_str);
        final_words.push(word);
    }
    match args.format {
        args::OutputFormat::JSON => {
            // Totally real JSON serialisation!!1!
            // At least it has no dependencies...
            println!(
                r#"{{"elapsed_ms":{{"dict":{elapsed_dict:.1},"solver":{elapsed_solver:.1}}},"words":[{}]}}"#,
                final_words
                    .into_iter()
                    .map(|word| format!(
                        r#"{{"gems_collected":{},"steps":[{}],"score":{},"swaps_used":{},"word":{:?}}}"#,
                        word.gems_collected,
                        (&word.steps)
                            .into_iter()
                            .map(|step| match step {
                                spellcast::Step::Normal { index } =>
                                    format!(r#"{{"swap":false,"index":{index}}}"#),
                                spellcast::Step::Swap { index, new_letter } => format!(
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
        args::OutputFormat::Simple => {
            for (i, word) in final_words.into_iter().enumerate().rev() {
                println!(
                    "{i}. {} (+{}pts, +{} gems){}",
                    word.word(&board, true, !args.no_colour),
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
                                        spellcast::Step::Normal { .. } => None,
                                        spellcast::Step::Swap { index, new_letter } => {
                                            Some(format!(
                                                "{}{} -> {new_letter}",
                                                (b'A' + (index % 5) as u8) as char,
                                                index / 5 + 1
                                            ))
                                        }
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                );
            }
        }
    }
}
