use std::{fs::File, io::Read as _};

use image::{ImageBuffer, Rgb};

use crate::{
    commandline::AutomaticSubCommand,
    quit,
    solver::{Board, Tile},
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

fn parse_tile(img: &Image, tx: u32, ty: u32) -> Tile {
    Tile {
        // TODO: detect letter.
        letter: '?',
        letter_multiplier: match img.get_pixel(tx + 19, ty + 24).0 {
            [0xfc, 0xf5, 0xa3] => 2,
            [0xd6, 0x70, 0x40] => 3,
            _ => 1,
        },
        // TODO: detect 3x (can't find it while testing).
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
    println!("This is WIP! Only some aspects of parsing are done.");
    println!(
        "There are {} gems on the image",
        gem_count(img, args.x, args.y)
    );
    println!("Tiles:\n{:?}", parse_board(img, args.x, args.y).tiles);
}
