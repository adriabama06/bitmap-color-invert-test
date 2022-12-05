#pragma once

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <stdint.h>

// http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm
typedef struct BITMAP_HEADER_FILE_S
{
    uint32_t filesize;
    uint32_t reserved;
    uint32_t dataoffset;

    uint32_t size;
    uint32_t width;
    uint32_t height;
    uint16_t planes;
    uint16_t bits_per_pixel;
    uint32_t compression;
    uint32_t imagesize;
    uint32_t y_pixels_per_m;
    uint32_t x_pixels_per_m;
    uint32_t colors_used;
    uint32_t important_colors;

    uint8_t signature[2]; // <-- this in theory need to go at the start, but for the padding this cause problems
} BITMAP_HEADER_FILE;

typedef struct RGB_S {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} RGB;

class BITMAP
{
private:
/* NOTHING */
public:
    BITMAP();
    BITMAP(std::string file);
    BITMAP(std::ifstream* file);

    ~BITMAP();

    void load(std::string file);
    void load(std::ifstream* file);

    void save(std::string file);
    void save(std::ofstream* file);

    void invert_colors();

    BITMAP_HEADER_FILE header;
    RGB* pixels; // pointer or vector? Who is faster?
};