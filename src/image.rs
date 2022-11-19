use crate::bitmap;

pub fn invert_colors(bmp: &mut bitmap::BITMAP) {
    for i in 0..(bmp.header.width * bmp.header.height) {
        let original_pixel = bmp.pixels.get(i as usize).unwrap();

        let inverted_pixel: bitmap::RGB = bitmap::RGB {
            r: 255 - original_pixel.r,
            g: 255 - original_pixel.g,
            b: 255 - original_pixel.b
        };

        bmp.pixels[i as usize] = inverted_pixel;
    }
}