use crate::{commandline::AutomaticSubCommand, quit};

pub fn entry(args: AutomaticSubCommand, num_threads: u8) {
    #[cfg(not(feature = "automatic"))]
    quit!("Automatic solver requires `automatic` feature, e.g. cargo build --release -F automatic");
    #[cfg(feature = "automatic")]
    {
        quit!("Automatic solver isn't implemented yet");
    }
}
