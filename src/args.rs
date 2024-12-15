use argh::FromArgs;

use crate::spellcast::Board;

#[derive(FromArgs, Debug)]
/// Spellcast solver CLI.
/// You can learn more about arguments in CLI.md.
pub struct Args {
    #[argh(
        option,
        description = "dictionary file (def=dictionary.txt)",
        short = 'd',
        default = "\"dictionary.txt\".into()"
    )]
    pub dictionary: String,
    #[argh(
        option,
        description = "number of threads to use (def=1)",
        short = 't',
        default = "1"
    )]
    pub threads: u8,
    #[argh(option, description = "board string", short = 'b')]
    pub board: Board,
    #[argh(
        option,
        description = "number of top moves to show (def=5)",
        short = 'c',
        default = "5"
    )]
    pub move_count: u8,
    #[argh(
        option,
        description = "number of swaps to consider (def=0)",
        short = 's',
        default = "0"
    )]
    pub swaps: u8,
    #[argh(switch, description = "print verbose output to stderr", short = 'v')]
    pub verbose: bool,
    #[argh(switch, description = "disable colours in output")]
    pub no_colour: bool,
}

pub fn parse() -> Args {
    argh::from_env()
}
