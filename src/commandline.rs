use argh::FromArgs;
use once_cell::sync::Lazy;

#[derive(FromArgs)]
/// An Spellcast solver.
/// README.md has more detailed info on arguments.
pub struct Config {
    #[argh(option, description="board string (def=read board.txt)", short='b')]
    pub board: Option<String>,
    #[argh(option, description="number of top moves to show (def=5)", short='c', default="5")]
    pub move_count: u8,
    #[argh(option, description="value added to tiles with gems (def=0)", short='g', default="0")]
    pub gem_value: u8,
    #[argh(switch, description="pretty-print moves", short='p')]
    pub pretty_print: bool,
    #[argh(option, description="number of swaps to consider (def=0)", short='s', default="0")]
    pub swap_count: u8,
    #[argh(switch, description="run benchmark", short='B')]
    pub benchmark: bool,
}
// TODO: Make subcommands for everything, separate current solver, benchmark and interactive mode
pub static CONFIG: Lazy<Config> = Lazy::new(|| argh::from_env());
