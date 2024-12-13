use std::str::FromStr;

/// Returns points given for a specific letter.
fn get_letter_points(letter: char) -> u8 {
    match letter {
        'a' | 'e' | 'i' | 'o' => 1,
        'n' | 'r' | 's' | 't' => 2,
        'd' | 'g' | 'l' => 3,
        'b' | 'h' | 'p' | 'm' | 'u' | 'y' => 4,
        'c' | 'f' | 'v' | 'w' => 5,
        'k' => 6,
        'j' | 'x' => 7,
        'q' | 'z' => 8,
        _ => 0,
    }
}

/// Spellcast tile.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Board {
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

/// Single step in word.
pub enum Step {
    /// Use tile @index as is.
    Normal { index: i8 },
    /// Swap tile @index to new_letter, then use.
    Swap { index: i8, new_letter: char },
}

impl Step {
    fn index(&self) -> i8 {
        match self {
            Self::Normal { index } => *index,
            Self::Swap { index, .. } => *index,
        }
    }

    fn letter(&self, board: &Board) -> char {
        match self {
            Self::Normal { index } => board.tiles[*index as usize].letter,
            Self::Swap { new_letter, .. } => *new_letter,
        }
    }
}

/// Struct that stores sequence of steps needed to form the word and word metadata.
pub struct Word {
    pub gems_collected: u8,
    pub score: u16, // Using u16 for score just in case of some miracle overflow.
    pub steps: Vec<Step>,
    pub swaps_used: u8,
    pub word: String,
}

impl Word {
    /// Calculates score and metadata for sequence of steps and returns new instance of Word.
    fn new(steps: Vec<Step>, board: &Board, word: String) -> Word {
        let mut gems_collected = 0;
        let mut score = 0;
        let mut swaps_used = 0;
        let mut word_multiplier = 1;
        for step in &steps {
            let tile = &board.tiles[step.index() as usize];
            score += (get_letter_points(step.letter(board)) * tile.letter_multiplier) as u16;
            word_multiplier = word_multiplier.max(tile.word_multiplier as u16);
            if tile.gem {
                gems_collected += 1;
            }
            if matches!(step, Step::Swap { .. }) {
                swaps_used += 1;
            }
        }
        score *= word_multiplier;
        if steps.len() >= 6 {
            score += 10;
        }
        Word {
            gems_collected,
            score,
            steps,
            swaps_used,
            word,
        }
    }
}
