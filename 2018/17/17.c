#include "..\common.h"
#include "17input.h"

void Fall(uint32_t x, uint32_t y);

uint32_t _count = 0;
char* _map;
uint32_t _width;
uint32_t _height;

// Index is an in/out. Contains start of string at input. Output is end of integer
uint32_t atoiHelper(char* string, uint32_t* index)
{
    uint32_t stringStart = *index;
    uint32_t i = stringStart;
    char c = string[i];
    while (c >= '0' && c <= '9')
    {
        i++;
        c = string[i];
    }
    string[i] = '\0';
    *index = i;
    return atoi(&string[stringStart]);
}

void DrawMap(char* map, uint32_t width, uint32_t height)
{
    printf("--------------------------------\n");
    for (uint32_t y = 0; y < height; y++)
    {
        printf("%02d ", y);
        for (uint32_t x = 0; x < width; x++)
        {
            printf("%c", map[x + y * width]);
        }
        printf("\n");
    }
}

bool Flow(uint32_t x, uint32_t y, int32_t offset)
{
    char c;
    // printf("Flowing row %d from x %d in direction %d\n", y, x, offset);
    do
    {
        if (_map[x + y * _width] != '|')
        {
            _map[x + y * _width] = '|';
            _count++;
        }

        // Can we fall?
        c = _map[x + (y + 1) * _width];
        if (c == '.')
        {
            // Can fall
            Fall(x, y + 1);
            return true;
        }

        // Can we flow more?
        x = x + offset;
        c = _map[x + y * _width];
    } while (c == '.');

    // Hit a wall
    return false;
}

void Fill(uint32_t x, uint32_t y, int32_t offset)
{
    char c;
    do
    {
        _map[x + y * _width] = '~';

        // Can we flow more?
        x = x + offset;
        c = _map[x + y * _width];
    } while (c == '.' || c == '|');

    // printf(" until %d", x - offset);
}

void Fall(uint32_t x, uint32_t y)
{
    DrawMap(_map, _width, _height);
    // printf("Falling from %d, %d\n", x, y);
    // Flow down as long as possible
    while (true)
    {
        if (_map[x + y * _width] != '|')
        {
            _map[x + y * _width] = '|';
            _count++;
        }

        // Check if this is about to go off the map
        if (y + 1 >= _height)
        {
            // printf("Fell off map at %d, %d\n", x, y);
            return;
        }

        if (_map[x + (y + 1) * _width] != '.')
        {
            break;
        }

        y++;
    }

    while (true)
    {
        bool flowLeft =  Flow(x, y, -1);
        bool flowRight = Flow(x, y, +1);

        if (flowLeft || flowRight)
        {
            break;
        }

        // Fill up this row, back up one and flow again
        // printf("Filling row %d", y);
        Fill(x, y, -1);
        Fill(x, y, +1);
        // printf("\n");
        y--;
    }
}

uint32_t CountReachable(char* inputReadOnly[], uint32_t length)
{
    // Parse input
    bool* inputOneIsX = malloc(sizeof(bool) * length);
    uint32_t* inputOne = malloc(sizeof(uint32_t) * length);
    uint32_t* inputTwoStart = malloc(sizeof(uint32_t) * length);
    uint32_t* inputTwoStop = malloc(sizeof(uint32_t) * length);
    uint32_t minX = UINT32_MAX;
    uint32_t maxX = 0;
    uint32_t minY = UINT32_MAX;
    uint32_t maxY = 0;
    for (uint32_t i = 0; i < length; i++)
    {
        uint32_t stringLen = strlen(inputReadOnly[i]) + 1;
        char* input = malloc(stringLen);
        memcpy(input, inputReadOnly[i], stringLen);
        uint32_t charIndex = 0;
        inputOneIsX[i] = (input[charIndex] == 'x');
        charIndex += 2;
        inputOne[i] = atoiHelper(input, &charIndex);
        charIndex += 4;
        inputTwoStart[i] = atoiHelper(input, &charIndex);
        charIndex += 2;
        inputTwoStop[i] = atoiHelper(input, &charIndex);
        // printf("%c=%d, %c=%d..%d\n",
        //     inputOneIsX[i] ? 'x' : 'y',
        //     inputOne[i],
        //     inputOneIsX[i] ? 'y' : 'x',
        //     inputTwoStart[i],
        //     inputTwoStop[i]);
        free(input);

        if (inputOneIsX[i])
        {
            maxX = (inputOne[i] > maxX) ? inputOne[i] : maxX;
            minX = (inputOne[i] < minX) ? inputOne[i] : minX;
            minY = (inputTwoStart[i] < minY) ? inputTwoStart[i] : minY;
            maxY = (inputTwoStop[i] > maxY) ? inputTwoStop[i] : maxY;
        }
        else
        {
            maxY = (inputOne[i] > maxY) ? inputOne[i] : maxY;
            minY = (inputOne[i] < minY) ? inputOne[i] : minY;
            minX = (inputTwoStart[i] < minX) ? inputTwoStart[i] : minX;
            maxX = (inputTwoStop[i] > maxX) ? inputTwoStop[i] : maxX;
        }
    }

    minX--;
    maxX++;
    printf("x: %d to %d, y: %d to %d\n", minX, maxX, minY, maxY);

    // Build a map!
    uint32_t width = maxX - minX + 1;
    uint32_t height = maxY + 1; // Allocate full height, so don't need to worry about y offset
    uint32_t mapSize = width * height;
    char* map = malloc(mapSize);
    memset(map, '.', mapSize);
    for (uint32_t i = 0; i < length; i++)
    {
        uint32_t xStart, xEnd, yStart, yEnd;
        if (inputOneIsX[i])
        {
            xStart = inputOne[i] - minX;
            xEnd = inputOne[i] - minX;
            yStart = inputTwoStart[i];
            yEnd = inputTwoStop[i];
        }
        else
        {
            xStart = inputTwoStart[i] - minX;
            xEnd = inputTwoStop[i] - minX;
            yStart = inputOne[i];
            yEnd = inputOne[i];
        }
        // printf("%d..%d, %d..%d\n", xStart, xEnd, yStart, yEnd);
        for (uint32_t y = yStart; y <= yEnd; y++)
        {
            for (uint32_t x = xStart; x <= xEnd; x++)
            {
                map[x + y * width] = '#';
            }
        }
    }

    _count = 0;
    uint32_t startX = 500 - minX;
    uint32_t startY = minY;
    map[startX] = '+';
    _map = map;
    _width = width;
    _height = height;
    Fall(startX, startY);
    // DrawMap(map, width, height);

    return _count;
}

int main()
{
    printf("Problem1: %d\n", CountReachable(testData, ARRAY_SIZE(testData)));
    // printf("Problem1: %d\n", CountReachable(inputData, ARRAY_SIZE(inputData))); // x: 392 to 676, y: 4 to 1785
    printf("Hello world\n");
    return 0;
}