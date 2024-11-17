mod commandline;
mod dictionary;
mod interactive;
mod oldsolver;
mod solver;
mod utils;

use commandline::SubCommand;

fn main() {
    let args = commandline::parse();
    if dictionary::load_dictionary_wrapper(args.dictionary).is_none() {
        quit!("Failed to load dictionary from file");
    }
    match args.subcommand {
        SubCommand::Benchmark(_) => quit!("Benchmark isn't implemented yet"),
        SubCommand::Interactive(args) => interactive::entry(args),
        SubCommand::Solver(args) => oldsolver::entry(args),
    }
}
