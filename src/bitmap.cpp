#include "include/bitmap.hpp"

#include <iostream>
#include <fstream>

RGB* parse_raw_pixels(uint8_t* raw_pixels, BITMAP_HEADER_FILE* header)
{
    RGB* pixels = new RGB[(header->width * header->height)]; // (RGB*) malloc((header->width * header->height) * sizeof(RGB));

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
    uint8_t* pixels = new uint8_t[header->imagesize]; //(uint8_t*) malloc(header->imagesize * sizeof(uint8_t));

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
    RGB* flip_pixels = new RGB[(header->width * header->height)];// (RGB*) malloc((header->width * header->height) * sizeof(RGB));

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


void BITMAP::load(std::ifstream* file)
{
    file->seekg(0); // go to the start

    file->read((char*) &header.signature, 2); // copy only the signature

    // you already at bite 2 in the file fseek(bmp_file_fd, 2, SEEK_SET); // skip signature

    file->read((char*) &header, 52); // copy header

    if(header.signature[0] != 'B' || header.signature[1] != 'M')
    {
        std::cout << "Invalid BitMap file" << std::endl;
        exit(1);
    }

    if(header.bits_per_pixel != 24)
    {
        std::cout << "Only 24 bit per pixel support" << std::endl;
        exit(1);
    }

    file->seekg(header.dataoffset);

    // copy to memory rest of the file
    uint8_t* raw_pixels = new uint8_t[header.imagesize]; // (uint8_t*) malloc(bmp->header.imagesize * sizeof(uint8_t));
    file->read((char*) raw_pixels, header.imagesize);
 
    // parse pixels
    RGB* tmp_pixels = parse_raw_pixels(raw_pixels, &header);

    // then flip the image
    pixels = flip_horizontally(tmp_pixels, &header);

    // clear memory
    delete raw_pixels;
    delete tmp_pixels;

    return;
}
void BITMAP::load(std::string file)
{
    std::ifstream file_stream(file);

    load(&file_stream);

    file_stream.close();

    return;
}


void BITMAP::save(std::ofstream* file)
{
    file->seekp(0); // go to the start

    file->write((char*) header.signature, 2); // copy only the signature

    // you already at bite 2 in the file fseek(bmp_file_fd, 2, SEEK_SET); // skip signature

    file->write((char*) &header, 52); // copy header

    if (header.dataoffset > 54)
    {
        file->write(0, header.dataoffset - 54);
    }

    // fseek(bmp_file_fp, bmp->header.dataoffset, SEEK_SET);

    RGB* tmp_pixels = flip_horizontally(pixels, &header);

    // copy to memory rest of the file
    uint8_t* raw_pixels = unparse_raw_pixels(tmp_pixels, &header);

    file->write((char*) raw_pixels, header.imagesize);

    // clear memory
    delete raw_pixels;
    delete tmp_pixels;

    return;
}
void BITMAP::save(std::string file)
{
    std::ofstream file_stream(file);

    save(&file_stream);

    file_stream.close();

    return;
}


BITMAP::BITMAP()
{
    pixels = new RGB[0];
}
BITMAP::BITMAP(std::string file)
{
    load(file);
}
BITMAP::BITMAP(std::ifstream* file)
{
    load(file);
}


BITMAP::~BITMAP()
{
    delete pixels;
}