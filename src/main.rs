use std::{fs::{self, File}, env};

mod bitmap;

fn main() {
    let args: Vec<String> = env::args().map(|arg| arg).collect::<Vec<String>>();

    if args.len() < 3 {
        println!("{} <input.bmp> <output.bmp>", args.get(0).unwrap());
        return;
    }

    println!("Opening... {}", args.get(1).unwrap());

    let mut to_open = File::open(
        args.get(1).unwrap()
    ).unwrap();

    let mut bmp: bitmap::BITMAP = bitmap::BITMAP::default();

    bitmap::bitmap_load(&mut to_open, &mut bmp);
}
