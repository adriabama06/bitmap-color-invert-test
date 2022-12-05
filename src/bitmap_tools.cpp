#include "include/bitmap.hpp"

void BITMAP::invert_colors()
{
    uint32_t image_size = (header.width * header.height);

    for (uint32_t i = 0; i < image_size; i++)
    {
        RGB* pixel = &pixels[i];

        pixel->b = 255 - pixel->b;
        pixel->g = 255 - pixel->g;
        pixel->r = 255 - pixel->r;
    }

    return;
}