package bitmap

import (
	"fmt"
	"os"
)

type BITMAP_HEADER_FILE struct {
	signature []uint8

	filesize   uint32
	reserved   uint32
	dataoffset uint32

	size             uint32
	width            uint32
	height           uint32
	planes           uint16
	bits_per_pixel   uint16
	compression      uint32
	imagesize        uint32
	y_pixels_per_m   uint32
	x_pixels_per_m   uint32
	colors_used      uint32
	important_colors uint32
}

type RGB struct {
	r uint8
	g uint8
	b uint8
}

type BITMAP struct {
	header BITMAP_HEADER_FILE
	pixels *RGB
}

func readUint16(b []byte) uint16 {
	return uint16(b[0]) | uint16(b[1])<<8
}

func readUint32(b []byte) uint32 {
	return uint32(b[0]) | uint32(b[1])<<8 | uint32(b[2])<<16 | uint32(b[3])<<24
}

func read_header(bmp_file *os.File, bmp *BITMAP) {
	bmp.header.signature = make([]byte, 2)
	bmp_file.Read(bmp.header.signature)

	//            4   * 8 bytes = 32
	var filesize []byte = make([]byte, 4)
	bmp_file.Read(filesize)
	bmp.header.filesize = readUint32(filesize)
}

func Bitmap_load(bmp_file *os.File, bmp *BITMAP) int {
	bmp_file.Seek(0, 0)

	read_header(bmp_file, bmp)

	fmt.Printf("%c %c - %d\n", bmp.header.signature[0], bmp.header.signature[1], bmp.header.filesize)

	return 0
}
