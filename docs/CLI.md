# Command line help

By running help, you will get the following message:

```
Usage: spellcast-solver [-d <dictionary>] [-t <threads>] <command> [<args>]

An Spellcast solver. docs/CLI.md has more detailed info on arguments.

Options:
  -d, --dictionary  dictionary file
  -t, --threads     number of threads to use (def=1)
  --help            display usage information

Commands:
  benchmark         run the benchmark
  interactive       run the interactive solver
  solver            run the solver CLI
```

## Arguments

### `--dictionary`

Path to dictionary file. If this argument isn't specified, it defaults to `dictionary.txt`. The [one](dictionary.txt) included in this repo (taken from original project) should be good enough. But if you want to reduce load time (I mean, current time is still much faster than python), you can just use smaller dictionary (like the 64k one from 12dicts, I was using it since beginning of the project before switching to original one). The dictionary file is literally words separated by newlines, only 3-25 character words are loaded.

### `--threads`

Number of threads to use for solving board.
**Bewarb, not all threads are created equal!**
While initial calls are indeed distributed evenly between threads, these calls aren't even by their nature.
Boards are random, so you shouldn't expect `Move::Swap {index: 13, new_letter: 'f'}` to always return exactly 102 possible move sequences after itself`.
Therefore, while using multiple threads improves performance a lot, you shouldn't expect 10x speedup from using 10 threads.

## Subcommands

### `benchmark`

**As of now, built-in benchmark isn't implemented yet due to technical difficulties.**
**You can find benchmark results in respective [section](README.md#benchmarks) of README.**

### `interactive`

An interactive solver that is good enough to actually use it in game.
No specific command-line arguments are required (except for dictionary and number of threads, these are program-wide); everything is configured in TUI.
You can learn more about TUI itself in [docs/INTERACTIVE.md](docs/INTERACTIVE.md)

### `solver`

A simple CLI that allows to access solver.
It takes a certain board position and solves it.

#### Arguments

- `-b`/`--board` - board string (defaults to reading `board.txt`)
- `-c`/`--move-count` - number of top moves to show (defaults to `5`)
- `-g`/`--gem-value` - value added to tiles with gems to prioritise them (defaults to `0`, e.g. you can skip adding `!` because it does nothing)
- `-p`/`--pretty-print` - whether to pretty print moves (print entire board with tiles highlighted instead of just printing text instructions; terminal with minimal colour support is recommended) (it's a flag, defaults to `false` if not specified)
- `-s`/`--swap-count` - number of swaps to consider (each one costs 3 gems for you) (defaults to `0`)

#### Board format

Format of board.txt (or the board command line argument) is basically the same as in original. Example:

```
W!+A!L$ER
O!FE!EK
OADN!E
R!N!R!NV!
IE!WCI
```

You just type the entire board (case-insensitive), optionally suffixing letters with certain symbols (you can use multiple symbols after one letter):

- `$` - double word score (2x) (make sure to escape it when running from the command line)
- `^` - triple word score (3x) (it was listed in the wiki, so I added it)
- `+` - double letter score (DL)
- `*` - triple letter score (TL)
- `!` - this tile has gem
- `#` - this tile can't be used ("frozen")

Newlines are completely optional, so you can type entire board in one line.
It is especially useful for command line.

#### Examples

For example, if you have two swaps, want to somewhat prioritise gems (+2 per gem), want to pretty-print the moves and want to specify board in command line, you can run `./target/release/spellcast-solver solver -p -g 2 -s 2 -b "board string here"`.
