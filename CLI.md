# CLI documentation

```
Usage: spellcast-solver [-d <dictionary>] [-t <threads>] -b <board> [-c <move-count>] [-s <swaps>] [-f <format>]

Spellcast solver CLI. You can learn more about arguments in CLI.md.

Options:
  -d, --dictionary  dictionary file (def=dictionary.txt)
  -t, --threads     number of threads to use (def=1)
  -b, --board       board string
  -c, --move-count  number of top moves to show (def=5)
  -s, --swaps       number of swaps to consider (def=0)
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

### `-f`/`--format`

Output format. Defaults to `simple`.
Possible values:

- `simple`  
  Simple output format that prints each word compactly on a single line.
  For each word it looks something like this:

  > ${\color{white} \text{0. mar} \color{red} \text{s} \color{white} \text{h} \color{red} \text{ma} \color{white} \text{llowy (+44pts, +0 gems) / B1 -> s, B2 -> m, C1 -> a}}$

  Swapped letters will be coloured red.
  If word has swapped letters, they'll also be printed after / in format `A1 -> x`, `x` being new letter and `A1` being chess-like tile notation with letter for column and number for row (e.g. `A1` is top-left tile and `E5` is bottom-right tile).  
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

- `board`  
  Board output format that prints order of steps on board.
  For each word it looks something like this:

  > ```===============|0|===============
  > #   A    B    C    D    E
  >   +----+----+----+----+----+
  > 1 |h  4|a  6|l  7|    |    | marshmallowy
  >   +----+----+----+----+----+
  > 2 |m  5|s  3|r  2|l  8|    | +44 pts, +0 gems
  >   +----+----+----+----+----+
  > 3 |    |a  1|m  0|w 10|o  9| B2 -> s
  >   +----+----+----+----+----+
  > 4 |    |    |    |y 11|    | A2 -> m
  >   +----+----+----+----+----+
  > 5 |    |    |    |    |    | B1 -> a
  >   +----+----+----+----+----+
  > ```

  Swapped letters will be coloured red. Step number will be coloured green.  
  If word has swapped letters, they'll also be printed on the right of board in format `A1 -> x`, `x` being new letter and `A1` being chess-like tile notation with letter for column and number for row (e.g. `A1` is top-left tile and `E5` is bottom-right tile).  
  Boards are shown in reverse order (the best one being at the bottom of terminal with index 0).  
  Note that step number is 0-based.
