use std::{fs::File, io::Read as _};

use image::{ImageBuffer, Rgb};

use crate::{commandline::AutomaticSubCommand, quit};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn gem_count(img: &Image, x: u32, y: u32) -> u8 {
    let mut gems = 0;
    for row in 0..=1 {
        for column in 0..5 {
            // Scientifically accurate values I found via GIMP.
            let pixel = &img.get_pixel(x + 553 + column * 19, y + 558 + row * 33).0;
            if pixel[0] > 0xaa && pixel[1] < 0xaa && pixel[2] > 0xaa {
                gems += 1;
            }
        }
    }
    gems
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
    )
}
