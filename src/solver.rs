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
    /// If it becomes longer than MAX_SOLUTIONS, last item (with smallest value) is popped.
    /// After function returns, self.inner is guaranteed to be sorted and <= 100 in length.
    pub fn push(&mut self, value: Word) {
        let mut l = 0;
        let mut r = self.inner.len();
        let mut m;
        while l < r {
            m = (l + r) / 2;
            if self.inner[m].sorting_score > value.sorting_score {
                l = m + 1;
            } else if self.inner[m].sorting_score == value.sorting_score {
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

impl std::default::Default for Board {
    fn default() -> Self {
        Self {
            tiles: std::array::from_fn(|_| Tile::empty('?')),
            gem_bonus: Default::default(),
        }
    }
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
    /// Consumes the board, solves it, and returns it back with solutions.
    /// Just a wrapper for actual solver that manages multi-threading.
    /// Why the consume stuff? Because borrow checker.
    /// Like it's the only solution I could find.
    /// Maybe I'll find a better solution one day.
    // FIXME: Find a better solution for multi-threading.
    pub fn solve(self, swaps: u8, num_threads: u8) -> (Vec<Word>, Self) {
        let mut calls = vec![];
        let mut words = SortedWordVec::new();
        let mut index = -1;
        for tile in &self.tiles {
            index += 1;
            if tile.frozen {
                continue;
            }
            calls.push((vec![Move::Normal { index }], tile.letter.to_string(), swaps));
            if swaps > 0 {
                for new_letter in LETTERS {
                    if new_letter == tile.letter {
                        continue;
                    }
                    calls.push((
                        vec![Move::Swap { index, new_letter }],
                        new_letter.to_string(),
                        swaps - 1,
                    ));
                }
            }
        }
        if num_threads <= 1 {
            for call in calls {
                new_solver(&self, call.0, call.1, call.2, &mut words);
            }
            return (words.inner, self);
        } else {
            // Let's hope bad stuff doesn't happen because of unsafe.
            // I mean, all threads are guaranteed to finish at the point of getting back self from Box.
            // It's just a silly little trick to convince borrow checker that threads can indeed spawn.
            let board_ptr = Box::into_raw(Box::new(self));
            let mut threads = vec![];
            let chunk_size = (calls.len() + num_threads as usize - 1) / num_threads as usize;
            {
                let board_ref: &'static Board = unsafe { &*board_ptr };
                while !calls.is_empty() {
                    let chunk = calls
                        .drain(..chunk_size.min(calls.len()))
                        .collect::<Vec<_>>(); // Bit hackish, but at least it doesn't do cloning.
                    threads.push(std::thread::spawn(|| {
                        let mut thread_words = SortedWordVec::new();
                        for call in chunk {
                            new_solver(board_ref, call.0, call.1, call.2, &mut thread_words);
                        }
                        thread_words
                    }))
                }
                for thread in threads {
                    if let Ok(thread_words) = thread.join() {
                        for word in thread_words.inner {
                            words.push(word);
                        }
                    }
                }
            }
            return (words.inner, unsafe { *Box::from_raw(board_ptr) });
        }
    }
}

/// Struct used to represent a sequence of moves that forms a valid word.
/// It exists to cache String representation of a word and resulting score.
pub struct Word {
    pub gems: u8,
    pub moves: Vec<Move>,
    pub score: u16, // Using u16 for score just in case of some miracle overflow.
    pub sorting_score: u16,
    pub swap_count: u8,
    pub word: String,
}

impl Word {
    fn new(moves: Vec<Move>, board: &Board, word: String) -> Word {
        let mut gems = 0;
        let mut score = 0;
        let mut swap_count = 0;
        let mut word_multiplier = 1;
        for m in &moves {
            match m {
                Move::Normal { index } => {
                    let tile = &board.tiles[*index as usize];
                    score += (get_letter_points(tile.letter) * tile.letter_multiplier) as u16;
                    word_multiplier = word_multiplier.max(tile.word_multiplier as u16);
                    if tile.gem {
                        gems += 1;
                    }
                }
                Move::Swap { index, new_letter } => {
                    let tile = &board.tiles[*index as usize];
                    score += (get_letter_points(*new_letter) * tile.letter_multiplier) as u16;
                    word_multiplier = word_multiplier.max(tile.word_multiplier as u16);
                    if tile.gem {
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
            sorting_score: score + gems as u16 * board.gem_bonus,
            swap_count,
            word,
        }
    }

    pub fn formatted(&self, board: &Board) -> String {
        let mut word_formatted = String::new();
        for m in &self.moves {
            match m {
                Move::Normal { index } => {
                    let tile = &board.tiles[*index as usize];
                    word_formatted += &tile.letter.to_string();
                }
                Move::Swap { new_letter, .. } => {
                    word_formatted += &format!("{RED}{new_letter}{RESET}");
                }
            }
        }
        word_formatted
    }
}

/// The solver itself.
/// Initial calls to this function are from Board::solve and contain only one move.
/// Then it just calls itself recursively with longer and longer move sequences until word is found or branch is cut.
/// The further down, the faster it becomes! For example, "e" can be followed by 24 different letters, but "ea" - only by 6.
fn new_solver(
    board: &Board,
    init_sequence: Vec<Move>,
    word: String,
    swaps: u8,
    words: &mut SortedWordVec,
) {
    let dictionary = DICTIONARY_CELL.get().unwrap();
    if let Some(last_move) = init_sequence.last() {
        let index = last_move.index();
        let old_moves: Vec<i8> = (&init_sequence).into_iter().map(|m| m.index()).collect();
        // TODO: Maybe store whether next letter is a prefix, word or both prefix and word in next_letters. It will help to avoid this dictionary lookup.
        if let Some(result) = dictionary.get(&word.as_str()) {
            let real_next_letters: &Vec<char>;
            match result {
                LookupResult::Word => {
                    words.push(Word::new(init_sequence, board, word));
                    return;
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
                            new_solver(board, tmp_sequence, tmp_word, swaps - 1, words);
                        }
                    }
                    if !original_letter_match {
                        continue;
                    }
                    let mut tmp_word = (&word).clone();
                    tmp_word.push(tile.letter);
                    let mut tmp_sequence = (&init_sequence).clone();
                    tmp_sequence.push(Move::Normal { index: ni });
                    new_solver(board, tmp_sequence, tmp_word, swaps, words);
                }
            }
        }
    }
}
