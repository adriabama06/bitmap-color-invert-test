#include "include/image.h"

void invert_colors(BITMAP* bmp)
{
    uint32_t image_size = (bmp->header.width * bmp->header.height);

    for (uint32_t i = 0; i < image_size; i++)
    {
        RGB* pixel = &bmp->pixels[i];

        pixel->b = 255 - pixel->b;
        pixel->g = 255 - pixel->g;
        pixel->r = 255 - pixel->r;
    }

    return;
}