/// Maximum number of solutions to keep in memory.
/// Smaller numbers mean less RAM usage.
/// Do not set it too low to keep words diverse (as it may include many different copies of same word).
/// You can learn more in [crate::spellcast::SortedWordVec].
pub const MAX_SOLUTIONS: usize = 256;
