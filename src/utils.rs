#[macro_export]
macro_rules! quit {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}

pub static CLEAR_HOME: &str = "\x1B[2J\x1B[H"; // Both clears the terminal and returns cursor to top left
pub static RED: &str = "\x1B[31m";
pub static RESET: &str = "\x1B[0m";
