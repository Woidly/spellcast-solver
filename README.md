# spellcast-solver

**The fastest solver for Discord Spellcast, written in Rust.**

This project is inspired by WintrCat's [Spellcast solver](https://github.com/WintrCat/spellcastsolver), but is [**more than 10x faster**](#benchmarks) due to several new optimisations and being written in Rust.

## Features

- Fast and efficient solver core that supports multithreading
- Support for all standard game features (DL/TL/2x, 0-3 swaps, gems, frozen tiles)
- Simple CLI for running the solver (you can learn more in [CLI.md](CLI.md))

## Setup

1. Install **Git** and **Rust**.
2. Clone this repository:  
   `git clone https://github.com/Woidly/spellcast-solver.git`
3. Build the project:  
   `RUSTFLAGS="-C target-cpu=native" cargo build --release`
4. Run the solver via CLI:  
   `./target/release/spellcast-solver --help`

## Benchmarks

**Benchmarks aren't ready yet**
