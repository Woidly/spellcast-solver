use argh::FromArgs;

use crate::spellcast::Board;

#[derive(FromArgs, Debug)]
/// Spellcast solver.
pub struct Args {
    #[argh(
        option,
        description = "dictionary",
        short = 'd',
        default = "\"dictionary.txt\".into()"
    )]
    pub dictionary: String,
    #[argh(option, description = "board string", short = 'b')]
    pub board: Board,
    #[argh(
        option,
        description = "number of top moves to show",
        short = 'c',
        default = "5"
    )]
    pub move_count: u8,
    #[argh(
        option,
        description = "number of swaps to consider",
        short = 's',
        default = "0"
    )]
    pub swaps: u8,
}

pub fn parse() -> Args {
    argh::from_env()
}
