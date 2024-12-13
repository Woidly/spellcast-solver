use std::str::FromStr;

use crate::{
    dictionary::{Dictionary, LookupResult},
    utils::MAX_SOLUTIONS,
};

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
#[derive(Clone)]
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

/// Wrapper for Vec<Word> that keeps only [crate::utils::MAX_SOLUTIONS] highest value items and is always sorted.
pub struct SortedWordVec {
    inner: Vec<Word>,
}

impl SortedWordVec {
    pub fn new() -> SortedWordVec {
        SortedWordVec {
            inner: Vec::with_capacity(MAX_SOLUTIONS + 1), // Add 1 because it temporary exceeds limit by 1 inside self.push.
        }
    }

    /// Inserts value into inner Vec into position determined by binary search.
    /// If it becomes longer than [crate::utils::MAX_SOLUTIONS], last item (with smallest value) is popped.
    /// After function returns, self.inner is guaranteed to be sorted and <= [crate::utils::MAX_SOLUTIONS] in length.
    pub fn push(&mut self, value: Word) {
        let mut l = 0;
        let mut r = self.inner.len();
        let mut m;
        while l < r {
            m = (l + r) / 2;
            if self.inner[m].score > value.score {
                l = m + 1;
            } else if self.inner[m].score == value.score {
                l = m;
                break;
            } else {
                r = m;
            }
        }
        self.inner.insert(l, value);
        if self.inner.len() > MAX_SOLUTIONS {
            self.inner.pop();
        }
    }
}

fn solver(
    board: &Board,
    steps: &mut Vec<Step>,
    word: &mut String,
    swaps: u8,
    words: &mut SortedWordVec,
    dictionary: &Dictionary,
) {
    let last_step = steps.last().expect("`steps` should have at least one item");
    let last_index = last_step.index();
    let old_moves: Vec<i8> = steps.into_iter().map(|m| m.index()).collect();
    // For whatever weird reason key remains borrowed even after lookup is done (I believe, borrow for key is held while we hold borrow for value).
    // Therefore, we need to clone it, otherwise we won't be able to mutate it below.
    let temp = word.clone();
    let this = dictionary
        .get(&temp.as_str())
        .expect("`word` should be a valid prefix/word");
    let final_next_letters;
    // TODO: Maybe pre-build the tree of next letter results?
    // Then it'll be possible to just pass down a single lookup done in initial solver() call for a single letter.
    // In theory it should bring number of dictionary lookups down from multiple millions (worst case, 3 swaps) to just 25 (0 swaps) or 650 (1+ swaps).
    // Also, it'll probably get rid of some cloning.
    match this {
        LookupResult::Word => return words.push(Word::new(steps.clone(), board, word.clone())),
        LookupResult::Both { next_letters } => {
            words.push(Word::new(steps.to_owned().clone(), board, word.clone()));
            final_next_letters = next_letters;
        }
        LookupResult::Prefix { next_letters } => final_next_letters = next_letters,
    }
    let x = last_index % 5;
    let y = last_index / 5;
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx > 4 || ny < 0 || ny > 4 {
                continue;
            }
            let ni = ny * 5 + nx;
            let tile = &board.tiles[ni as usize];
            if tile.frozen || old_moves.contains(&ni) {
                continue;
            }
            let original_letter_match = final_next_letters.contains(&tile.letter);
            if swaps > 0 {
                for letter in final_next_letters {
                    // Skip original letter. It's already here, no need to waste a swap on it.
                    if *letter == tile.letter {
                        continue;
                    }
                    steps.push(Step::Swap {
                        index: ni,
                        new_letter: *letter,
                    });
                    word.push(*letter);
                    solver(board, steps, word, swaps - 1, words, dictionary);
                    steps.pop();
                    word.pop();
                }
            }
            if !original_letter_match {
                continue;
            }
            steps.push(Step::Normal { index: ni });
            word.push(tile.letter);
            solver(board, steps, word, swaps, words, dictionary);
            steps.pop();
            word.pop();
        }
    }
}
