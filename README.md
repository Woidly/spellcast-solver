# spellcast-solver

**The fastest solver for Discord Spellcast, written in Rust.**

This project is inspired by WintrCat's [Spellcast solver](https://github.com/WintrCat/spellcastsolver), but is [**more than 10x faster**](#benchmarks) due to several new optimisations and being written in Rust.

## Features

- Fast and efficient solver core that supports multithreading
- Support for all standard game features (DL/TL/2x, 0-3 swaps, gems, frozen tiles)
- Simple CLI for running the solver (you can learn more in [CLI.md](CLI.md))
- Userscript that automatically plays Spellcast ([Woidly/spellcast-autoplay](https://github.com/Woidly/spellcast-autoplay))

## Setup

1. Install **Git** and **Rust**.
2. Clone this repository:  
   `git clone https://github.com/Woidly/spellcast-solver.git`
3. Build the project:  
   `RUSTFLAGS="-C target-cpu=native" cargo build --release`
4. Run the solver via CLI:  
   `./target/release/spellcast-solver --help`

## Benchmarks

**Due to technical difficulties, benchmarks are currently performed using the external [benchmark.py](benchmark.py) script.**

Below is the table with benchmarks done on my mediocre computer with 11th gen Intel Core i5 CPU (compiled w/ `RUSTFLAGS="-C target-cpu=native"`).
Results are in milliseconds, rounded to one decimal place and calculated as the mean (`sum(times)/len(times)`) of 1000 runs (250 runs for each swap count).

| Benchmark  | 0 swaps | 1 swap | 2 swaps  | 3 swaps   |
| ---------- | ------- | ------ | -------- | --------- |
| 1 thread   | 0.1 ms  | 5.3 ms | 147.6 ms | 2537.0 ms |
| 12 threads | 0.6 ms  | 1.7 ms | 30.4 ms  | 478.6 ms  |

> [!NOTE]
> Times shown in table do not include time it takes to load dictionary.
> With new binary dictionary cache, average dictionary load time is `22.7 ms`.
> When program is launched for the first time (dictionary cache isn't yet created), load time is around `~100 ms`.
