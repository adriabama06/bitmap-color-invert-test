use std::{fs::{File}, env};

mod bitmap;
mod image;

fn main() {
    let args: Vec<String> = env::args().map(|arg| arg).collect::<Vec<String>>();

    if args.len() < 3 {
        println!("{} <input.bmp> <output.bmp>", args.get(0).unwrap());
        return;
    }

    let mut to_open: File = File::open(
        args.get(1).unwrap()
    ).unwrap();

    let mut bmp: bitmap::BITMAP = bitmap::BITMAP::default();

    bitmap::bitmap_load(&mut to_open, &mut bmp);

    drop(to_open);

    image::invert_colors(&mut bmp);

    let mut to_save: File = File::create(
        args.get(2).unwrap()
    ).unwrap();

    bitmap::bitmap_save(&mut to_save, &bmp);

    drop(to_save);
}
