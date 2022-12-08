#include "../common.h"
#include "03input.h"

void CountOverlapAndFindPerfect(char* input[], uint32_t length)
{
    // Parse input, eg: "#1 @ 108,350: 22x29"
    uint32_t* startX = malloc(length * sizeof(uint32_t));
    uint32_t* startY = malloc(length * sizeof(uint32_t));
    uint32_t* widthX = malloc(length * sizeof(uint32_t));
    uint32_t* widthY = malloc(length * sizeof(uint32_t));
    for (uint32_t stringI = 0; stringI < length; stringI++)
    {
        uint32_t charI = 0;
        char* string = input[stringI];
        char c = string[charI];
        uint32_t atI = 0;
        uint32_t commaI = 0;
        uint32_t colonI = 0;
        uint32_t xI = 0;
        while (c != '\0')
        {
            if (c == '@')
            {
                atI = charI;
            }
            else if (c == ',')
            {
                commaI = charI;
            }
            else if (c == ':')
            {
                colonI = charI;
            }
            else if (c == 'x')
            {
                xI = charI;
            }
            charI++;
            c = input[stringI][charI];
        }
        uint32_t nullI = charI;

        // Build sub strings, eg: "#1 @ 108,350: 22x29"
        char xString[4] = {0};
        uint32_t destI = 0;
        for (uint32_t sourceI = atI + 2; sourceI < commaI; sourceI++, destI++)
        {
            xString[destI] = string[sourceI];
        }

        char yString[4] = {0};
        destI = 0;
        for (uint32_t sourceI = commaI + 1; sourceI < colonI; sourceI++, destI++)
        {
            yString[destI] = string[sourceI];
        }

        char widthXString[4] = {0};
        destI = 0;
        for (uint32_t sourceI = colonI + 2; sourceI < xI; sourceI++, destI++)
        {
            widthXString[destI] = string[sourceI];
        }

        char widthYString[4] = {0};
        destI = 0;
        for (uint32_t sourceI = xI + 1; sourceI < nullI; sourceI++, destI++)
        {
            widthYString[destI] = string[sourceI];
        }

        startX[stringI] = atoi(xString);
        startY[stringI] = atoi(yString);
        widthX[stringI] = atoi(widthXString);
        widthY[stringI] = atoi(widthYString);
    }

    // Add a count for each square inch
    #define MAX_X 1000
    #define MAX_Y 1000
    uint32_t** inches = malloc(MAX_X * sizeof(uint32_t*));
    for (uint32_t x = 0; x < MAX_X; x++)
    {
        inches[x] = malloc(MAX_Y * sizeof(uint32_t));
        memset(inches[x], 0, MAX_Y * sizeof(uint32_t));
    }
    uint32_t count = 0;
    for (uint32_t i = 0; i < length; i++)
    {
        for (uint32_t x = startX[i]; x < startX[i] + widthX[i]; x++)
        {
            for (uint32_t y = startY[i]; y < startY[i] + widthY[i]; y++)
            {
                inches[x][y]++;
                if (inches[x][y] == 2)
                {
                    count++;
                }
            }
        }
    }

    printf("Overlaped inces sqr: %d\n", count);

    // Now find the perfect one
    for (uint32_t i = 0; i < length; i++)
    {
        bool perfect = true;
        for (uint32_t x = startX[i]; x < startX[i] + widthX[i] && perfect; x++)
        {
            for (uint32_t y = startY[i]; y < startY[i] + widthY[i] && perfect; y++)
            {
                if (inches[x][y] != 1)
                {
                    perfect = false;
                }
            }
        }

        if (perfect)
        {
            printf("Perfect one is %d\n", i + 1);
            break;
        }
    }
}

int main(int argc, char* argv[])
{
    CountOverlapAndFindPerfect(input, ARRAY_SIZE(input));
    return 0;
}