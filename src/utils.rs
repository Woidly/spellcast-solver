// Using panic instead of process::exit so Rust drops Getch and fixes terminal.
// Backtrace kind of ruins the cleanness but it's better than broken terminal.
// Keeping this macro (for now) instead of panic just to have it coloured xD
#[macro_export]
macro_rules! quit {
    ($($arg:tt)*) => {{
        panic!("{}{}{}", crate::utils::RED, format!($($arg)*), crate::utils::RESET);
    }};
}

pub const BOARD_COLUMNS: [(&str, i8); 5] = [("A", 0), ("B", 1), ("C", 2), ("D", 3), ("E", 4)];

/// Converts board index (0-24) into standard coordinate system (A-E for column + 1-5 for row).
pub fn i2c(index: i8) -> String {
    format!("{}{}", BOARD_COLUMNS[(index % 5) as usize].0, index / 5 + 1)
}

pub const BLACK: &str = "\x1B[30m";
pub const CLEAR_HOME: &str = "\x1B[2J\x1B[H"; // Both clears the terminal and returns cursor to top left.
pub const GREEN: &str = "\x1B[92m";
pub const GREY: &str = "\x1B[90m";
pub const MAGENTA: &str = "\x1B[35m";
pub const RED: &str = "\x1B[31m";
pub const RESET: &str = "\x1B[0m";

/// Maximum number of solutions to keep in memory.
/// Smaller numbers mean less RAM usage.
/// Do not set it too low to keep words diverse (as it may include many different copies of same word).
/// You can learn more in [crate::solver::SortedWordVec].
///
/// Note: each thread gets its own SortedWordVec instance, so there are actually MAX_SOLUTIONS*threads solutions in RAM in most cases.
/// 256 seems good enough, as even with 12 threads RAM usage stays below 100 MB.
pub const MAX_SOLUTIONS: usize = 256;
