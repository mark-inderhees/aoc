#include "..\common.h"
#include "17input.h"

typedef enum _workType {
    fall,
    flow,
    fill,
} workType;

typedef struct _work {
    workType type;
    uint32_t x;
    uint32_t y;
} work;

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

    // Print X label
    printf("     ");
    for (uint32_t x = 0; x < width; x++)
    {
        printf("%01d", x / 100);
    }
    printf("\n");
    printf("     ");
    for (uint32_t x = 0; x < width; x++)
    {
        printf("%01d", (x % 100) / 10);
    }
    printf("\n");
    printf("     ");
    for (uint32_t x = 0; x < width; x++)
    {
        printf("%01d", x % 10);
    }
    printf("\n");

    for (uint32_t y = 0; y < height; y++)
    {
        printf("%04d ", y);
        for (uint32_t x = 0; x < width; x++)
        {
            printf("%c", map[x + y * width]);
        }
        printf("\n");
    }
}

// Returns true if can fall, returns false if hit a wall
bool Flow(char* map, uint32_t width, uint32_t* pCount, uint32_t x, uint32_t y, int32_t offset, uint32_t* pXOut)
{
    char c;
    // printf("Flowing row %d from x %d in direction %d\n", y, x, offset);
    do
    {
        c = map[x + y * width];
        if (c == '.')
        {
            map[x + y * width] = '|';
            *pCount += 1;
        }

        // Can we fall?
        c = map[x + (y + 1) * width];
        if (c == '.' || c == '|')
        {
            // Can fall
            *pXOut = x;
            // printf("Need to fall from x=%d\n", x);
            return true;
        }

        // Can we flow more?
        x = x + offset;
        c = map[x + y * width];
    } while (c == '.' || c == '|');

    // Hit a wall
    return false;
}

void Fill(char* map, uint32_t width, uint32_t x, uint32_t y, int32_t offset)
{
    char c;
    do
    {
        map[x + y * width] = '~';

        // Can we flow more?
        x = x + offset;
        c = map[x + y * width];
    } while (c == '.' || c == '|');

    // printf(" until %d", x - offset);
}

// Returns true if can flow, returns fall if fell off map
bool Fall(char* map, uint32_t width, uint32_t height, uint32_t* pCount, uint32_t x, uint32_t y, uint32_t* pYOut)
{
    // DrawMap(_map, _width, _height);
    // printf("Falling from %d, %d\n", x, y);
    // Flow down as long as possible
    char c;
    while (true)
    {
        c = map[x + y * width];
        if (c == '.')
        {
            assert(c == '.');
            map[x + y * width] = '|';
            *pCount += 1;
        }

        if (y + 1 >= height)
        {
            // Fell off map!
            // printf("Fell off map at %d, %d\n", x, y);
            return false;
        }

        // Check if can fall
        c = map[x + (y + 1) * width];
        if (c != '.' && c != '|')
        {
            // Cannot fall, can now flow
            *pYOut = y;
            return true;
        }

        if (c == '|')
        {
            // We have already fallen here, do nothing
            return false;
        }

        y++;
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

    // DrawMap(map, width, height);

    uint32_t count = 0;
    uint32_t startX = 500 - minX;
    uint32_t startY = minY;
    map[startX] = '+';
    #define MAX_STACK 10000
    uint32_t stackI = 0;
    work* stack = malloc(sizeof(work) * MAX_STACK);
    work* pNewWork = &stack[stackI++];
    pNewWork->type = fall;
    pNewWork->x = startX;
    pNewWork->y = startY;
    // printf("Fall from %d, %d\n", pNewWork->x, pNewWork->y);
    while (stackI > 0)
    {
        assert(stackI < MAX_STACK);
        // printf("Stack count: %d\n", stackI);

        // DrawMap(map, width, height);

        // Pop work
        work work = stack[--stackI];
        assert(work.x > 0);
        assert(work.y > 0);
        assert(work.x <= width);
        assert(work.y <= maxY);
        if (work.type == fall)
        {
            uint32_t newY;
            bool canFlow = Fall(map, width, height, &count, work.x, work.y, &newY);
            if (canFlow)
            {
                // Can flow, add new work
                pNewWork = &stack[stackI++];
                pNewWork->type = flow;
                pNewWork->x = work.x;
                pNewWork->y = newY;
                assert(pNewWork->y <= maxY);
                // printf("Flow from %d, %d\n", pNewWork->x, pNewWork->y);
            }
        }
        else if (work.type == flow)
        {
            uint32_t newX;

            // Flow left
            bool fallLeft = Flow(map, width, &count, work.x, work.y, -1, &newX);
            if (fallLeft)
            {
                // Can fall, add work
                pNewWork = &stack[stackI++];
                pNewWork->type = fall;
                pNewWork->x = newX;
                pNewWork->y = work.y;
                assert(newX <= width);
                // printf("Finished flow left, fall from %d, %d\n", pNewWork->x, pNewWork->y);
            }

            bool fallRight = Flow(map, width, &count, work.x, work.y, 1, &newX);
            if (fallRight)
            {
                // Can fall, add work
                pNewWork = &stack[stackI++];
                pNewWork->type = fall;
                pNewWork->x = newX;
                pNewWork->y = work.y;
                assert(newX <= width);
                // printf("Finished flow right, fall from %d, %d\n", pNewWork->x, pNewWork->y);
            }

            if (!fallLeft && !fallRight)
            {
                // Need to fill this row and flow one row higher
                pNewWork = &stack[stackI++];
                pNewWork->type = fill;
                pNewWork->x = work.x;
                pNewWork->y = work.y;
                // printf("Need to fill\n");
            }
        }
        else if (work.type == fill)
        {
            // Only fill this row if it has not already been filled
            if (map[work.x + work.y * width] != '~')
            {
                // Fill up this row, back up one and flow again
                // printf("Filling row %d", y);
                Fill(map, width, work.x, work.y, -1);
                Fill(map, width, work.x, work.y, +1);
                // printf("\n");
                pNewWork = &stack[stackI++];
                pNewWork->type = flow;
                pNewWork->x = work.x;
                pNewWork->y = work.y - 1;
                // assert(pNewWork->y <= maxY);
                // printf("Completed fill, need to reflow\n");
            }
        }
    }

    // DrawMap(map, width, height);

    return count;
}

int main()
{
    // printf("Problem1: %d\n", CountReachable(testData, ARRAY_SIZE(testData)));
    printf("Problem1: %d\n", CountReachable(inputData, ARRAY_SIZE(inputData))); // x: 392 to 676, y: 4 to 1785
    printf("Hello world\n");
    return 0;
}