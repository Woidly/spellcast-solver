# SpellcastSolver

It's a... Spellcast solver... isn't the name obvious? (It's currently WIP.)

This project is inspired by WintrCat's Spellcast solver [[YouTube video]](https://youtu.be/hYoojWO9hh8) [[YouTube video 2]](https://youtu.be/QvJATba04u8) [[GitHub repo]](https://github.com/WintrCat/spellcastsolver) (but mine is more than 10x faster cause Rust + several new optimizations).

## Stuff

It has most of the original features:

- Considering tile boosts when calculating score (DL, TL, 2x, 3x, long word bonus)
- Frozen tiles
- 1-3 tile swaps
- Ability to prioritise gems (configurable)
- Benchmark (WIP)

But some things are missing:

- Doesn't recommend shuffling (use your brain to figure out when to shuffle)
- It doesn't make any assumptions (no "estimated values"), it just works with current board state (in most cases results are the same as in original solver)

Some things are exclusive to this solver:

- Interactive mode that makes solver actually useful in game
- Multi-threading (it's not ideal, but it's still pretty good and I'll improve it in future)

Score calculation follows https://discord.fandom.com/wiki/SpellCast and fully matches the original project.

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

(Single-threaded version is faster for 0 swaps because multithreading is overkill for such a simple task)
