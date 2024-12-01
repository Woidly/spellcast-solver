# spellcast-solver

It's a... Spellcast solver... isn't the name obvious?

This project is inspired by WintrCat's [Spellcast solver](https://github.com/WintrCat/spellcastsolver) (you can check out his videos on original solver - [[1]](https://youtu.be/hYoojWO9hh8), [[2]](https://youtu.be/QvJATba04u8)). Though, mine solver is **more than 10x faster** than original because it is written in Rust and has several new optimizations. You can see list of features below (some are still WIP):

## Features

As solver is now much more complicated than original, I logically split it into 4 parts:

### Solver ([`src/solver.rs`](src/solver.rs))

Program's core, code that actually "solves" the board.

- Proper score calculation according to [wiki](https://discord.fandom.com/wiki/SpellCast) (DL/TL, 2x/3x, long word bonus)
- Additional score bonus for tiles with gems to prioritise them
- "Frozen tiles" (e.g. tiles that solver completely avoids)
- Ability to use swaps
- Multithreading (it's not ideal, but it's still pretty good and I'll improve it in future)

Though it works in a different way, results are as good as in original, because solver is guaranteed to visit every valid node.
Only difference for you is that it doesn't use "estimated values", therefore it doesn't recommend shuffling like original (though you can decide when to shuffle yourself).

### CLI solver ([`src/oldsolver.rs`](src/oldsolver.rs))

Slightly better version of WintrCat's project.

- Same board string format as original
- Everything configurable via command line arguments
- Pretty-printing moves is bit more useful as it shows order of moves

### Interactive solver ([`src/interactive.rs`](src/interactive.rs))

A TUI solver.
Not sure whether it's better than CLI solver.
You can learn more in [INTERACTIVE.md](INTERACTIVE.md)

### Automatic solver (WIP)

Weird framework for making automatic solver that can be actually useful in game.
You input screenshot, it uses OCR provided by you to parse the board, it solves the board and gives bunch of commands that your script must turn into mouse movements.
(Do not worry, it includes decent script examples.)
It is WIP, check `automatic` branch for more details (eventually it'll be merged into `main`).

## How

If you want to know details of how this thing works, I left plenty of comments in code.
These comments (and basic programming skills) should be enough to completely understand how it works.
My code is much simpler than the original, if I do say so myself.

## Running

### Basic setup (obvious)

- Have Git, Rust and Cargo installed
- Clone the repository, build the release binary

### Terminal

This tool is for terminal (no GUI).
Everything can be configured via command-line arguments (except for interactive mode).
Basic 16-colour support in terminal is recommended.
You can learn more about command line arguments in [CLI.md](CLI.md).

## Benchmarks

**As of now, built-in benchmark isn't implemented yet due to technical difficulties.**
**Results related to my solver are done in a bit hackish way via [benchmark.py](benchmark.py).**

Below you can see benchmark results from my mediocre computer with 11th gen Intel Core i5 with `RUSTFLAGS="-C target-cpu=native"` that compare my solver with original. To keep table compact, I use these shortcuts:

- WLY - Woidly's solver (this one) with single thread
- WLY-12t - Woidly's solver with 12 threads
- WCT - WintrCat's solver (original) on default Python runtime
- WCT-pypy - WintrCat's solver on PyPy runtime

The times in the table are mean/average of all runs (total_time / board_count), as in original benchmark.
Each benchmark is run with 100 boards, so 25 for each swap count (0/1/2/3).

| Target      | 0 swaps | 1 swap   | 2 swaps    | 3 swaps    |
| ----------- | ------- | -------- | ---------- | ---------- |
| **WLY-12t** | 0.6 ms  | 4.0 ms   | 114.0 ms   | 1535.5 ms  |
| WLY         | 0.2 ms  | 14.2 ms  | 387.8 ms   | 8503.8 ms  |
| WCT-pypy    | 2.6 ms  | 119.3 ms | 2982.9 ms  | 51150.3 ms |
| WCT         | 3.5 ms  | 263.2 ms | 10129.1 ms | 100000+ ms |

> [!NOTE]  
> Results related to my solver do not include time it takes to load a dictionary (~50ms).
> The same applies to WintrCat's solver.

> [!NOTE]  
> Results related to my solver are from commit `b34bb07`.
> I will update results only when making commits related to performance.
> As for WintrCat's solver, current results are from pre-multiprocessing version of repository.
