mod commandline;
mod dictionary;
#[cfg(feature = "interactive")]
#[path = "interactive.rs"]
mod interactive;
#[cfg(not(feature = "interactive"))]
#[path = "interactive_stub.rs"]
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
        SubCommand::Interactive(sub) => interactive::entry(sub, args.threads),
        SubCommand::Solver(sub) => oldsolver::entry(sub, args.threads),
    }
}
