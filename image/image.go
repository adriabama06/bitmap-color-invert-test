package image

import "github.com/adriabama06/bitmap-color-invert-test/bitmap"

func Invert_colors(bmp *bitmap.BITMAP) {
	var size uint32 = (bmp.Header.Width * bmp.Header.Height)
	for i := uint32(0); i < size; i++ {
		var original_pixel bitmap.RGB = bmp.Pixels[i]

		var inverted_pixel bitmap.RGB = bitmap.RGB{
			R: 255 - original_pixel.R,
			G: 255 - original_pixel.G,
			B: 255 - original_pixel.B,
		}

		bmp.Pixels[i] = inverted_pixel
	}
}
