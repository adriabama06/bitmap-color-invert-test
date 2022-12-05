#include "include/bitmap.hpp"

#include <iostream>
#include <string>

int main(int argc, const char** argv)
{
    if(argc < 3)
    {
        std::cout << argv[0] << "<input.bmp> <output.bmp>" << std::endl;
        return 0;
    }

    std::string input = std::string(argv[1]);
    std::string output = std::string(argv[2]);

    BITMAP bitmap(input);

    bitmap.invert_colors();

    bitmap.save(output);

    return 0;
}