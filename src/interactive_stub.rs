use crate::{commandline::InteractiveSubCommand, quit};

pub fn entry(_: InteractiveSubCommand, _: u8) {
    quit!("This function requires \"interactive\" feature to work (e.g. `cargo build --release -F interactive`)");
}
