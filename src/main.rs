extern crate clap;
extern crate image;
use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::iter;
use std::fs::File;
use clap::{
    Arg,
    App
};
use image::{
    GenericImage,
    ImageBuffer
};

fn main() {
    let matches = App::new("Gradient output")
                    .version("0.1.0")
                    .arg(Arg::with_name("OUTPUT_FILE")
                            .help("Sets the output file to use")
                            .required(true)
                            .index(1))
                    .arg(Arg::with_name("width")
                            .short("x")
                            //.long("width")
                            .help("Sets the width of the image: default 200")
                            .takes_value(true))
                    .arg(Arg::with_name("height")
                            .short("y")
                            //.long("height")
                            .help("Sets the height of the image. default: 100")
                            .takes_value(true))
                    .get_matches();

    let out_filename = matches.value_of("OUTPUT_FILE").unwrap_or("./out.ppm");
    let width_in = matches.value_of("width").unwrap_or("200");
    let height_in = matches.value_of("height").unwrap_or("100");

    let width: u32 = match width_in.parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("invalid width");
        }
    };

    let height: u32 = match height_in.parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("invalid height");
        }
    };

    {
        let mut img = ImageBuffer::new(width, height);

        for y in (0..height).rev() {
            for x in 0..width {
                let r = x as f32 / width as f32;
                let g = y as f32 / height as f32;
                let b = 0.2;
                let ir = (255.99 * r) as u8;
                let ig = (255.99 * g) as u8;
                let ib = (255.99 * b) as u8;

                img.put_pixel(x, y, image::Rgb([ir, ig, ib]));
            }
        }

        let ref mut fout = File::create(out_filename).unwrap();
        let _ = image::ImageRgb8(img).save(fout, image::PNG);
        println!("wrote {}", out_filename);
    }
}
