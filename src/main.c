#include "include/bitmap.h"
#include "include/image.h"

#include <stdio.h>

int main(int argc, const char** argv)
{
    if(argc < 3)
    {
        printf("%s <input.bmp> <output.bmp>\n", argv[0]);
        return 0;
    }

    FILE* to_open = fopen(argv[1], "rb");

    BITMAP bmp;

    bitmap_load(to_open, &bmp);    

    fclose(to_open);

    invert_colors(&bmp);    
    
    FILE* to_save = fopen(argv[2], "wb");

    bitmap_save(to_save, &bmp);

    fclose(to_save);

    return 0;
}