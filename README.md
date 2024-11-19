# SpellcastSolver

It's a... Spellcast solver... isn't the name obvious? (It's currently WIP.)

This project is inspired by WintrCat's Spellcast solver [[YouTube video]](https://youtu.be/hYoojWO9hh8) [[YouTube video 2]](https://youtu.be/QvJATba04u8) [[GitHub repo]](https://github.com/WintrCat/spellcastsolver) (but mine is faster cause Rust + several new optimizations).

## Stuff

It has most of the original features:

- Considering tile boosts when calculating score (DL, TL, 2x, 3x, long word bonus)
- Frozen tiles
- 1-3 tile swaps
- Ability to prioritise gems (configurable)

But some things are missing:

- No benchmarks (WIP)
- Doesn't recommend shuffling (use your brain to figure out when to shuffle)
- It doesn't make any assumptions (no "estimated values"), it just works with current data (in most cases results are the same as in original solver)

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
Basic colour support in terminal is recommended.
You can learn move about command line arguments in [CLI.md](CLI.md).

## Benchmarks

**As of now, benchmarks aren't implemented.**
