use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// An Spellcast solver.
/// README.md has more detailed info on arguments.
struct Args {
    #[argh(subcommand)]
    subcommand: SubCommand,
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
#[argh(
    subcommand,
    name = "solver",
    description = "run the old no-state solver"
)]
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

pub fn parse() -> SubCommand {
    argh::from_env::<Args>().subcommand
}
