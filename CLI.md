# CLI documentation

```
Usage: spellcast-solver [-d <dictionary>] [-t <threads>] -b <board> [-c <move-count>] [-s <swaps>] [-v] [--no-colour]

Spellcast solver CLI. You can learn more about arguments in CLI.md.

Options:
  -d, --dictionary  dictionary file (def=dictionary.txt)
  -t, --threads     number of threads to use (def=1)
  -b, --board       board string
  -c, --move-count  number of top moves to show (def=5)
  -s, --swaps       number of swaps to consider (def=0)
  -v, --verbose     print verbose output to stderr
  --no-colour       disable colours in output
  --help            display usage information
```

## Arguments

### `-d`/`--dictionary`

Path to the dictionary file. Defaults to `dictionary.txt`. Dictionary format is just a list of words separated by newlines. Only 3-25 character words are used.

### `-t`/`--threads`

Number of threads to use for solver. Defaults to `1`.  
**Bewarb, not all threads are created equal!**  
While initial calls are distributed evenly between threads, these calls aren't even by their nature.
Some calls may quickly return, while other might search bit longer.  
Therefore, while using multiple threads improves performance a lot, you shouldn't expect 10x speedup from using 10 threads.

### `-b`/`--board`

Board string. It's a required argument.  
Board string syntax is based on one WintrCat made.
Each tile is represented by a (case-insensitive) letter than can have postfix consisting of the following characters:

- `$` - 2x word multiplier
- `+`/`*` - DL/TL letter multiplier
- `!` - tile has a gem
- `#` - frozen tile

Any characters other than ones described above will be silently ignored.
This means it is compatible with original format that has newlines and numbers at the bottom.

### `-c`/`--move-count`

Number of top moves to show. Defaults to `5`.
However, you can (and will) get less moves than this number (especially with low [`MAX_SOLUTIONS`](src/utils.rs#L5)).

### `-s`/`--swaps`

Number of swaps to consider. Defaults to `0`.
Basically a number of gems you currently have divided by 3 and rounded down.

### `-v`/`--verbose`

Whether to print verbose output. It's a switch, therefore it's `false` (no verbose output) unless it's specified.
As of now, it just prints time it took to load dictionary and solve the board. It is printed to stderr, therefore it won't disrupt any automated parsing of stdout.

### `--no-colour`

Whether to disable colours in output. It's a switch, therefore it's `false` (colours are enabled) unless it's specified.
As of now, colours are used only to highlight which letters have been swapped (in red). If colours are disabled, swapped letters will be put in \[square brackets\] instead.
