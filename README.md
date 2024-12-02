# spellcast-solver

It's a... Spellcast solver... isn't the name obvious?

This project is inspired by WintrCat's [Spellcast solver](https://github.com/WintrCat/spellcastsolver) (you can check out his videos on original solver - [[1]](https://youtu.be/hYoojWO9hh8), [[2]](https://youtu.be/QvJATba04u8)). Though, mine solver is **more than 10x faster** than original because it is written in Rust and has several new optimizations.

## Features

- Fast solver that supports all standard game elements (DL/TL/2x, 0-3 swaps, frozen tiles, gems)
- CLI that allows to access solver (you can learn more in [docs/CLI.md](docs/CLI.md))
- "Interactive solver" TUI (you can learn more in [docs/INTERACTIVE.md](docs/INTERACTIVE.md))

## How

If you want to know details of how this thing works, I left plenty of comments in code.
These comments (and basic programming skills) should be enough to completely understand how it works.
My code is much simpler than the original, if I do say so myself.

## Basic setup

- Have Git, Rust and Cargo installed
- Clone/download the repository
- Build a release binary
- Run it in terminal (16-colour support in terminal is recommended)

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

> [!NOTE]  
> Unlike original, my benchmarks only measure speed.
> Since the game is random, game-related metrics aren't meaningful in any way.
> The solver is guaranteed to find the optimal solution for any given board state.
