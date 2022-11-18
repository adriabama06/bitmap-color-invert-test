package main

import (
	"fmt"
	"os"

	"github.com/adriabama06/bitmap-color-invert-test/bitmap"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Printf("%s <input.bmp> <output.bmp>\n", os.Args[0])
		return
	}

	to_read, _ := os.Open(os.Args[1])

	var bmp bitmap.BITMAP

	bitmap.Bitmap_load(to_read, &bmp)

	to_read.Close()
}
