use std::{
    fs::File,
    io::{Seek, SeekFrom, Read}
};

#[allow(non_camel_case_types)]

pub struct BITMAP_HEADER_FILE
{
    pub filesize: u32,
    pub reserved: u32,
    pub dataoffset: u32,

    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub imagesize: u32,
    pub y_pixels_per_m: u32,
    pub x_pixels_per_m: u32,
    pub colors_used: u32,
    pub important_colors: u32,
    
    // array = [type, len]
    pub signature: [u8; 2], // <-- this in theory need to go at the start, but for the padding this cause problems
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct BITMAP {
    pub header: BITMAP_HEADER_FILE,
    pub pixels: Vec<RGB>,
}

impl Default for BITMAP {
    fn default() -> BITMAP {
        BITMAP {
            header: BITMAP_HEADER_FILE {
                filesize: 0,
                reserved: 0,
                dataoffset: 0,
                size: 0,
                width: 0,
                height: 0,
                planes: 0,
                bits_per_pixel: 0,
                compression: 0,
                imagesize: 0,
                y_pixels_per_m: 0,
                x_pixels_per_m: 0,
                colors_used: 0,
                important_colors: 0,
                signature: [0, 0]
            },
            pixels: Vec::new()
        }
    }
}

fn readU16(b: &mut [u8; 2]) -> u16 {
    return u16::from(b[0]) | u16::from(b[1])<<8;
}

fn readU32(b: &mut [u8; 4]) -> u32 {
    return u32::from(b[0]) | u32::from(b[1])<<8 | u32::from(b[2])<<16 | u32::from(b[3])<<24;
}

pub fn bitmap_load(bmp_file: &mut File, bmp: &mut BITMAP) {
    println!("Loading...");

    //bmp_file.seek(SeekFrom::Start(0)).unwrap();

    let mut signature: [u8; 2] = [0, 0];
    bmp_file.read(&mut signature).unwrap();

    let mut filesize: [u8; 4] = [0,0,0,0];
    bmp_file.read(&mut filesize).unwrap();

    bmp.header.signature = signature;
    bmp.header.filesize = readU32(&mut filesize);

    println!("{} {}", *bmp.header.signature.get(0).unwrap() as char, *bmp.header.signature.get(1).unwrap() as char);
    println!("{}", bmp.header.filesize);

    return;
}