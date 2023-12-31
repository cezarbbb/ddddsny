use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};
use std::fs::File;
use std::io::{self, Write};
use termion::{clear, cursor};
use std::env;

enum DuffyAndFriends {
    Olu,
    LinaBell,
    CookieAnn,
}

struct Config {
    character: DuffyAndFriends
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let character = config.character;
    let file_name = match character {
        DuffyAndFriends::Olu => "examples/Olu.gif",
        DuffyAndFriends::CookieAnn => "examples/CookieAnn.gif",
        DuffyAndFriends::LinaBell => "examples/LinaBell.gif",
        _ => panic!("Sorry, the character you choose is not avaliable now!"),
    };

    let file = File::open(file_name).unwrap();

    let decoder = GifDecoder::new(file).unwrap();
    let frames = decoder.into_frames();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for frame in frames {
        let frame = frame.unwrap().into_buffer();
        let frame = resize_image(&frame,32, 32);
        let frame_string = convert_frame_to_string(&frame);
        write!(handle, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        handle.flush().unwrap();
        print!("{}", frame_string);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn parse_config(args: &[String]) -> Config {
    let default = DuffyAndFriends::Olu;
    let mut character = default;
    if args.len() >= 2 {
        character = match &args[1][..] {
            "olu" => DuffyAndFriends::Olu,
            "linabell" => DuffyAndFriends::LinaBell,
            "cookieann" => DuffyAndFriends::CookieAnn,
            _ => panic!("Sorry, the character {} you choose is not avaliable now!", &args[1]),
        };
        println!("Selecting character: {}!", &args[1]);
    }
    else { 
        println!("No character selected yet. Selecting default character!");
    }
    Config { character }
}

fn resize_image(image: &image::RgbaImage, width: u32, height: u32) -> image::RgbImage {
    let dynamic_image = DynamicImage::ImageRgba8(image.clone());
    dynamic_image.resize(width, height, image::imageops::FilterType::Lanczos3).to_rgb8()
}


// fn convert_frame_to_colored_string(frame: &image::RgbImage) -> String {
//     let mut result = String::new();
//     for y in 0..frame.height() {
//         for x in 0..frame.width() {
//             let pixel = frame.get_pixel(x, y);
//             let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
//             let color_code = format!("\x1b[48;2;{};{};{}m  ", r, g, b);
//             result.push_str(&color_code);
//         }
//         result.push_str("\x1b[0m\n");
//     }
//     result
// }

fn convert_frame_to_string(frame: &image::RgbImage) -> String {
    let mut result = String::new();
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let pixel = frame.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let brightness = ((r as u32 + g as u32 + b as u32) / 3) as u8;
            let ascii_char = match brightness {
                0..=64 => "  ",
                65..=128 => "..",
                129..=192 => "**",
                _ => "##",
            };
            result.push_str(ascii_char);
        }
        result.push('\n');
    }
    result
}
