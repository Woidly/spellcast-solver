use argh::FromArgs;

use crate::spellcast::Board;

#[derive(FromArgs, Debug)]
/// Spellcast solver.
pub struct Args {
    #[argh(option, description = "board string", short = 'b')]
    pub board: Board,
}

pub fn parse() -> Args {
    argh::from_env()
}
