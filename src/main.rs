mod args;
mod dictionary;
mod output;
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
    let (words, board) = spellcast::solver_wrapper(
        args.board,
        args.swaps,
        args.threads,
        dictionary,
        args.move_count,
    );
    let elapsed_solver = clock.elapsed().as_secs_f64() * 1000.;
    if args.format.is_for_humans() {
        println!("Solved the board in {elapsed_solver:.1}ms",);
    }
    match args.format {
        output::OutputFormat::JSON => {
            output::json_output(&board, words, elapsed_dict, elapsed_solver);
        }
        output::OutputFormat::Simple => {
            output::simple_output(&board, words, args.no_colour);
        }
    }
}
