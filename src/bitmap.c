#include "include/bitmap.h"

#include <stdlib.h>
#include <string.h>

RGB* parse_raw_pixels(uint8_t* raw_pixels, BITMAP_HEADER_FILE* header)
{
    RGB* pixels = (RGB*) malloc((header->width * header->height) * sizeof(RGB));

    uint32_t padding = header->width - ((header->width / 4) * 4);

    uint32_t width_count = 0;
    uint32_t normal_count = 0;

    for (uint32_t i = 0; i < header->imagesize;)
    {
        width_count++;

        if(width_count <= header->width) // while is not the end of width, do this
        {
            RGB pixel;

            pixel.b = raw_pixels[i++];
            pixel.g = raw_pixels[i++];
            pixel.r = raw_pixels[i++];

            pixels[normal_count++] = pixel;
        }
        else // in other case move i to skip padding and reset the width count
        {
            i += padding;
            width_count = 0;
        }
    }

    return pixels;
}

uint8_t* unparse_raw_pixels(RGB* raw_pixels, BITMAP_HEADER_FILE* header)
{
    uint8_t* pixels = (uint8_t*) malloc(header->imagesize * sizeof(uint8_t));

    uint32_t padding = header->width - ((header->width / 4) * 4);

    uint32_t width_count = 0;
    uint32_t normal_count = 0;

    for (uint32_t i = 0; i < header->imagesize;)
    {
        width_count++;

        if(width_count <= header->width) // while is not the end of width, do this
        {
            RGB pixel = raw_pixels[normal_count++];

            pixels[i++] = pixel.b;
            pixels[i++] = pixel.g;
            pixels[i++] = pixel.r;
        }
        else // in other case move i to skip padding and reset the width count
        {
            //i += padding;
            
            for (uint32_t j = i; j < padding; j++)
            {
                pixels[j] = (uint8_t) 0;
            }
            
            i += padding;

            width_count = 0;
        }
    }

    return pixels;
}

RGB* flip_horizontally(RGB* pixels, BITMAP_HEADER_FILE* header)
{
    RGB* flip_pixels = (RGB*) malloc((header->width * header->height) * sizeof(RGB));

    uint32_t normal_count = 0;

    for(int32_t row = header->height - 1; row >= 0; row--)
    {
        for(uint32_t col = 0; col < header->width; col++)
        {
            uint32_t pos = (row * header->width) + col;

            RGB pixel = pixels[pos];
            
            flip_pixels[normal_count++] = pixel;
        }
    }

    return flip_pixels;
}

// if return value is > 0 has errors 
void bitmap_load(FILE* bmp_file_fp, BITMAP* bmp)
{
    /*
    For best performance i don't disable padding, see "test/padding.c"
    */

    fseek(bmp_file_fp, 0, SEEK_SET); // go to the start

    fread(&bmp->header.signature, 1, 2, bmp_file_fp); // copy only the signature

    // you already at bite 2 in the file fseek(bmp_file_fd, 2, SEEK_SET); // skip signature

    fread(&bmp->header, 1, 52, bmp_file_fp); // copy header

    if(bmp->header.signature[0] != 'B' || bmp->header.signature[1] != 'M')
    {
        printf("Invalid BitMap file\n");
        exit(1);
    }

    if(bmp->header.bits_per_pixel != 24)
    {
        printf("Only 24 bit per pixel support\n");
        exit(1);
    }

    fseek(bmp_file_fp, bmp->header.dataoffset, SEEK_SET);

    // copy to memory rest of the file
    uint8_t* raw_pixels = (uint8_t*) malloc(bmp->header.imagesize * sizeof(uint8_t));
    fread(raw_pixels, 1, bmp->header.imagesize, bmp_file_fp);
 
    // parse pixels
    /*
    3 pixels ->
    data = [b, g, r,   b, g ,r,   b, g, r]
    as pixels RGB struct
    pixel1.b = data[0]
    pixel1.g = data[1]
    pixel1.r = data[2]
    pixel2.b = data[3]
    pixel2.g = data[4]
    pixel2.r = data[5]
    pixel3.b = data[6]
    pixel3.g = data[7]
    pixel3.r = data[8]
    */
    RGB* tmp_pixels = parse_raw_pixels(raw_pixels, &bmp->header);

    // then flip the image
    bmp->pixels = flip_horizontally(tmp_pixels, &bmp->header);

    // clear memory
    free(raw_pixels);
    free(tmp_pixels);

    return;
}

// if return value is > 0 has errors 
void bitmap_save(FILE* bmp_file_fp, BITMAP* bmp)
{
    /*
    For best performance i don't disable padding, see "test/padding.c"
    */

    fseek(bmp_file_fp, 0, SEEK_SET); // go to the start

    fwrite(bmp->header.signature, 1, 2, bmp_file_fp); // copy only the signature

    // you already at bite 2 in the file fseek(bmp_file_fd, 2, SEEK_SET); // skip signature

    fwrite(&bmp->header, 1, 52, bmp_file_fp); // copy header

    if (bmp->header.dataoffset > 54)
    {
        fwrite((uint8_t) 0, 1, bmp->header.dataoffset - 54, bmp_file_fp);
    }

    // fseek(bmp_file_fp, bmp->header.dataoffset, SEEK_SET);

    RGB* tmp_pixels = flip_horizontally(bmp->pixels, &bmp->header);

    // copy to memory rest of the file
    uint8_t* raw_pixels = unparse_raw_pixels(tmp_pixels, &bmp->header);

    fwrite(raw_pixels, bmp->header.imagesize, 1, bmp_file_fp);

    // clear memory
    free(raw_pixels);
    free(tmp_pixels);

    return;
}