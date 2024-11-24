use crate::{commandline::AutomaticSubCommand, quit};

pub fn entry(_: AutomaticSubCommand, _: u8) {
    quit!(
        "Automatic solver requires `automatic` feature, e.g. `cargo build --release -F automatic`"
    );
}
