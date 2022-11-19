use std::{
    fs::File,
    io::{Seek, SeekFrom, Read, Write}
};

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
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

fn read_u16(b: &mut [u8; 2]) -> u16 {
    return u16::from(b[0]) | u16::from(b[1])<<8;
}

fn read_u32(b: &mut [u8; 4]) -> u32 {
    return u32::from(b[0]) | u32::from(b[1])<<8 | u32::from(b[2])<<16 | u32::from(b[3])<<24;
}

fn transform_u16_to_u8(n: u16) -> [u8; 2] {
    let mut buff: [u8; 2] = [(n >> 8) as u8, n as u8];
    buff.reverse();
    return buff;
}

fn transform_u32_to_u8(n: u32) -> [u8; 4] {
    let mut buff: [u8; 4] = [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8];
    buff.reverse();
    return buff;
}


fn read_header(bmp_file: &mut File, bmp: &mut BITMAP) {
    bmp_file.seek(SeekFrom::Start(0)).unwrap();

    let mut signature: [u8; 2] = [0, 0];

    let mut filesize: [u8; 4] = [0,0,0,0];
    let mut reserved: [u8; 4] = [0,0,0,0];
    let mut dataoffset: [u8; 4] = [0,0,0,0];

    let mut size: [u8; 4] = [0,0,0,0];
    let mut width: [u8; 4] = [0,0,0,0];
    let mut height: [u8; 4] = [0,0,0,0];
    let mut planes: [u8; 2]= [0,0];
    let mut bits_per_pixel: [u8; 2]= [0,0];
    let mut compression: [u8; 4] = [0,0,0,0];
    let mut imagesize: [u8; 4] = [0,0,0,0];
    let mut y_pixels_per_m: [u8; 4] = [0,0,0,0];
    let mut x_pixels_per_m: [u8; 4] = [0,0,0,0];
    let mut colors_used: [u8; 4] = [0,0,0,0];
    let mut important_colors: [u8; 4] = [0,0,0,0];


    bmp_file.read(&mut signature).unwrap();

    bmp_file.read(&mut filesize).unwrap();
    bmp_file.read(&mut reserved).unwrap();
    bmp_file.read(&mut dataoffset).unwrap();

    bmp_file.read(&mut size).unwrap();
    bmp_file.read(&mut width).unwrap();
    bmp_file.read(&mut height).unwrap();
    bmp_file.read(&mut planes).unwrap();
    bmp_file.read(&mut bits_per_pixel).unwrap();
    bmp_file.read(&mut compression).unwrap();
    bmp_file.read(&mut imagesize).unwrap();
    bmp_file.read(&mut y_pixels_per_m).unwrap();
    bmp_file.read(&mut x_pixels_per_m).unwrap();
    bmp_file.read(&mut colors_used).unwrap();
    bmp_file.read(&mut important_colors).unwrap();


    bmp.header.signature = signature;

    bmp.header.filesize = read_u32(&mut filesize);

    bmp.header.filesize = read_u32(&mut filesize);
    bmp.header.reserved = read_u32(&mut reserved);
    bmp.header.dataoffset = read_u32(&mut dataoffset);

    bmp.header.size = read_u32(&mut size);
    bmp.header.width = read_u32(&mut width);
    bmp.header.height = read_u32(&mut height);
    bmp.header.planes = read_u16(&mut planes);
    bmp.header.bits_per_pixel = read_u16(&mut bits_per_pixel);
    bmp.header.compression = read_u32(&mut compression);
    bmp.header.imagesize = read_u32(&mut imagesize);
    bmp.header.y_pixels_per_m = read_u32(&mut y_pixels_per_m);
    bmp.header.x_pixels_per_m = read_u32(&mut x_pixels_per_m);
    bmp.header.colors_used = read_u32(&mut colors_used);
    bmp.header.important_colors = read_u32(&mut important_colors);
}

fn write_header(bmp_file: &mut File, bmp: &BITMAP) {
    bmp_file.seek(SeekFrom::Start(0)).unwrap();

    let signature: [u8; 2] = bmp.header.signature;

    let filesize: [u8; 4] = transform_u32_to_u8(bmp.header.filesize);
    let reserved: [u8; 4] = transform_u32_to_u8(bmp.header.reserved);
    let dataoffset: [u8; 4] = transform_u32_to_u8(bmp.header.dataoffset);

    let size: [u8; 4] = transform_u32_to_u8(bmp.header.size);
    let width: [u8; 4] = transform_u32_to_u8(bmp.header.width);
    let height: [u8; 4] = transform_u32_to_u8(bmp.header.height);
    let planes: [u8; 2] = transform_u16_to_u8(bmp.header.planes);
    let bits_per_pixel: [u8; 2] = transform_u16_to_u8(bmp.header.bits_per_pixel);
    let compression: [u8; 4] = transform_u32_to_u8(bmp.header.compression);
    let imagesize: [u8; 4] = transform_u32_to_u8(bmp.header.imagesize);
    let y_pixels_per_m: [u8; 4] = transform_u32_to_u8(bmp.header.y_pixels_per_m);
    let x_pixels_per_m: [u8; 4] = transform_u32_to_u8(bmp.header.x_pixels_per_m);
    let colors_used: [u8; 4] = transform_u32_to_u8(bmp.header.colors_used);
    let important_colors: [u8; 4] = transform_u32_to_u8(bmp.header.important_colors);

    bmp_file.write(&signature).unwrap();

    bmp_file.write(&filesize).unwrap();
    bmp_file.write(&reserved).unwrap();
    bmp_file.write(&dataoffset).unwrap();

    bmp_file.write(&size).unwrap();
    bmp_file.write(&width).unwrap();
    bmp_file.write(&height).unwrap();
    bmp_file.write(&planes).unwrap();
    bmp_file.write(&bits_per_pixel).unwrap();
    bmp_file.write(&compression).unwrap();
    bmp_file.write(&imagesize).unwrap();
    bmp_file.write(&y_pixels_per_m).unwrap();
    bmp_file.write(&x_pixels_per_m).unwrap();
    bmp_file.write(&colors_used).unwrap();
    bmp_file.write(&important_colors).unwrap();
}

fn parse_raw_pixels(raw_pixels: &Vec<u8>, header: &BITMAP_HEADER_FILE) -> Vec<RGB> {
    let mut pixels: Vec<RGB> = Vec::new();

    let padding: u32 = header.width - ((header.width / 4) * 4);

    let mut width_count: u32 = 0;

    let mut i: u32 = 0;

    while i < header.imagesize {
        width_count += 1; // why Rust don't has x++; ?

        if width_count <= header.width // while is not the end of width, do this
        {
            let mut pixel: RGB = RGB { r: 0, g: 0, b: 0 };

            pixel.b = raw_pixels[i as usize];
            i += 1;
            pixel.g = raw_pixels[i as usize];
            i += 1;
            pixel.r = raw_pixels[i as usize];
            i += 1;

            pixels.push(pixel);
        }
        else // in other case move i to skip padding and reset the width count
        {
            i += padding;
            width_count = 0;
        }
    }

    return pixels;
}

fn unparse_raw_pixels(raw_pixels: &Vec<RGB>, header: &BITMAP_HEADER_FILE) -> Vec<u8> {
    let mut pixels: Vec<u8> = vec![0; header.imagesize as usize];

    let padding: u32 = header.width - ((header.width / 4) * 4);

    let mut width_count: u32 = 0;

    let mut i: u32 = 0;

    let mut normal_count: u32 = 0;

    while i < header.imagesize {
        width_count += 1; // why Rust don't has x++; ?

        if width_count <= header.width // while is not the end of width, do this
        {
            let pixel: RGB = *raw_pixels.get(normal_count as usize).unwrap();
            normal_count += 1;

            pixels[i as usize] = pixel.b;
            i += 1;
            pixels[i as usize] = pixel.g;
            i += 1;
            pixels[i as usize] = pixel.r;
            i += 1;
        }
        else // in other case move i to skip padding and reset the width count
        {
            for j in i..padding {
                pixels[j as usize] = 0;
            }

            i += padding;
            
            width_count = 0;
        }
    }

    return pixels;
}

fn flip_horizontally(pixels: &Vec<RGB>, header: &BITMAP_HEADER_FILE) -> Vec<RGB> {
    let mut flip_pixels: Vec<RGB> = Vec::new();

    let mut row: i32 = (header.height - 1) as i32;

    while row >= 0 {
        let mut col: u32 = 0;

        while col < header.width {
            let pos: u32 = (row as u32 * header.width) + col;

            let pixel: RGB = *pixels.get(pos as usize).unwrap();

            flip_pixels.push(pixel);

            col += 1;
        }
        row -= 1;
    }

    return flip_pixels;
}

pub fn bitmap_load(bmp_file: &mut File, bmp: &mut BITMAP) {
    read_header(bmp_file, bmp);
    
    bmp_file.seek(SeekFrom::Start(u64::from(bmp.header.dataoffset))).unwrap();

    let mut raw_pixels: Vec<u8> = vec![0; bmp.header.imagesize as usize];

    bmp_file.read(&mut raw_pixels).unwrap();

    let tmp_pixels: Vec<RGB> = parse_raw_pixels(&raw_pixels, &bmp.header);

    bmp.pixels = flip_horizontally(&tmp_pixels, &bmp.header);

    return;
}

pub fn bitmap_save(bmp_file: &mut File, bmp: &BITMAP) {
    write_header(bmp_file, bmp);

    if bmp.header.dataoffset > 54 {
        let buff: Vec<u8> = vec![0; (bmp.header.dataoffset - 54) as usize];
        bmp_file.write(&buff).unwrap();
    }

    let tmp_pixels: Vec<RGB> = flip_horizontally(&bmp.pixels, &bmp.header);

    let raw_pixels: Vec<u8> = unparse_raw_pixels(&tmp_pixels, &bmp.header);

    bmp_file.write(&raw_pixels).unwrap();

    return;
}