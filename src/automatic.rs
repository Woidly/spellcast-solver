use std::{
    fs::File,
    io::{Read as _, Write},
};

use image::{ImageBuffer, Rgb};
use once_cell::sync::Lazy;

use crate::{
    commandline::AutomaticSubCommand,
    quit,
    solver::{Board, Move, Tile},
};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn is_gem_pink(pixel: &[u8; 3]) -> bool {
    pixel[0] > 0xaa && pixel[1] < 0xaa && pixel[2] > 0xaa
}

fn gem_count(img: &Image, x: u32, y: u32) -> u8 {
    let mut gems = 0;
    for row in 0..=1 {
        for column in 0..5 {
            // Scientifically accurate values I found via GIMP.
            if is_gem_pink(&img.get_pixel(x + 553 + column * 19, y + 558 + row * 33).0) {
                gems += 1;
            }
        }
    }
    gems
}

const LETTER_LOOKUP_TABLE: Lazy<[[u64; 6]; 26]> = Lazy::new(|| {
    let buf = include_bytes!("letters.bin");
    assert_eq!(buf.len(), 26 * 6 * 8, "letters.bin is corrupt");
    let mut lookup_table = [[0u64; 6]; 26];
    for (i, letter) in lookup_table.iter_mut().enumerate() {
        for (j, chunk) in letter.iter_mut().enumerate() {
            let start = (i * 6 + j) * 8;
            let end = start + 8;
            let bytes: [u8; 8] = buf[start..end].try_into().unwrap();
            *chunk = u64::from_le_bytes(bytes);
        }
    }
    lookup_table
});

/// High-tech letter recognition algorithm (just kidding, it sucks).
/// It compares tile pixels to previously captured values stored in letters.bin.
/// Letter with most matching pixels is returned.
// TODO: Change it to something more stable.
// Font rendering in canvas is very unstable (it is literally one of common browser fingerprinting techniques).
// And unlike other detections, which check colour of single pixel (and I put most of offsets in a place, where neighbour pixels have the same colour), this one checks for multiple pixels at once limited to 1 bit of colour.
// Though, including multiple full-coloured samples may make program way too big.
// Not sure about it.
// Maybe I should just use proper OCR?
fn detect_letter(img: &Image, tx: u32, ty: u32) -> char {
    let mut letter_id = vec![];
    let mut current: u64 = 0;
    let mut current_bit: u8 = 0;
    for counter in 0..384 {
        let x = tx + 28 + counter % 16;
        let y = ty + 27 + counter / 16;
        let status = img.get_pixel(x, y).0[0] < 0x55;
        if status {
            current |= 1 << current_bit;
        }
        if current_bit == 63 {
            current_bit = 0;
            letter_id.push(current);
            current = 0;
        } else {
            current_bit += 1;
        }
    }
    let mut best_match: u8 = 0;
    let mut best_score = 0;
    for (i, lookup_letter) in LETTER_LOOKUP_TABLE.iter().enumerate() {
        let mut score = 0;
        for (chunk, lookup_chunk) in letter_id.iter().zip(lookup_letter) {
            score += (chunk & lookup_chunk).count_ones();
        }
        if score > best_score {
            best_score = score;
            best_match = i as u8;
        }
    }
    (b'a' + best_match) as char
}

fn parse_tile(img: &Image, tx: u32, ty: u32) -> Tile {
    Tile {
        letter: detect_letter(img, tx, ty),
        letter_multiplier: match img.get_pixel(tx + 19, ty + 24).0 {
            [0xfc, 0xf5, 0xa3] => 2,
            [0xd6, 0x70, 0x40] => 3,
            _ => 1,
        },
        // TODO: detect 3x (couldn't find it while testing).
        word_multiplier: if is_gem_pink(&img.get_pixel(tx + 48, ty + 6).0) {
            2
        } else {
            1
        },
        gem: is_gem_pink(&img.get_pixel(tx + 18, ty + 50).0),
        // TODO: detect frozen tiles.
        frozen: false,
    }
}

fn parse_board(img: &Image, x: u32, y: u32) -> Board {
    let mut tiles = vec![];
    for row in 0..5 {
        for column in 0..5 {
            // Yeah, "tile" size is really 70x69. Why not 70x70? No clue.
            // Tile is in quotes because that is just a region I define as a tile.
            // Technically it includes space between tile sprites (which is even horizontally, but not vertically).
            let tx = x + 464 + column * 70;
            let ty = y + 167 + row * 69;
            tiles.push(parse_tile(img, tx, ty));
        }
    }
    Board {
        tiles: tiles.try_into().unwrap(),
        gem_bonus: 0,
    }
}

fn get_swap_menu_coord(letter: char, x: u32, y: u32) -> (u32, u32) {
    let o = letter as u8 - 'a' as u8;
    let ox = o % 6;
    let oy = o / 6;
    (x + 480 + ox as u32 * 65, y + 270 + oy as u32 * 65)
}

fn get_tile_coord(index: i8, x: u32, y: u32) -> (u32, u32) {
    let column = index % 5;
    let row = index / 5;
    (
        x + 464 + column as u32 * 70 + 25,
        y + 167 + row as u32 * 69 + 25,
    )
}

pub fn entry(args: AutomaticSubCommand, num_threads: u8) {
    let mut image_buf = Vec::new();
    if args.input == "-" {
        let _ = std::io::stdin()
            .read_to_end(&mut image_buf)
            .map_err(|e| quit!("Error when reading stdin: {e:?}"));
    } else {
        let _ = File::open(args.input)
            .map_err(|e| quit!("Error when opening file: {e:?}"))
            .unwrap()
            .read_to_end(&mut image_buf)
            .map_err(|e| quit!("Error when reading file: {e:?}"));
    }
    let dyn_img = image::load_from_memory(&image_buf)
        .map_err(|e| quit!("Error when reading image: {e:?}"))
        .unwrap();
    let _converted_img: Image;
    let img: &Image;
    if let Some(x) = dyn_img.as_rgb8() {
        img = x;
    } else {
        _converted_img = dyn_img.to_rgb8();
        img = &_converted_img;
    }
    let mut _stdout;
    let mut _file;
    let output: &mut dyn Write;
    if args.output == "-" {
        _stdout = std::io::stdout().lock();
        output = &mut _stdout;
    } else {
        _file = File::create(args.output)
            .map_err(|e| quit!("Error when opening file: {e:?}"))
            .unwrap();
        output = &mut _file;
    }
    writeln!(
        output,
        "PRINT This is WIP! Expect letter detection to return incorrect results."
    )
    .unwrap();
    let board = parse_board(img, args.x, args.y);
    let clock = std::time::Instant::now();
    let (mut words, _) = board.solve(gem_count(img, args.x, args.y) / 3, num_threads);
    words.sort_by_key(|x| -(x.score as i32));
    if let Some(word) = words.first() {
        writeln!(
            output,
            "PRINT {:.2}ms elapsed",
            clock.elapsed().as_secs_f64() * 1000.
        )
        .unwrap();
        writeln!(output, "PRINT {} (+{})", word.word, word.score).unwrap();
        writeln!(output, "PRINT {} swaps used", word.swap_count).unwrap();
        writeln!(output, "PRINT {} gems collected", word.gems).unwrap();
        for m in &word.moves {
            if let Move::Swap { index, new_letter } = m {
                let swap_button_x = args.x + 740;
                let swap_button_y = args.y + 580;
                let (tile_x, tile_y) = get_tile_coord(*index, args.x, args.y);
                let (letter_x, letter_y) = get_swap_menu_coord(*new_letter, args.x, args.y);
                writeln!(
                    output,
                    "SWAP {swap_button_x} {swap_button_y} {tile_x} {tile_y} {letter_x} {letter_y}"
                )
                .unwrap();
            }
        }
        writeln!(
            output,
            "MOVE {}",
            (&word.moves)
                .into_iter()
                .map(|m| {
                    let (tile_x, tile_y) = get_tile_coord(m.index(), args.x, args.y);
                    format!("{tile_x} {tile_y}")
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    } else {
        quit!("No solution found!");
    }
}
