use random::Source as _;

use crate::{
    dictionary::{LookupResult, DICTIONARY_CELL},
    utils::*,
};

const DELTAS: [i8; 3] = [-1, 0, 1];
const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

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

pub struct Tile {
    pub letter: char,
    pub letter_multiplier: u8,
    pub word_multiplier: u8,
    pub gem: bool,
    pub frozen: bool,
}

impl Tile {
    pub fn empty(letter: char) -> Self {
        Tile {
            letter,
            letter_multiplier: 1,
            word_multiplier: 1,
            gem: false,
            frozen: false,
        }
    }
}

#[derive(Clone)]
pub enum Move {
    Normal { index: i8 },
    Swap { index: i8, new_letter: char },
}

impl Move {
    pub fn index(&self) -> i8 {
        match self {
            Move::Normal { index } => *index,
            Move::Swap { index, .. } => *index,
        }
    }

    pub fn letter(&self, board: &Board) -> char {
        match self {
            Move::Normal { index } => board.tiles[*index as usize].letter,
            Move::Swap { new_letter, .. } => *new_letter,
        }
    }
}

pub struct Board {
    pub tiles: [Tile; 25],
    pub gem_bonus: u16,
}

#[derive(Debug)]
pub struct ParseBoardError {}

impl std::fmt::Display for ParseBoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid board (most likely, wrong number of tiles)")
    }
}

impl std::error::Error for ParseBoardError {}

impl std::str::FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Tile> = vec![];
        for char in str.to_lowercase().chars() {
            if let Some(last_tile) = grid.last_mut() {
                match char {
                    'a'..='z' => {
                        grid.push(Tile::empty(char));
                    }
                    '$' => {
                        last_tile.word_multiplier = 2;
                    }
                    '^' => {
                        last_tile.word_multiplier = 3;
                    }
                    '+' => {
                        last_tile.letter_multiplier = 2;
                    }
                    '*' => {
                        last_tile.letter_multiplier = 3;
                    }
                    '!' => {
                        last_tile.gem = true;
                    }
                    '#' => {
                        last_tile.frozen = true;
                    }
                    _ => (),
                }
            } else {
                match char {
                    'a'..='z' => {
                        grid.push(Tile::empty(char));
                    }
                    _ => {}
                }
            }
        }
        Ok(Board {
            tiles: grid.try_into().map_err(|_| ParseBoardError {})?,
            gem_bonus: 0,
        })
    }
}

impl Board {
    /// Generates a random board for use in benchmark.
    pub fn random(
        rng: &mut random::Default,
        do_gems: bool,
        do_double_letter: bool,
        do_double_word: bool,
    ) -> Board {
        // TODO: Maybe switch to another random crate. But the popular "rand" seems to have too much dependencies.
        let mut tiles =
            std::array::from_fn(|_| Tile::empty(LETTERS[(rng.read_f64() * 26.) as usize]));
        if do_gems {
            let mut indexes = vec![];
            while indexes.len() < 10 {
                let index = rng.read_u64() as usize % 25;
                if !indexes.contains(&index) {
                    indexes.push(index);
                }
            }
            for index in indexes {
                tiles[index].gem = true;
            }
        }
        if do_double_letter {
            tiles[rng.read_u64() as usize % 25].letter_multiplier = 2;
        }
        if do_double_word {
            tiles[rng.read_u64() as usize % 25].word_multiplier = 2;
        }
        Board {
            tiles,
            gem_bonus: 0,
        }
    }

    /// Sort of wrapper for actual solver. It creates initial moves consisting of just a single tile.
    /// One Normal move with tile's original letter and (if we have swaps) Swap moves for rest of the letters.
    /// This function just returns raw Vec<Word>, all the processing is done in program entry.
    pub fn solve(&self, swaps: u8) -> Vec<Word> {
        let mut words = vec![];
        // TODO: Multi-threading (and maybe command line argument to configure amount of threads).
        let mut index = 0;
        for tile in &self.tiles {
            if tile.frozen {
                continue;
            }
            words.extend(new_solver(
                self,
                vec![Move::Normal { index }],
                tile.letter.to_string(),
                swaps,
            ));
            if swaps > 0 {
                for new_letter in LETTERS {
                    if new_letter == tile.letter {
                        continue;
                    }
                    words.extend(new_solver(
                        self,
                        vec![Move::Swap { index, new_letter }],
                        new_letter.to_string(),
                        swaps - 1,
                    ));
                }
            }
            index += 1;
        }
        words
    }
}

/// Struct used to represent a sequence of moves that forms a valid word.
/// It exists to cache String representation of a word and resulting score.
pub struct Word {
    pub gems: u8,
    pub moves: Vec<Move>,
    pub score: u16, // Using u16 for score just in case of some miracle overflow
    pub swap_count: u8,
    pub word: String,
    pub word_formatted: String,
}

impl Word {
    fn new(moves: Vec<Move>, board: &Board, word: String) -> Word {
        let mut gems = 0;
        let mut score = 0;
        let mut swap_count = 0;
        let mut word_formatted = String::new();
        let mut word_multiplier = 1;
        for m in &moves {
            match m {
                Move::Normal { index } => {
                    let tile = &board.tiles[*index as usize];
                    score += (get_letter_points(tile.letter) * tile.letter_multiplier) as u16;
                    word_formatted += &tile.letter.to_string();
                    word_multiplier = word_multiplier.max(tile.word_multiplier as u16);
                    if tile.gem {
                        score += board.gem_bonus;
                        gems += 1;
                    }
                }
                Move::Swap { index, new_letter } => {
                    let tile = &board.tiles[*index as usize];
                    score += (get_letter_points(*new_letter) * tile.letter_multiplier) as u16;
                    word_formatted += &format!("{RED}{new_letter}{RESET}");
                    word_multiplier = word_multiplier.max(tile.word_multiplier as u16);
                    if tile.gem {
                        score += board.gem_bonus;
                        gems += 1;
                    }
                    swap_count += 1;
                }
            }
        }
        score *= word_multiplier;
        if moves.len() >= 6 {
            score += 10;
        }
        Word {
            gems,
            moves,
            score,
            swap_count,
            word,
            word_formatted,
        }
    }
}

/// The solver itself.
/// Initial calls to this function are from Board::solve and contain only one move.
/// Then it just calls itself recursively with longer and longer move sequences until word is found or branch is cut.
/// The further down, the faster it becomes! For example, "e" can be followed by 24 different letters, but "ea" - only by 6.
fn new_solver(board: &Board, init_sequence: Vec<Move>, word: String, swaps: u8) -> Vec<Word> {
    let dictionary = DICTIONARY_CELL.get().unwrap();
    let mut words = vec![];
    if let Some(last_move) = init_sequence.last() {
        let index = last_move.index();
        let old_moves: Vec<i8> = (&init_sequence).into_iter().map(|m| m.index()).collect();
        if let Some(result) = dictionary.get(&word.as_str()) {
            let real_next_letters: &Vec<char>;
            match result {
                LookupResult::Word => {
                    words.push(Word::new(init_sequence, board, word));
                    return words;
                }
                LookupResult::Both { next_letters } => {
                    // FIXME: Maybe it's possible to avoid cloning? It isn't really significant, but would be nice to get rid of it.
                    // Can't move because for whatever reason borrow for dict.get() is held entire lifetime of returned value (prolonged by next_letters), even tho key is needed only when lookup happens.
                    words.push(Word::new(init_sequence.clone(), board, word.clone()));
                    real_next_letters = next_letters;
                }
                LookupResult::Prefix { next_letters } => {
                    real_next_letters = next_letters;
                }
            }
            let x = index % 5;
            let y = index / 5;
            for dx in DELTAS {
                for dy in DELTAS {
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
                    let original_letter_match = real_next_letters.contains(&tile.letter);
                    if swaps > 0 {
                        for letter in real_next_letters {
                            if *letter == tile.letter {
                                continue;
                            } // Skip original letter. It's already here, no need to waste a swap on it.
                            let mut tmp_word = (&word).clone();
                            tmp_word.push(*letter);
                            let mut tmp_sequence = (&init_sequence).clone();
                            tmp_sequence.push(Move::Swap {
                                index: ni,
                                new_letter: *letter,
                            });
                            words.extend(new_solver(board, tmp_sequence, tmp_word, swaps - 1));
                        }
                    }
                    if !original_letter_match {
                        continue;
                    }
                    let mut tmp_word = (&word).clone();
                    tmp_word.push(tile.letter);
                    let mut tmp_sequence = (&init_sequence).clone();
                    tmp_sequence.push(Move::Normal { index: ni });
                    words.extend(new_solver(board, tmp_sequence, tmp_word, swaps));
                }
            }
        } else {
            // This is dead branch, no reason to continue search
            return words;
        }
    }
    words
}
