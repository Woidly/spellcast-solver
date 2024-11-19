use getch_rs::{Getch, Key};

use crate::{
    commandline::InteractiveSubCommand,
    quit,
    solver::{Board, Move, Tile, Word},
    utils::*,
};

/// Just a wrapper for Getch that automatically handles errors and Ctrl+C/Ctrl+Z.
struct GetchWrapper {
    getch: Getch,
}

impl GetchWrapper {
    fn new() -> Self {
        GetchWrapper {
            getch: Getch::new(),
        }
    }

    fn getch(&self) -> Key {
        match self.getch.getch() {
            Err(e) => quit!("Error in getch(): {e}"),
            Ok(Key::Ctrl('c' | 'z' | 'C' | 'Z')) => quit!("Exit requested by user"),
            Ok(x) => return x,
        }
    }
}

static COLUMNS: [(&str, i8); 5] = [("A", 0), ("B", 1), ("C", 2), ("D", 3), ("E", 4)];

#[derive(Debug)]
enum TileAction {
    DL,
    TL,
    TwoX,
    ThreeX,
}

#[derive(Debug)]
enum NumberAction {
    GemCount,
    GemBonus,
}

#[derive(Debug)]
enum State {
    /// Level editor, which is also used as initial state so user can type board.
    LetterEditor,
    GemEditor,
    /// Normal state. Board isn't yet solved, user can edit tiles and game meta.
    Normal,
    /// Editor sub-states. They do their thing according to inner value and return to Normal.
    PickNumber(NumberAction),
    PickTile(TileAction),
    /// Solved state. User can pick one of top words and switch to Move state.
    Solved,
    /// Move state. User sees move sequence on board and can either accept or cancel it.
    /// Inner value represents move index of top_moves array.
    /// Accepting removes tiles used in move, changes game meta (e.g. remove/add gems) and returns to Normal state.
    /// Cancelling returns to Solved state.
    Move(usize),
}

/// Returns 8-dot braille character that represents tile multiplier.
/// Two dots on top represent word multiplier, two dots on bottom represent letter multiplier.
/// No dots mean there is no multiplier, one dot means x2/DL and two dots mean x3/TL.
fn get_multiplier_indicator(word_multiplier: u8, letter_multiplier: u8) -> char {
    // https://en.wikipedia.org/wiki/Braille_Patterns
    // Unwrap is safe because 0x2800 + {0, 0x1, 0x9} + {0, 0x40, 0xC0} is valid Unicode
    char::from_u32(
        0x2800
            + match word_multiplier {
                2 => 0x1,
                3 => 0x1 | 0x8,
                _ => 0,
            }
            + match letter_multiplier {
                2 => 0x40,
                3 => 0x40 | 0x80,
                _ => 0,
            },
    )
    .unwrap()
}

struct InteractiveSolver {
    // TODO: Maybe clean up those a bit, move some of state-specific values into state inner.
    board: Board,
    editor_index: i8,
    elapsed: String,
    gems: u8,
    num_threads: u8,
    state: State,
    tile_picker: (i8, i8),
    top_moves: Vec<Word>,
}

impl InteractiveSolver {
    /// Returns a Board filled with ? tiles.
    fn empty_board() -> Board {
        Board {
            tiles: std::array::from_fn(|_| Tile::empty('?')),
            gem_bonus: 0,
        }
    }

    fn new(num_threads: u8) -> Self {
        InteractiveSolver {
            board: InteractiveSolver::empty_board(),
            editor_index: 0,
            elapsed: String::new(),
            gems: 3,
            num_threads,
            state: State::LetterEditor,
            tile_picker: (-1, -1),
            top_moves: vec![],
        }
    }

    /// Prints a visual representation of a board state.
    /// It is called from "high-level" print functions, so it includes CLEAR_HOME escape sequence.
    /// It allows to highlight a column and/or row.
    /// Because it's a "low-level" function, it doesn't decide how to render tiles on it's own, so tile_renderer function must be provided.
    /// It takes i8 tile index and must return String representation of tile.
    /// In most cases it's a closure that has a reference to board, so it can get tile from index on it's own.
    /// It also accepts meta, a Vec<String> with up to 5 elements that will be placed alongside board.
    /// In theory it could accept 6 elements, but as of now the first available row will be hardcoded to display self.state.
    fn print_board(
        &self,
        highlight_column: i8,
        hightlight_row: i8,
        tile_renderer: impl Fn(i8) -> String,
        meta: Vec<String>,
    ) {
        let mut buf = format!("{CLEAR_HOME}# ");
        for (letter, number) in COLUMNS {
            if number == highlight_column {
                buf += &format!("{RED}{letter} {RESET}");
            } else {
                buf = buf + letter + " ";
            }
        }
        buf += &format!(" State::{:?}\n", self.state);
        for row in 0..5 {
            if row == hightlight_row {
                buf += &format!("{RED}{} {RESET}", row + 1);
            } else {
                buf += &format!("{} ", row + 1);
            }
            for index in (row * 5)..(row * 5 + 5) {
                buf += &tile_renderer(index);
            }
            buf += &format!(
                " {}\n",
                if let Some(x) = meta.get(row as usize) {
                    x
                } else {
                    ""
                }
            );
        }
        print!("{}", buf);
    }

    /// Default tile_renderer implementation.
    /// Tiles with gems are coloured green.
    /// Letter/word multipliers are shown next to letter via [get_multiplier_indicator].
    fn default_tile_renderer(&self, index: i8) -> String {
        let tile = &self.board.tiles[index as usize];
        let letter = tile.letter;
        let multipliers = get_multiplier_indicator(tile.word_multiplier, tile.letter_multiplier);
        if tile.gem {
            format!("{GREEN}{letter}{multipliers}{RESET}")
        } else {
            format!("{letter}{multipliers}")
        }
    }

    /// Prints a visual representation of letter editor board.
    /// Has column/row highlighting (based on self.editor_index).
    /// Has custom tile renderer (selected - red, ? - grey, everything else - normal).
    /// Has custom meta (pos: index (column/row)).
    fn print_letter_editor_board(&self) {
        self.print_board(
            self.editor_index % 5,
            self.editor_index / 5,
            |index| {
                let letter = self.board.tiles[index as usize].letter;
                if index == self.editor_index {
                    format!("{RED}{letter} {RESET}")
                } else if letter == '?' {
                    format!("{GREY}{letter} {RESET}")
                } else {
                    format!("{letter} ")
                }
            },
            vec![format!(
                "pos: {} ({}{})",
                self.editor_index,
                COLUMNS[(self.editor_index % 5) as usize].0,
                self.editor_index / 5 + 1
            )],
        );
        let mut done = true;
        for tile in &self.board.tiles {
            if tile.letter == '?' {
                done = false;
                break;
            }
        }
        println!(
            "\n[{RED}Arrow keys{RESET}] Move cursor | [{RED}A{RESET}-{RED}Z{RESET}] Change letter
{}[{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit",
            if done {
                format!("[{RED}Enter{RESET}] Done\n")
            } else {
                String::new()
            }
        );
    }

    /// Handles key press for GemEditor state.
    /// Arrow keys - move self.editor_index.
    /// Enter - switch to Normal.
    /// A-Z - change tile.letter for current tile, optionally move self.editor_index to available ? tile.
    fn handle_letter_editor_state(&mut self, key: Key) {
        match key {
            // Arrow-key-based navigation to make it good enough to be general-purpose editor, not just one for entering initial board state.
            Key::Left => {
                if self.editor_index > 0 {
                    self.editor_index -= 1;
                    self.print_letter_editor_board();
                }
            }
            Key::Right => {
                if self.editor_index < 24 {
                    self.editor_index += 1;
                    self.print_letter_editor_board();
                }
            }
            Key::Up => {
                if self.editor_index >= 5 {
                    self.editor_index -= 5;
                    self.print_letter_editor_board();
                }
            }
            Key::Down => {
                if self.editor_index < 20 {
                    self.editor_index += 5;
                    self.print_letter_editor_board();
                }
            }
            Key::Char('\r' | '\n') => {
                // If no tiles are ?, change state to Normal and print_normal_board.
                for tile in &self.board.tiles {
                    if tile.letter == '?' {
                        return;
                    }
                }
                self.state = State::Normal;
                self.print_normal_board();
            }
            Key::Char(x) => match x {
                // Change letter on current tile, move cursor to next ? tile if it exists.
                'a'..='z' | 'A'..='Z' => {
                    let tile = &mut self.board.tiles[self.editor_index as usize];
                    tile.letter = x.to_ascii_lowercase();
                    let mut moved = false;
                    if self.editor_index < 24 {
                        for index in self.editor_index..25 {
                            if self.board.tiles[index as usize].letter == '?' {
                                self.editor_index = index;
                                moved = true;
                                break;
                            }
                        }
                    }
                    if !moved && self.editor_index > 0 {
                        for index in 0..self.editor_index {
                            if self.board.tiles[index as usize].letter == '?' {
                                self.editor_index = index;
                                break;
                            }
                        }
                    }
                    self.print_letter_editor_board();
                }
                _ => (),
            },
            _ => (),
        }
    }

    /// Prints a visual representation of gem editor board.
    /// Has column/row highlighting (based on self.editor_index).
    /// Has custom tile renderer (selected - red, unselected - normal, gem - adds ! indicator).
    /// Has custom meta (pos: index (column/row), gem: true/false).
    fn print_gem_editor_board(&self) {
        self.print_board(
            self.editor_index % 5,
            self.editor_index / 5,
            |index| {
                let tile = &self.board.tiles[index as usize];
                let letter = tile.letter;
                let gem_indicator = if tile.gem {
                    format!("{GREEN}!{RESET}")
                } else {
                    String::from(" ")
                };
                if index == self.editor_index {
                    format!("{RED}{letter}{gem_indicator}{RESET}")
                } else {
                    format!("{letter}{gem_indicator}")
                }
            },
            vec![
                format!(
                    "pos: {} ({}{})",
                    self.editor_index,
                    COLUMNS[(self.editor_index % 5) as usize].0,
                    self.editor_index / 5 + 1
                ),
                format!("gem: {}", self.board.tiles[self.editor_index as usize].gem),
            ],
        );
        println!(
            "\n[{RED}Arrow keys{RESET}] Move cursor | [{RED}G{RESET}/{RED}!{RESET}] Toggle gem
[{RED}Enter{RESET}] Done
[{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit"
        );
    }

    /// Handles key press for GemEditor state.
    /// Arrow keys - move self.editor_index.
    /// Enter - switch to Normal.
    /// G/! - toggle tile.gem for current tile.
    fn handle_gem_editor_state(&mut self, key: Key) {
        match key {
            Key::Left => {
                if self.editor_index > 0 {
                    self.editor_index -= 1;
                    self.print_gem_editor_board();
                }
            }
            Key::Right => {
                if self.editor_index < 24 {
                    self.editor_index += 1;
                    self.print_gem_editor_board();
                }
            }
            Key::Up => {
                if self.editor_index >= 5 {
                    self.editor_index -= 5;
                    self.print_gem_editor_board();
                }
            }
            Key::Down => {
                if self.editor_index < 20 {
                    self.editor_index += 5;
                    self.print_gem_editor_board();
                }
            }
            Key::Char('\r' | '\n') => {
                self.state = State::Normal;
                self.print_normal_board();
            }
            Key::Char('g' | 'G' | '!' | '1') => {
                let tile = &mut self.board.tiles[self.editor_index as usize];
                tile.gem = !tile.gem;
                self.print_gem_editor_board();
            }
            _ => (),
        }
    }

    /// Prints a visual representation of normal state board.
    /// Has no column/row highlighting.
    /// Uses default tile renderer.
    /// Has custom meta (X gems (X/3 swaps), (+Y score per gem)).
    fn print_normal_board(&self) {
        self.print_board(
            -1,
            -1,
            |index| self.default_tile_renderer(index),
            vec![
                format!("{} gems ({GREEN}{} swaps{RESET})", self.gems, self.gems / 3),
                format!("(+{} score per gem)", self.board.gem_bonus),
            ],
        );
        println!(
            "\nEdit board: [{RED}L{RESET}]etters | [{RED}G{RESET}]ems | [{RED}N{RESET}]ew game
Letter multiplier: [{RED}0{RESET}] Remove | [{RED}+{RESET}] DL | [{RED}*{RESET}] TL
Word multiplier: [{RED}1{RESET}] Remove | [{RED}${RESET}/{RED}2{RESET}] 2x | [{RED}^{RESET}/{RED}3{RESET}] 3x
Edit meta: Gem [{RED}C{RESET}]ount | Gem score [{RED}B{RESET}]onus
[{RED}S{RESET}]olve | [{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit"
        );
    }

    /// Handles key press for Normal state.
    /// L - switch to LetterEditor.
    /// G - switch to GemEditor.
    /// N - reset every inner state (except for gem_bonus), make new ? board, switch to LetterEditor.
    /// 0 - reset letter multiplier of every tile to 1.
    /// + - switch to TilePicker(DL).
    /// * - switch to TilePicker(TL).
    /// 1 - reset word multiplier of every tile to 1.
    /// $/2 - switch to TilePicker(TwoX).
    /// ^/3 - switch to TilePicker(ThreeX).
    /// C - switch to PickNumber(GemCount).
    /// B - switch to PickNumber(GemBonus).
    /// S - solves the board, switches to Solved.
    fn handle_normal_state(&mut self, key: Key) {
        match key {
            Key::Char('l' | 'L') => {
                self.state = State::LetterEditor;
                self.print_letter_editor_board();
            }
            Key::Char('g' | 'G') => {
                self.state = State::GemEditor;
                self.print_gem_editor_board();
            }
            Key::Char('n' | 'N') => {
                let gem_bonus = self.board.gem_bonus;
                self.board = InteractiveSolver::empty_board();
                self.board.gem_bonus = gem_bonus;
                self.editor_index = 0;
                self.gems = 3;
                self.state = State::LetterEditor;
                self.print_letter_editor_board();
            }
            Key::Char('0' | ')') => {
                let mut found = false;
                for tile in &mut self.board.tiles {
                    if tile.letter_multiplier != 1 {
                        tile.letter_multiplier = 1;
                        found = true;
                        break;
                    }
                }
                if found {
                    self.print_normal_board();
                }
            }
            Key::Char('+' | '=') => {
                self.tile_picker = (-1, -1);
                self.state = State::PickTile(TileAction::DL);
                self.print_tile_picker();
            }
            Key::Char('*' | '8') => {
                self.tile_picker = (-1, -1);
                self.state = State::PickTile(TileAction::TL);
                self.print_tile_picker();
            }
            Key::Char('1' | '!') => {
                let mut found = false;
                for tile in &mut self.board.tiles {
                    if tile.word_multiplier != 1 {
                        tile.word_multiplier = 1;
                        found = true;
                        break;
                    }
                }
                if found {
                    self.print_normal_board();
                }
            }
            Key::Char('$' | '4' | '2') => {
                self.tile_picker = (-1, -1);
                self.state = State::PickTile(TileAction::TwoX);
                self.print_tile_picker();
            }
            Key::Char('^' | '6' | '3') => {
                self.tile_picker = (-1, -1);
                self.state = State::PickTile(TileAction::ThreeX);
                self.print_tile_picker();
            }
            Key::Char('c' | 'C') => {
                self.state = State::PickNumber(NumberAction::GemCount);
                self.print_number_picker();
            }
            Key::Char('b' | 'B') => {
                self.state = State::PickNumber(NumberAction::GemBonus);
                self.print_number_picker();
            }
            Key::Char('s' | 'S') => {
                println!(
                    "{CLEAR_HOME}{RED}Solving the board, please stand by...{RESET}
{}
(Usually it doesn't take more than 10 seconds)
",
                    [
                        "I promise, I'm smarter than I look... probably.",
                        "Did someone say bruteforce?",
                        "Finding words you didn't know existed...",
                        "Mining cr- I mean, solving the board...",
                        "Even Chrome was scared of that RAM usage.",
                        "Please don't crash, please don't crash!"
                    ][random::Source::read_u64(&mut get_random()) as usize % 6]
                );
                self.top_moves.clear();
                let clock = std::time::Instant::now();
                // Making one little temporary value is worth it, threads have so much more performance benefit.
                let board = std::mem::take(&mut self.board);
                let (mut words, board) = board.solve(self.gems / 3, self.num_threads);
                self.board = board;
                self.elapsed = format!("{:.2}ms", clock.elapsed().as_secs_f64() * 1000.);
                words.sort_by_key(|x| -(x.score as i32));
                let mut existing_words = vec![];
                let mut counter = 0;
                for word in words {
                    if counter >= 10 {
                        break;
                    }
                    if existing_words.contains(&word.word) {
                        continue;
                    }
                    counter += 1;
                    existing_words.push((&word.word).clone());
                    self.top_moves.push(word);
                }
                self.state = State::Solved;
                self.print_solved_state();
            }
            Key::Char('e' | 'E') => {
                // [E]aster egg
                println!("oe");
            }
            _ => (),
        }
    }

    /// Prints a visual representation of number picker.
    /// Has no column/row highlighting.
    /// Uses default tile renderer.
    /// Has no custom meta.
    /// 
    /// Why would it? It literally asks for a single number.
    fn print_number_picker(&self) {
        self.print_board(-1, -1, |index| self.default_tile_renderer(index), vec![]);
        println!(
            "\n[{RED}0{RESET}-{RED}9{RESET}] Choose 0-9 | [{RED}-{RESET}] Choose 10
[{RED}Esc{RESET}/{RED}Z{RESET}] Back | [{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit"
        );
    }

    /// Handles key press for NumberPicker state.
    /// Z - switch to Normal.
    /// 0-9 - change some parameter to 0-9 according to inner.
    /// - - change some parameter to 10 according to inner.
    fn handle_number_picker(&mut self, key: Key) {
        let action = if let State::PickNumber(x) = &self.state {
            x
        } else {
            return;
        };
        let number = match key {
            Key::Esc | Key::Char('z' | 'Z') => {
                self.state = State::Normal;
                self.print_normal_board();
                return;
            }
            Key::Char(x) => match x {
                '0'..='9' => x as u8 - '0' as u8,
                '-' | '_' => 10,
                _ => return,
            },
            _ => return,
        };
        match action {
            NumberAction::GemBonus => {
                self.board.gem_bonus = number as u16;
            }
            NumberAction::GemCount => {
                self.gems = number;
            }
        }
        self.state = State::Normal;
        self.print_normal_board();
    }

    /// Prints a visual representation of tile picker.
    /// Has column/row highlighting (based on self.tile_picker).
    /// Has custom tile renderer (red if every non -1 parameter matches, normal otherwise).
    /// Has no custom meta.
    fn print_tile_picker(&self) {
        self.print_board(
            self.tile_picker.0,
            self.tile_picker.1,
            |index| {
                let column = index % 5;
                let row = index / 5;
                if (self.tile_picker.0 != -1 || self.tile_picker.1 != -1)
                    && (self.tile_picker.0 == -1 || self.tile_picker.0 == column)
                    && (self.tile_picker.1 == -1 || self.tile_picker.1 == row)
                {
                    format!("{RED}{} {RESET}", self.board.tiles[index as usize].letter)
                } else {
                    format!("{} ", self.board.tiles[index as usize].letter)
                }
            },
            vec![],
        );
        println!("\n[{RED}A{RESET}-{RED}E{RESET}] Choose column | [{RED}1{RESET}-{RED}5{RESET}] Choose row
{}[{RED}Esc{RESET}/{RED}Z{RESET}] Back | [{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit",
if self.tile_picker.0 != -1 && self.tile_picker.1 != -1 {
            format!("[{RED}Enter{RESET}] Done")
        } else {String::new()});
    }

    /// Handles key press for TilePicker state.
    /// Enter - if both column and row are present, change selected tile's multipliers according to inner.
    /// Z - switch to Normal.
    /// A-E - set column (self.tile_picker.0)
    /// 1-5 - set row (self.tile_picker.1)
    fn handle_tile_picker(&mut self, key: Key) {
        let action = if let State::PickTile(x) = &self.state {
            x
        } else {
            return;
        };
        match key {
            Key::Char('\r' | '\n') => {
                if self.tile_picker.0 != -1 && self.tile_picker.1 != -1 {
                    let index = (self.tile_picker.1 * 5 + self.tile_picker.0) as usize;
                    match action {
                        TileAction::DL => {
                            for tile in &mut self.board.tiles {
                                tile.letter_multiplier = 1;
                            }
                            self.board.tiles[index].letter_multiplier = 2;
                        }
                        TileAction::TL => {
                            for tile in &mut self.board.tiles {
                                tile.letter_multiplier = 1;
                            }
                            self.board.tiles[index].letter_multiplier = 3;
                        }
                        TileAction::TwoX => {
                            for tile in &mut self.board.tiles {
                                tile.word_multiplier = 1;
                            }
                            self.board.tiles[index].word_multiplier = 2;
                        }
                        TileAction::ThreeX => {
                            for tile in &mut self.board.tiles {
                                tile.word_multiplier = 1;
                            }
                            self.board.tiles[index].word_multiplier = 3;
                        }
                    }
                    self.state = State::Normal;
                    self.print_normal_board();
                }
                return;
            }
            Key::Esc | Key::Char('z' | 'Z') => {
                self.state = State::Normal;
                self.print_normal_board();
                return;
            }
            Key::Char(x) => match x {
                'A'..='E' | 'a'..='e' => {
                    self.tile_picker.0 = x.to_ascii_lowercase() as i8 - 'a' as i8;
                }
                '1'..='5' => {
                    self.tile_picker.1 = x as i8 - '1' as i8;
                }
                _ => return,
            },
            _ => return,
        }
        self.print_tile_picker();
    }

    /// Prints a visual representation of solved state board.
    /// Has no column/row highlighting.
    /// Uses default tile renderer.
    /// Has custom meta (X elapsed).
    fn print_solved_state(&self) {
        self.print_board(
            -1,
            -1,
            |index| self.default_tile_renderer(index),
            vec![format!("{} elapsed", self.elapsed)],
        );
        let mut buffer = String::from("\n");
        for index in 0..self.top_moves.len() {
            let word = &self.top_moves[index];
            let mut letters = vec![];
            for m in &word.moves {
                match m {
                    Move::Normal { index } => {
                        let tile = &self.board.tiles[*index as usize];
                        letters.push(tile.letter.to_string());
                    }
                    Move::Swap { new_letter, .. } => {
                        letters.push(format!("{RED}{}{RESET}", new_letter));
                    }
                }
            }
            buffer += &format!(
                "[{RED}{}{RESET}] {} (+{})",
                (index + 1) % 10,
                letters.join(""),
                word.score
            );
            if (index + 1) % 2 == 0 {
                buffer += "\n";
            } else {
                buffer += " | ";
            }
        }
        println!(
            "{}[{RED}U{RESET}]nsolve | [{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit",
            buffer
        );
    }

    /// Handles key press for Solved state.
    /// U - switch to Normal.
    /// 0-9 - switch to Move(X), where X is pressed key - 1.
    fn handle_solved_state(&mut self, key: Key) {
        match key {
            Key::Char('u' | 'U') => {
                self.state = State::Normal;
                self.print_normal_board();
            }
            Key::Char(x) => match x {
                '0'..='9' => {
                    let mut index = x as usize - '0' as usize;
                    if index == 0 {
                        index = 9;
                    } else {
                        index -= 1;
                    }
                    self.state = State::Move(index);
                    self.print_move_state(index);
                }
                _ => (),
            },
            _ => (),
        }
    }

    /// Prints a visual representation of move state board.
    /// Has no column/row highlighting.
    /// Has custom tile renderer (red if swap move, normal if normal move, replaced with invisible * if no move, hexadecimal move order indicator).
    /// Has custom meta (+X points, Y swaps used, Z gems collected, W gems after move).
    fn print_move_state(&mut self, index: usize) {
        let word = &self.top_moves[index];
        let mut tiles: [Option<(u8, &Move)>; 25] = std::array::from_fn(|_| None);
        let mut counter: u8 = 0;
        for m in &word.moves {
            tiles[m.index() as usize] = Some((counter, m));
            counter += 1;
        }
        self.print_board(
            -1,
            -1,
            |tile_index| {
                if let Some((step, m)) = tiles[tile_index as usize] {
                    let converted_step;
                    if step < 10 {
                        converted_step = step.to_string();
                    } else {
                        converted_step = (('a' as u8 + step - 10) as char).to_string();
                    }
                    let letter = m.letter(&self.board);
                    if let Move::Swap { .. } = m {
                        format!("{RED}{letter}{GREY}{converted_step}{RESET}")
                    } else {
                        format!("{letter}{GREY}{converted_step}{RESET}")
                    }
                } else {
                    format!("{BLACK}* {RESET}")
                }
            },
            vec![
                word.formatted(&self.board),
                format!("+{} points", word.score),
                format!("{} swaps used", word.swap_count),
                format!("{} gems collected", word.gems),
                format!(
                    "{} gems after move",
                    (self.gems - (word.swap_count * 3) + word.gems).min(10)
                ),
            ],
        );
        println!(
            "\n^ Now make the move following this instruction
Number on the right of tile is step number
Two-digit numbers use hex-like letters (e.g. 14=e)
[{RED}A{RESET}]ccept
[{RED}Esc{RESET}/{RED}Z{RESET}] Back | [{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit"
        );
    }

    /// Handles key press for Move state.
    /// A - update gem count, replace tiles involved in move with ?, switch to LetterEditor.
    /// Esc/Z - switch to Solved.
    fn handle_move_state(&mut self, key: Key, index: usize) {
        match key {
            Key::Char('a' | 'A') => {
                let word = &self.top_moves[index];
                self.gems = (self.gems - (word.swap_count * 3) + word.gems).min(10);
                let mut first_tile = i8::MAX;
                for m in &word.moves {
                    let index = m.index();
                    first_tile = first_tile.min(index);
                    self.board.tiles[index as usize] = Tile::empty('?');
                }
                self.state = State::LetterEditor;
                self.editor_index = first_tile;
                self.print_letter_editor_board();
            }
            Key::Esc | Key::Char('z' | 'Z') => {
                self.state = State::Solved;
                self.print_solved_state();
                return;
            }
            _ => (),
        }
    }

    /// Runs the infinite loop that reads getch and calls respective state handler.
    fn run(mut self, getch: GetchWrapper) {
        self.print_letter_editor_board();
        loop {
            let key = getch.getch();
            match self.state {
                State::LetterEditor => self.handle_letter_editor_state(key),
                State::GemEditor => self.handle_gem_editor_state(key),
                State::Normal => self.handle_normal_state(key),
                // Can't pass inner value directly into handlers because of borrow checker.
                State::PickNumber(_) => self.handle_number_picker(key),
                State::PickTile(_) => self.handle_tile_picker(key),
                State::Solved => self.handle_solved_state(key),
                // And here we can, because index is primitive.
                State::Move(index) => self.handle_move_state(key, index),
            }
        }
    }
}

pub fn entry(_: InteractiveSubCommand, num_threads: u8) {
    let getch = GetchWrapper::new();
    println!(
        "{CLEAR_HOME}\
The {RED}superior{RESET} Discord Spellcast solver - now in Rust!
(c) 2024 {MAGENTA}Woidly{RESET} (MIT license)
{RED}https://github.com/Woidly/spellcast-solver{RESET}

Welcome to interactive mode!
You can learn more about it from {RED}INTERACTIVE.md{RESET}.
Terminal size of at least 14x52 is required.
Basic 16-colour support is recommended.

[{RED}S{RESET}]tart | [{RED}Ctrl+C{RESET}/{RED}Ctrl+Z{RESET}] Exit"
    );
    loop {
        match getch.getch() {
            Key::Char('s' | 'S') => break,
            _ => (),
        }
    }
    InteractiveSolver::new(num_threads).run(getch);
}
