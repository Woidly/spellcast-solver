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
        args::OutputFormat::JSON => todo!("JSON output format isn't implemented yet"),
        args::OutputFormat::Simple => {
            for (i, word) in final_words.into_iter().enumerate().rev() {
                println!(
                    "{i}. {} (+{}pts, +{} gems, -{} swaps)",
                    word.word(&board, true, !args.no_colour),
                    word.score,
                    word.gems_collected,
                    word.swaps_used
                );
            }
        }
    }
}
