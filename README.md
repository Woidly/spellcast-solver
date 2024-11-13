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

- Doesn't recommend shuffling (use your brain to figure out when to shuffle)
- No "gem management" (use your brain to figure out when to enable gem prioritising)
- It doesn't make any assumptions (no "estimated values"), it just works with current data (in most cases results are the same as in original solver)

And some things I planned aren't even made yet:

- Interactive mode (to make it actually useful in game)
- Benchmarks
- Multi-threading

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
By running help, you will get the following message:

```
Usage: spellcast-solver <command> [<args>]

An Spellcast solver. README.md has more detailed info on arguments.

Options:
  --help            display usage information

Commands:
  benchmark         run the benchmark
  interactive       run the interactive solver
  solver            run the old no-state solver
```

Below are detailed descriptions of each sub-command and its arguments:

#### `benchmark`

Read [Benchmarks](#benchmarks) section for more info.

#### `interactive`

An interactive solver that is good enough to actually use it in game.
No command-line arguments are required; everything is configured in TUI.
TODO: Make a proper documentation of TUI when it's done, preferably in separate file.

#### `solver`

It's basically what you expected from original project - a simple solver that takes certain board position and gives you best words.

##### Board format

Format of board.txt (or the board command line argument) is basically the same as in original. Example:

```
W!+A!L$ER
O!FE!EK
OADN!E
R!N!R!NV!
IE!WCI
```

You just type the entire board, optionally suffixing letters with certain symbols (you can use multiple symbols after one letter):

- `$` - double word score (2x) (make sure to escape it when running from the command line)
- `^` - triple word score (3x) (it was listed in the wiki, so I added it)
- `+` - double letter score (DL)
- `*` - triple letter score (TL)
- `!` - this tile has gem
- `#` - this tile can't be used ("frozen")

Newlines are completely optional, so you can type entire board in one line.
It is especially useful for command line.

#### Arguments

- `-b`/`--board` - board string (defaults to reading `board.txt`)
- `-c`/`--move-count` - number of top moves to show (defaults to `5`)
- `-g`/`--gem-value` - value added to tiles with gems to prioritise them (defaults to `0`, e.g. you can skip adding `!` because it does nothing)
- `-p`/`--pretty-print` - whether to pretty print moves (print entire board with tiles highlighted instead of just printing text instructions; terminal with minimal colour support is recommended) (it's a flag, defaults to `false` if not specified)
- `-s`/`--swap-count` - number of swaps to consider (each one costs 3 gems for you) (defaults to `0`)

For example, if you have two swaps, want to somewhat prioritise gems (+3 per gem), want to pretty-print the moves and want to specify board in command line, you can run `./target/release/spellcast-solver solver -p -g 3 -s 2 -b "board string here"`

Dictionary is loaded from [src/dictionary.txt](src/dictionary.txt) at compile time. The default one (taken from original project) should be good enough. But if you want to reduce load time (I mean, current time is still much faster than python), you can just recompile with smaller dictionary (like the 64k one from 12dicts, I was using it since beginning of the project before switching to original one).

## Benchmarks

**As of now, benchmarks aren't implemented.**
