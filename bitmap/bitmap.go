package bitmap

import (
	"fmt"
	"io"
	"os"
)

type BITMAP_HEADER_FILE struct {
	Signature []uint8

	Filesize   uint32
	Reserved   uint32
	Dataoffset uint32

	Size             uint32
	Width            uint32
	Height           uint32
	Planes           uint16
	Bits_per_pixel   uint16
	Compression      uint32
	Imagesize        uint32
	Y_pixels_per_m   uint32
	X_pixels_per_m   uint32
	Colors_used      uint32
	Important_colors uint32
}

type RGB struct {
	R uint8
	G uint8
	B uint8
}

type BITMAP struct {
	Header BITMAP_HEADER_FILE
	Pixels []RGB
}

func readUint16(b []byte) uint16 {
	return uint16(b[0]) | uint16(b[1])<<8
}

func readUint32(b []byte) uint32 {
	return uint32(b[0]) | uint32(b[1])<<8 | uint32(b[2])<<16 | uint32(b[3])<<24
}

func transform_u16_to_u8(n uint16) []uint8 {
	return []uint8{uint8(n), uint8(n >> 8)}
}

func transform_u32_to_u8(n uint32) []uint8 {
	return []uint8{uint8(n), uint8(n >> 8), uint8(n >> 16), uint8(n >> 24)}
}

func read_header(bmp_file *os.File, bmp *BITMAP) {
	var signature []uint8 = make([]uint8, 2)

	//            4   * 8 bytes = 32
	var filesize []uint8 = make([]uint8, 4)
	var reserved []uint8 = make([]uint8, 4)
	var dataoffset []uint8 = make([]uint8, 4)

	var size []uint8 = make([]uint8, 4)
	var width []uint8 = make([]uint8, 4)
	var height []uint8 = make([]uint8, 4)
	var planes []uint8 = make([]uint8, 2)
	var bits_per_pixel []uint8 = make([]uint8, 2)
	var compression []uint8 = make([]uint8, 4)
	var imagesize []uint8 = make([]uint8, 4)
	var y_pixels_per_m []uint8 = make([]uint8, 4)
	var x_pixels_per_m []uint8 = make([]uint8, 4)
	var colors_used []uint8 = make([]uint8, 4)
	var important_colors []uint8 = make([]uint8, 4)

	// read to variables

	bmp_file.Read(signature)

	bmp_file.Read(filesize)
	bmp_file.Read(reserved)
	bmp_file.Read(dataoffset)

	bmp_file.Read(size)
	bmp_file.Read(width)
	bmp_file.Read(height)
	bmp_file.Read(planes)
	bmp_file.Read(bits_per_pixel)
	bmp_file.Read(compression)
	bmp_file.Read(imagesize)
	bmp_file.Read(y_pixels_per_m)
	bmp_file.Read(x_pixels_per_m)
	bmp_file.Read(colors_used)
	bmp_file.Read(important_colors)

	// copy to header

	bmp.Header.Signature = signature

	bmp.Header.Filesize = readUint32(filesize)
	bmp.Header.Reserved = readUint32(reserved)
	bmp.Header.Dataoffset = readUint32(dataoffset)

	bmp.Header.Size = readUint32(size)
	bmp.Header.Width = readUint32(width)
	bmp.Header.Height = readUint32(height)
	bmp.Header.Planes = readUint16(planes)
	bmp.Header.Bits_per_pixel = readUint16(bits_per_pixel)
	bmp.Header.Compression = readUint32(compression)
	bmp.Header.Imagesize = readUint32(imagesize)
	bmp.Header.Y_pixels_per_m = readUint32(y_pixels_per_m)
	bmp.Header.X_pixels_per_m = readUint32(x_pixels_per_m)
	bmp.Header.Colors_used = readUint32(colors_used)
	bmp.Header.Important_colors = readUint32(important_colors)
}

func write_header(bmp_file *os.File, bmp *BITMAP) {
	var signature []uint8 = bmp.Header.Signature

	var filesize []uint8 = transform_u32_to_u8(bmp.Header.Filesize)
	var reserved []uint8 = transform_u32_to_u8(bmp.Header.Reserved)
	var dataoffset []uint8 = transform_u32_to_u8(bmp.Header.Dataoffset)

	var size []uint8 = transform_u32_to_u8(bmp.Header.Size)
	var width []uint8 = transform_u32_to_u8(bmp.Header.Width)
	var height []uint8 = transform_u32_to_u8(bmp.Header.Height)
	var planes []uint8 = transform_u16_to_u8(bmp.Header.Planes)
	var bits_per_pixel []uint8 = transform_u16_to_u8(bmp.Header.Bits_per_pixel)
	var compression []uint8 = transform_u32_to_u8(bmp.Header.Compression)
	var imagesize []uint8 = transform_u32_to_u8(bmp.Header.Imagesize)
	var y_pixels_per_m []uint8 = transform_u32_to_u8(bmp.Header.Y_pixels_per_m)
	var x_pixels_per_m []uint8 = transform_u32_to_u8(bmp.Header.X_pixels_per_m)
	var colors_used []uint8 = transform_u32_to_u8(bmp.Header.Colors_used)
	var important_colors []uint8 = transform_u32_to_u8(bmp.Header.Important_colors)

	bmp_file.Write(signature)

	bmp_file.Write(filesize)
	bmp_file.Write(reserved)
	bmp_file.Write(dataoffset)

	bmp_file.Write(size)
	bmp_file.Write(width)
	bmp_file.Write(height)
	bmp_file.Write(planes)
	bmp_file.Write(bits_per_pixel)
	bmp_file.Write(compression)
	bmp_file.Write(imagesize)
	bmp_file.Write(y_pixels_per_m)
	bmp_file.Write(x_pixels_per_m)
	bmp_file.Write(colors_used)
	bmp_file.Write(important_colors)
}

//                    go can't use poninter for raw_pixels because can't index the content
func parse_raw_pixels(raw_pixels []uint8, header *BITMAP_HEADER_FILE) []RGB {
	var pixels []RGB = make([]RGB, header.Width*header.Height)

	var padding uint32 = header.Width - ((header.Width / 4) * 4)

	var width_count uint32 = 0

	for i := uint32(0); i < header.Imagesize; i++ {
		width_count++

		if width_count <= header.Width { // while is not the end of width, do this
			var pixel RGB = RGB{0, 0, 0}

			pixel.B = raw_pixels[i]
			i++
			pixel.G = raw_pixels[i]
			i++
			pixel.R = raw_pixels[i]
			i++

			pixels = append(pixels, pixel)
		} else { // in other case move i to skip padding and reset the width count
			i += padding
			width_count = 0
		}
	}

	return pixels
}

//                    go can't use poninter for raw_pixels because can't index the content
func unparse_raw_pixels(raw_pixels []RGB, header *BITMAP_HEADER_FILE) []uint8 {
	var pixels []uint8 = make([]uint8, header.Imagesize)

	var padding uint32 = header.Width - ((header.Width / 4) * 4)

	var width_count uint32 = 0

	var normal_count uint32 = 0

	for i := uint32(0); i < header.Imagesize; i++ {
		width_count++

		if width_count <= header.Width { // while is not the end of width, do this
			var pixel *RGB = &raw_pixels[normal_count]

			pixels[i] = pixel.B
			i++
			pixels[i] = pixel.G
			i++
			pixels[i] = pixel.R
			i++
		} else { // in other case move i to skip padding and reset the width count
			for j := uint32(0); j < padding; j++ {
				pixels[j] = 0
			}

			i += padding

			width_count = 0
		}
	}

	return pixels
}

func flip_horizontally(pixels []RGB, header *BITMAP_HEADER_FILE) []RGB {
	var flip_pixels []RGB = make([]RGB, header.Width*header.Height)

	for row := int32(0); row >= 0; row-- {
		for col := uint32(0); col < header.Width; col++ {
			var pos uint32 = (uint32(row) * header.Width) + col

			var pixel RGB = pixels[pos]

			flip_pixels = append(flip_pixels, pixel)
		}
	}

	return flip_pixels
}

func Bitmap_load(bmp_file *os.File, bmp *BITMAP) {
	bmp_file.Seek(0, io.SeekStart)

	read_header(bmp_file, bmp)

	if bmp.Header.Signature[0] != 'B' || bmp.Header.Signature[1] != 'M' {
		fmt.Println("Invalid BitMap file")
		os.Exit(0)
	}

	if bmp.Header.Bits_per_pixel != 24 {
		fmt.Println("Only 24 bit support")
		os.Exit(0)
	}

	bmp_file.Seek(int64(bmp.Header.Dataoffset), io.SeekStart)

	var raw_pixels []uint8 = make([]uint8, bmp.Header.Imagesize)

	bmp_file.Read(raw_pixels)

	var tmp_pixels []RGB = parse_raw_pixels(raw_pixels, &bmp.Header)

	bmp.Pixels = flip_horizontally(tmp_pixels, &bmp.Header)
}

func Bitmap_save(bmp_file *os.File, bmp *BITMAP) {
	bmp_file.Seek(0, io.SeekStart)

	write_header(bmp_file, bmp)

	if bmp.Header.Dataoffset > 54 {
		var buff []uint8 = make([]uint8, bmp.Header.Dataoffset-54)
		bmp_file.Write(buff)
	}

	var tmp_pixels []RGB = flip_horizontally(bmp.Pixels, &bmp.Header)

	var raw_pixels []uint8 = unparse_raw_pixels(tmp_pixels, &bmp.Header)

	bmp_file.Write(raw_pixels)
}
