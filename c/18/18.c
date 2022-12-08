#include "..\common.h"
#include "18input.h"

// open ground (.)
// trees (|)
// lumberyard (#)

uint32_t CountAdjacent(char* map, uint32_t x, uint32_t y, uint32_t width, uint32_t height, char lookingFor)
{
    uint32_t xStart = (x > 0) ? x - 1 : x;
    uint32_t xEnd = (x < width - 1) ? x + 1 : x;
    uint32_t yStart = (y > 0) ? y - 1 : y;
    uint32_t yEnd = (y < height - 1) ? y + 1 : y;
    uint32_t count = 0;
    for (uint32_t yI = yStart; yI <= yEnd; yI++)
    {
        for (uint32_t xI = xStart; xI <= xEnd; xI++)
        {
            if (yI == y && xI == x)
            {
                continue;
            }

            char square = map[xI + yI * width];
            if (square == lookingFor)
            {
                count++;
            }
        }
    }
    return count;
}

char ConvertSquare(char* map, uint32_t x, uint32_t y, uint32_t width, uint32_t height)
{
    char square = map[x + y * width];
    if (square == '.')
    {
        // An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
        uint32_t count = CountAdjacent(map, x, y, width, height, '|');
        if (count >= 3)
        {
            return '|';
        }
        else
        {
            return '.';
        }
    }
    else if (square == '|')
    {
        // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
        uint32_t count = CountAdjacent(map, x, y, width, height, '#');
        if (count >= 3)
        {
            return '#';
        }
        else
        {
            return '|';
        }
    }
    else if (square == '#')
    {
        // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
        uint32_t countYard = CountAdjacent(map, x, y, width, height, '#');
        uint32_t countTrees = CountAdjacent(map, x, y, width, height, '|');
        if (countYard >= 1 && countTrees >= 1)
        {
            return '#';
        }
        else
        {
            return '.';
        }
    }
    else
    {
        assert(false);
    }
}

void DrawMap(char* map, uint32_t width, uint32_t height)
{
    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            printf("%c", map[x + y * width]);
        }
        printf("\n");
    }
}


uint32_t countTreesPrevious = 0;
uint32_t countYardsPrevious = 0;
uint32_t countEmptyPrevious = 0;
uint32_t GetScore(char* map, uint32_t width, uint32_t height)
{
    uint32_t countTrees = 0;
    uint32_t countYards = 0;
    uint32_t countEmpty = 0;
    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            char square = map[x + y * width];
            if (square == '|')
            {
                countTrees++;
            }
            else if (square == '#')
            {
                countYards++;
            }
            else
            {
                countEmpty++;
            }
        }
    }

    assert(countTrees + countYards + countEmpty == width * height);

    uint32_t score = countTrees * countYards;

    printf("%d - %d %d %d - %+03d %+03d %+03d\n",
        score,
        countTrees, countYards, countEmpty,
        countTrees - countTreesPrevious, countYards - countYardsPrevious, countEmpty - countEmptyPrevious);

    countTreesPrevious = countTrees;
    countYardsPrevious = countYards;
    countEmptyPrevious = countEmpty;

    return score;
}

uint32_t Problem1(char* input[], uint32_t length)
{
    uint32_t width = strlen(input[0]);
    uint32_t height = length;
    uint32_t size = width * height;
    char* map1 = malloc(size);
    char* map2 = malloc(size);
    for (uint32_t i = 0; i < length; i++)
    {
        memcpy(&map1[i * width], input[i], width);
    }

    // printf("Initial state:\n");
    // DrawMap(map1, width, height);

    char* mapSource = map1;
    char* mapDestination = map2;

    // uint32_t time = 10; // Problem one!!!
    uint32_t time = 10000; // Problem two - look for the pattern. From something like iteration 450, there is a 28 entry pattern. Use ((1000000000 - 450) % 28) + 450 --> index of answer
    // uint32_t time = 1000000000;
    // uint32_t previousScore = 0;
    for (uint32_t i = 0; i < time; i++)
    {
        if (i % 2 == 0)
        {
            mapSource = map1;
            mapDestination = map2;
        }
        else
        {
            mapSource = map2;
            mapDestination = map1;
        }

        for (uint32_t y = 0; y < height; y++)
        {
            for (uint32_t x = 0; x < width; x++)
            {
                mapDestination[x + y * width] = ConvertSquare(mapSource, x, y, width, height);
            }
        }

        // printf("\nAfter %d minute::\n", i + 1);
        // DrawMap(mapDestination, width, height);

        GetScore(mapDestination, width, height);
        // uint32_t score = GetScore(mapDestination, width, height);
        // printf("Score after %d: %d (diff: %d)\n", i + 1, score, score - previousScore);
        // previousScore = score;
    }

    return GetScore(mapDestination, width, height);
}

int main()
{
    // printf("Problem1: %d\n", Problem1(testData, ARRAY_SIZE(testData)));
    printf("Problem1: %d\n", Problem1(input, ARRAY_SIZE(input)));
    return 0;
}