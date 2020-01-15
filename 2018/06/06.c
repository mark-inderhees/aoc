#include "..\common.h"
#include "06input.h"

uint32_t FindDistance(pair a, pair b)
{
    return abs(a.x - b.x) + abs(a.y - b.y);
}

uint32_t FindLargestArea(pair input[], uint32_t length)
{
    uint32_t minX = UINT32_MAX;
    uint32_t minY = UINT32_MAX;
    uint32_t maxX = 0;
    uint32_t maxY = 0;
    for (uint32_t i = 0; i < length; i++)
    {
        if (input[i].x < minX)
        {
            minX = input[i].x;
        }

        if (input[i].y < minY)
        {
            minY = input[i].y;
        }

        if (input[i].x > maxX)
        {
            maxX = input[i].x;
        }

        if (input[i].y > maxY)
        {
            maxY = input[i].y;
        }
    }

    // Build grid!
    uint32_t width = maxX - minX + 1;
    uint32_t height = maxY - minY + 1;
    uint32_t size = width * height;
    uint8_t* grid = malloc(size);
    memset(grid, UINT8_MAX, size);
    uint32_t* dotCount = malloc(sizeof(uint32_t) * length);
    memset(dotCount, 0, sizeof(uint32_t) * length);
    for (uint32_t x = 0; x < width; x++)
    {
        for (uint32_t y = 0; y < height; y++)
        {
            uint32_t minDistance = UINT32_MAX;
            pair p = {x + minX, y + minY};
            for (uint32_t i = 0; i < length; i++)
            {
                uint32_t d = FindDistance(p, input[i]);
                if (d < minDistance)
                {
                    minDistance = d;
                    grid[x + y * width] = i;
                }
            }
            dotCount[grid[x + y * width]]++;
        }
    }

    // Erase anyone touching the edges
    for (uint32_t x = 0; x < width; x++)
    {
        dotCount[grid[x + 0 * width]] = 0;
        dotCount[grid[x + (height-1) * width]] = 0;
    }

    for (uint32_t y = 0; y < height; y++)
    {
        dotCount[grid[0 + y * width]] = 0;
        dotCount[grid[(width-1) + y * width]] = 0;
    }

    // Find largest count
    uint32_t largestCount = 0;
    for (uint32_t i = 0; i < length; i++)
    {
        if (dotCount[i] > largestCount)
        {
            largestCount = dotCount[i];
        }
    }

    return largestCount;
}

int main(int argc, char* argv[])
{
    printf("FindLargestArea %d\n", FindLargestArea(input, ARRAY_SIZE(input)));
    return 0;
}