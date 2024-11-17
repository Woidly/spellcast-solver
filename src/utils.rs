use std::time::{SystemTime, UNIX_EPOCH};

// Using panic instead of process::exit so Rust drops Getch and fixes terminal.
// Backtrace kind of ruins the cleanness but it's better than broken terminal.
// Keeping this macro (for now) instead of panic just to have it coloured xD
#[macro_export]
macro_rules! quit {
    ($($arg:tt)*) => {{
        panic!("{}{}{}", crate::utils::RED, format!($($arg)*), crate::utils::RESET);
    }};
}

pub fn get_random() -> random::Xorshift128Plus {
    random::default(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time flow should be linear")
            .as_nanos() as u64,
    )
}

pub static BLACK: &str = "\x1B[30m";
pub static CLEAR_HOME: &str = "\x1B[2J\x1B[H"; // Both clears the terminal and returns cursor to top left
pub static GREEN: &str = "\x1B[92m";
pub static GREY: &str = "\x1B[90m";
pub static MAGENTA: &str = "\x1B[35m";
pub static RED: &str = "\x1B[31m";
pub static RESET: &str = "\x1B[0m";
