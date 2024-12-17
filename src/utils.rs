/// Maximum number of solutions to keep in memory.
/// Smaller numbers mean less RAM usage.
/// Do not set it too low to keep words diverse (as it may include many different copies of same word).
/// You can learn more in [crate::spellcast::SortedWordVec].
pub const MAX_SOLUTIONS: usize = 256;

pub const GREEN: &str = "\x1B[32m";
pub const GREY: &str = "\x1B[90m";
pub const RED: &str = "\x1B[31m";
pub const RESET: &str = "\x1B[0m";

#[macro_export]
macro_rules! quit {
    ($($arg:tt)*) => {{
        eprintln!("{}{}{}", crate::utils::RED, format!($($arg)*), crate::utils::RESET);
        std::process::exit(1);
    }};
}
