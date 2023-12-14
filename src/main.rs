use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};
use std::fs::File;
use std::io::{self, Write};
use termion::{clear, cursor};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let character = &args[1];
    assert_eq!(character, "olu");
    println!("Selecting character: {}!", character);

    let file = File::open("examples/Olu.gif").unwrap();

    let decoder = GifDecoder::new(file).unwrap();
    let frames = decoder.into_frames();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for frame in frames {
        let frame = frame.unwrap().into_buffer();
        let frame = resize_image(&frame, 32, 32);
        let frame_string = convert_frame_to_string(&frame);
        write!(handle, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        handle.flush().unwrap();
        print!("{}", frame_string);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn resize_image(image: &image::RgbaImage, width: u32, height: u32) -> image::RgbImage {
    let dynamic_image = DynamicImage::ImageRgba8(image.clone());
    dynamic_image.resize(width, height, image::imageops::FilterType::Lanczos3).to_rgb8()
}


fn convert_frame_to_string(frame: &image::RgbImage) -> String {
    let mut result = String::new();
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let pixel = frame.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let brightness = ((r as u32 + g as u32 + b as u32) / 3) as u8;
            let ascii_char = match brightness {
                0..=64 => " ",
                65..=128 => ".",
                129..=192 => "*",
                _ => "#",
            };
            result.push_str(ascii_char);
        }
        result.push('\n');
    }
    result
}
