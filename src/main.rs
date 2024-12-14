mod args;
mod dictionary;
mod spellcast;
mod utils;

fn main() {
    let args = args::parse();
    // TODO: Maybe make dictionary loading a part of argument parsing (similar to board argument)?
    let dictionary = match dictionary::load_dictionary_file(&args.dictionary) {
        Ok(dictionary) => Box::leak(Box::new(dictionary)),
        Err(e) => quit!("Failed to load dictionary: {e}"),
    };
    if args.swaps > 3 {
        quit!("Swap count can't be higher than 3!")
    }
    let clock = std::time::Instant::now();
    let (words, board) =
        spellcast::solver_wrapper(args.board, args.swaps, args.threads, dictionary);
    let elapsed_ms = clock.elapsed().as_secs_f64() * 1000.;
    println!("{elapsed_ms:.1}ms elapsed");
    let mut existing_words = vec![];
    let mut counter = 0;
    let mut final_words = vec![];
    for word in words {
        if counter >= args.move_count {
            break;
        }
        let word_str = word.word(&board);
        if existing_words.contains(&word_str) {
            continue;
        }
        counter += 1;
        existing_words.push(word_str);
        final_words.push(word);
    }
    for (i, word) in final_words.into_iter().enumerate().rev() {
        println!(
            "{i}. {} (+{}pts, +{} gems, -{} swaps)",
            word.word(&board),
            word.score,
            word.gems_collected,
            word.swaps_used
        );
    }
}
