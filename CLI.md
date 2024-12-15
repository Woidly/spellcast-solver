# CLI documentation

```
Usage: spellcast-solver [-d <dictionary>] [-t <threads>] -b <board> [-c <move-count>] [-s <swaps>] [--no-colour] [-f <format>]

Spellcast solver CLI. You can learn more about arguments in CLI.md.

Options:
  -d, --dictionary  dictionary file (def=dictionary.txt)
  -t, --threads     number of threads to use (def=1)
  -b, --board       board string
  -c, --move-count  number of top moves to show (def=5)
  -s, --swaps       number of swaps to consider (def=0)
  --no-colour       disable colours in output
  -f, --format      output format (def=simple)
  --help            display usage information
```

## Arguments

### `-d`/`--dictionary`

Path to the dictionary file. Defaults to `dictionary.txt`. Dictionary format is just a list of words separated by newlines. Only 3-25 character words are used.

### `-t`/`--threads`

Number of threads to use for solver. Defaults to `1`.  
**Bewarb, not all threads are created equal!**  
While initial calls are distributed evenly between threads, these calls aren't even by their nature.
Some calls may complete quickly, while others might take longer to process.  
Therefore, while using multiple threads significantly improves performance, you shouldn't expect 10x speedup from using 10 threads.

### `-b`/`--board`

Board string. It's a required argument.  
Board string syntax is based on one WintrCat made.
Each tile is represented by a (case-insensitive) letter that can have postfix consisting of the following characters:

- `$` - 2x word multiplier
- `+`/`*` - DL/TL letter multiplier
- `!` - tile has a gem
- `#` - frozen tile

Any characters other than ones described above will be silently ignored.
This means it is compatible with original format that has newlines and numbers at the bottom.

### `-c`/`--move-count`

Number of top moves to show. Defaults to `5`.
However, you can (and will) get fewer moves than this number (especially with low [`MAX_SOLUTIONS`](src/utils.rs#L5)).

### `-s`/`--swaps`

Number of swaps to consider. Defaults to `0`.
Basically a number of gems you currently have divided by 3 and rounded down.

### `--no-colour`

Whether to disable colours in output. It's a switch, therefore it's `false` (colours are enabled) unless it's specified.
As of now, colours are used only to highlight which letters have been swapped (in red). If colours are disabled, swapped letters will be put in \[square brackets\] instead.

### `-f`/`--format`

Output format. Defaults to `simple`.
Possible values:

- `simple`  
   Simple output format that prints each word compactly on a single line.
  For each word it looks something like this:

  > 0.  mar**s**h**ma**llowy (+44pts, +0 gems, -3 swaps)

  Swapped letters (shown here in bold) will either be coloured red or (with `--no-colour`) put in \[square brackets\].
  Words are shown in reverse order (the best one being at the bottom of terminal with index 0).

- `json`  
   JSON output format that is intended for automation purposes.
  It looks something like this (I prettied it for clarity, but it's compact in actual output):

  ```json
  {
    "elapsed_ms": { "dict": 77.8, "solver": 837.0 },
    "words": [
      {
        "gems_collected": 0,
        "steps": [
          { "swap": false, "index": 12 },
          { "swap": false, "index": 11 },
          { "swap": false, "index": 7 },
          { "swap": true, "index": 1, "new_letter": "s" }
          /* More steps here... */
        ],
        "score": 44,
        "swaps_used": 3,
        "word": "marshmallowy"
      }
      /* More words here... */
    ]
  }
  ```

  It has the following structure:

  - `elapsed_ms` - time (in milliseconds) spent in different parts of the program:
    - `dict` - time spent loading the dictionary
    - `solver` - time spent solving the board
  - `words` - array of top words. Each item is as follows:
    - `gems_collected` - number of gems collected with this word
    - `steps` - array of steps needed to play the word. Each item is as follows:
      - `swap` - boolean indicating whether this step swaps a letter
      - `index` - 0-based flat index of tile (`0` being top-left tile, `24` being bottom-right tile)
      - `new_letter` - _(optional)_ if `swap` is true, single-char string indicating new letter
    - `score` - score you'll get with this word
    - `swaps_used` - number of swaps used
    - `word` - string representing the actual word
