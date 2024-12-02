use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// An Spellcast solver.
/// docs/CLI.md has more detailed info on arguments.
pub struct Args {
    #[argh(subcommand)]
    pub subcommand: SubCommand,
    #[argh(
        option,
        description = "dictionary file",
        short = 'd',
        default = "\"dictionary.txt\".to_string()"
    )]
    pub dictionary: String,
    #[argh(
        option,
        description = "number of threads to use (def=1)",
        short = 't',
        default = "1"
    )]
    pub threads: u8,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Benchmark(BenchmarkSubCommand),
    Interactive(InteractiveSubCommand),
    Solver(SolverSubCommand),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "benchmark", description = "run the benchmark")]
pub struct BenchmarkSubCommand {}

#[derive(FromArgs, Debug)]
#[argh(
    subcommand,
    name = "interactive",
    description = "run the interactive solver"
)]
pub struct InteractiveSubCommand {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "solver", description = "run the solver CLI")]
pub struct SolverSubCommand {
    #[argh(option, description = "board string (def=read board.txt)", short = 'b')]
    pub board: Option<String>,
    #[argh(
        option,
        description = "number of top moves to show (def=5)",
        short = 'c',
        default = "5"
    )]
    pub move_count: u8,
    #[argh(
        option,
        description = "value added to tiles with gems (def=0)",
        short = 'g',
        default = "0"
    )]
    pub gem_value: u8,
    #[argh(switch, description = "pretty-print moves", short = 'p')]
    pub pretty_print: bool,
    #[argh(
        option,
        description = "number of swaps to consider (def=0)",
        short = 's',
        default = "0"
    )]
    pub swap_count: u8,
}

pub fn parse() -> Args {
    argh::from_env()
}
