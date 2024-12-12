use std::str::FromStr;

/// Spellcast tile.
struct Tile {
    letter: char,
    letter_multiplier: u8,
    word_multiplier: u8,
    gem: bool,
    frozen: bool,
}

impl Tile {
    /// Returns new tile with specified letter that doesn't have any special properties (no multipliers, no gem, not frozen).
    fn empty(letter: char) -> Self {
        Tile {
            letter,
            letter_multiplier: 1,
            word_multiplier: 1,
            gem: false,
            frozen: false,
        }
    }
}

/// Spellcast board.
struct Board {
    tiles: [Tile; 25],
}

impl FromStr for Board {
    type Err = String;

    /// Parses the board string into actual board.
    /// Board string syntax is based on one WintrCat made.
    /// Each tile is represented by a letter than can have postfix consisting of the following characters:
    /// `$` - 2x word multiplier;
    /// `+`/`*` - DL/TL letter multiplier;
    /// `!` - tile has a gem;
    /// `#` - frozen tile;
    /// However, this parser is much more lenient than original.
    /// Any characters other than ones described above will be silently ignored.
    /// This means it is compatible with original format that has newlines and numbers at the bottom.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = vec![];
        for char in s.to_lowercase().chars() {
            if let Some(last) = tiles.last_mut() {
                match char {
                    'a'..='z' => tiles.push(Tile::empty(char)),
                    '$' => last.word_multiplier = 2,
                    '+' => last.letter_multiplier = 2,
                    '*' => last.letter_multiplier = 3,
                    '!' => last.gem = true,
                    '#' => last.frozen = true,
                    _ => (),
                }
            } else if matches!(char, 'a'..='z') {
                tiles.push(Tile::empty(char));
            }
        }
        let count = tiles.len();
        if count != 25 {
            return Err(format!("Expected 25 tiles, but got {count}"));
        }
        Ok(Board {
            tiles: tiles
                .try_into()
                .map_err(|_| "Failed to convert Vec<Tile> to [Tile; 25]".to_string())?,
        })
    }
}
