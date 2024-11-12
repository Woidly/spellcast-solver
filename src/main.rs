use std::process::exit;

use commandline::CONFIG;
use dictionary::DICTIONARY;
use solver::{Board, Move};

mod commandline;
mod dictionary;
mod solver;

static RESET: &str = "\x1B[0m";
static RED: &str = "\x1B[31m";

// TODO: Move ClockLogger into benchmark file when implementing benchmarks or make a better timing thingy
struct ClockLogger {
    instant: std::time::Instant,
    previous: f64
}

impl ClockLogger {
    fn new() -> Self {
        ClockLogger {instant: std::time::Instant::now(), previous: 0.}
    }

    fn log(&mut self, text: impl ToString) {
        let ms = self.instant.elapsed().as_secs_f64() * 1000.;
        let previous = self.previous;
        self.previous = ms;
        println!("[{:.2}ms/{:.2}ms delta] {}", ms, ms - previous, text.to_string());
    }
}

fn quit(message: &str) {
    eprintln!("{}", message);
    exit(1);
}

fn main() {
    if CONFIG.benchmark {
        todo!("Benchmark isn't implemented yet") 
        // let mut clock = ClockLogger::new();
        // stuff();
        // clock.log("stuff done");
    } else {
        if CONFIG.swap_count > 3 {
            quit("Swap amount can't be higher than 3");
        }
        if let Some(board_str) = CONFIG.board.clone().or_else(|| std::fs::read_to_string("board.txt").ok()) {
            if let Some(board) = Board::from_str(&board_str) {
                // Just load the dictionary by doing something with it (to time actual search time properly).
                println!("Loaded dictionary - {} entries in lookup table", DICTIONARY.len());
                let clock = std::time::Instant::now();
                let mut words = board.solve(CONFIG.swap_count);
                println!("Found {} unique move sequences in {:.2}ms", words.len(), clock.elapsed().as_secs_f64() * 1000.);
                words.sort_by_key(|x| -(x.score as i32));
                let mut existing_words = vec![];
                let mut counter = 0;
                for word in words {
                    if counter >= CONFIG.move_count { break; }
                    if existing_words.contains(&word.word) { continue; }
                    counter += 1;
                    existing_words.push((&word.word).clone());
                    let mut letters = vec![];
                    let mut swaps = vec![];
                    for m in &word.moves {
                        match m {
                            Move::Normal { index } => letters.push(board.tiles[*index as usize].letter.to_string()),
                            Move::Swap { index, new_letter } => {
                                letters.push(format!("{RED}{new_letter}{RESET}"));
                                swaps.push(format!("{} -> {} @ {},{}", board.tiles[*index as usize].letter, new_letter, index % 5, index / 5));
                            }
                        }
                    }
                    if CONFIG.pretty_print {
                        let mut table: [String; 25] = std::array::from_fn(|_| ".".to_string());
                        for m in &word.moves {
                            if let Move::Swap { .. } = m {
                                table[m.index() as usize] = format!("{RED}{}{RESET}", m.letter(&board));
                            } else {
                                table[m.index() as usize] = m.letter(&board).to_string();
                            }
                        }
                        println!("=========[{}]=========", counter);
                        for y in 0..5 {
                            for x in 0..5 {
                                print!("{}", table[y * 5 + x]);
                            }
                            if y == 0 {
                                print!(" {}", letters.join(""));
                            }
                            if y == 1 {
                                print!(" {} points", word.score);
                            }
                            if y >= 2 {
                                let swap_index = y - 2;
                                if swap_index < swaps.len() {
                                    print!(" {}", swaps[swap_index as usize]);
                                }
                            }
                            println!();
                        }
                    } else {
                        println!("{}. [{}] {} / {}", counter, word.score, letters.join(""), swaps.join("; "))
                    }
                }
            } else {
                quit("Failed to parse board string!");
            }
        } else {
            quit("Failed to read board string from file!");
        }
    }
}
