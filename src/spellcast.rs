use std::str::FromStr;

use crate::{dictionary::Node, utils::MAX_SOLUTIONS};

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

/// Single step in forming a word.
#[derive(Clone, Debug)]
pub enum Step {
    /// Use tile @ `self.index` as is.
    Normal { index: i8 },
    /// Swap tile @ `self.index` to `self.new_letter`, then use.
    Swap { index: i8, new_letter: char },
}

impl Step {
    /// Returns `self.index`.
    /// Just a convenience function that handles matching and dereferencing.
    fn index(&self) -> i8 {
        match self {
            Self::Normal { index } => *index,
            Self::Swap { index, .. } => *index,
        }
    }

    /// Returns the letter represented by this Step.
    /// For Normal, it's letter from board tile @ `self.index`.
    /// For Swap, it's `self.new_letter`.
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
}

impl Word {
    /// Calculates score and metadata for sequence of steps and returns new instance of Word.
    fn new(steps: Vec<Step>, board: &Board) -> Word {
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
        }
    }

    /// Returns actual word string.
    pub fn word(&self, board: &Board) -> String {
        let mut buf = String::new();
        for step in &self.steps {
            buf += &step.letter(board).to_string();
        }
        buf
    }
}

/// Wrapper for Vec<Word> that keeps only [crate::utils::MAX_SOLUTIONS] highest value items and is always sorted.
pub struct SortedWordVec {
    inner: Vec<Word>,
}

impl SortedWordVec {
    /// Creates empty SortedWordVec.
    pub fn new() -> SortedWordVec {
        SortedWordVec {
            inner: Vec::with_capacity(MAX_SOLUTIONS + 1), // Add 1 because it temporary exceeds limit by 1 inside self.push.
        }
    }

    /// Inserts value into inner Vec into position determined by binary search.
    /// If it becomes longer than [crate::utils::MAX_SOLUTIONS], last item (with smallest value) is popped.
    /// After function returns, `self.inner` is guaranteed to be sorted and <= [crate::utils::MAX_SOLUTIONS] in length.
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

/// Recursively solves the board starting from `node`and adds found words to `words` [SortedWordVec].
/// `steps` is used to avoid duplicate steps and determine current position on board.
/// If `swaps` is not 0, additional calls with `swaps` reduced by 1 and `Step::Swap` for remaining next letters are created.
fn solver(board: &Board, steps: &mut Vec<Step>, node: &Node, swaps: u8, words: &mut SortedWordVec) {
    let last_step = steps.last().expect("`steps` should have at least one item");
    let last_index = last_step.index();
    let old_moves: Vec<i8> = steps.into_iter().map(|m| m.index()).collect();
    let final_next_letters;
    match node {
        Node::Word => return words.push(Word::new(steps.clone(), board)),
        Node::Both { next_letters } => {
            words.push(Word::new(steps.to_owned().clone(), board));
            final_next_letters = next_letters;
        }
        Node::Prefix { next_letters } => final_next_letters = next_letters,
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
            for (letter, sub_node) in final_next_letters {
                if *letter == tile.letter {
                    steps.push(Step::Normal { index: ni });
                    solver(board, steps, sub_node, swaps, words);
                    steps.pop();
                } else if swaps > 0 {
                    steps.push(Step::Swap {
                        index: ni,
                        new_letter: *letter,
                    });
                    solver(board, steps, sub_node, swaps - 1, words);
                    steps.pop();
                }
            }
        }
    }
}

/// Wrapper that creates initial [solver] calls and handles multithreading.
/// For each tile initial calls consist of `Step::Normal` for original letters and (if swaps are available) `Step::Swap` for rest of letters.
/// It takes ownership of board because of how multithreading is implemented, but it is returned back alongside with solving results.
/// Returned words are automatically sorted thanks to [SortedWordVec].
pub fn solver_wrapper(
    board: Board,
    swaps: u8,
    thread_count: u8,
    dictionary: &'static Vec<(char, Node)>,
) -> (Vec<Word>, Board) {
    let mut calls = vec![];
    let mut words = SortedWordVec::new();
    for (index, tile) in (&board.tiles).into_iter().enumerate() {
        let index = index as i8;
        if tile.frozen {
            continue;
        }
        for (new_letter, node) in dictionary {
            if *new_letter == tile.letter {
                calls.push((vec![Step::Normal { index }], node, swaps));
                continue;
            } else if swaps > 0 {
                calls.push((
                    vec![Step::Swap {
                        index,
                        new_letter: *new_letter,
                    }],
                    node,
                    swaps - 1,
                ));
            }
        }
    }
    if thread_count <= 1 {
        for mut call in calls {
            solver(&board, &mut call.0, call.1, call.2, &mut words);
        }
        return (words.inner, board);
    } else {
        // Nope, won't be doing Arc (tested it, performance with Arc sucks).
        // Unsafe code is completely safe, because all threads using board_ref are join()ed before taking back the board.
        let board_ptr = Box::into_raw(Box::new(board));
        let mut threads = vec![];
        let chunk_size = (calls.len() + thread_count as usize - 1) / thread_count as usize;
        {
            let board_ref: &'static Board = unsafe { &*board_ptr };
            while !calls.is_empty() {
                let chunk = calls
                    .drain(..chunk_size.min(calls.len()))
                    .collect::<Vec<_>>();
                threads.push(std::thread::spawn(move || {
                    let mut thread_words = SortedWordVec::new();
                    for mut call in chunk {
                        solver(&board_ref, &mut call.0, call.1, call.2, &mut thread_words);
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
