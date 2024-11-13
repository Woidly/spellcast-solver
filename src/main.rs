mod commandline;
mod dictionary;
mod oldsolver;
mod solver;
mod utils;

use commandline::SubCommand;

fn main() {
    match commandline::parse() {
        SubCommand::Benchmark(_) => quit!("Benchmark isn't implemented yet"),
        SubCommand::Interactive(_) => quit!("Interactive mode isn't implemented yet"),
        SubCommand::Solver(args) => oldsolver::entry(args),
    }
}
