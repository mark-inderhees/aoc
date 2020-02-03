#include "..\common.h"
#include "20input.h"

void BuildMap(char map[], uint32_t mapSide, char* input, uint32_t inputIndex, uint32_t x, uint32_t y,
    uint32_t* xMin, uint32_t* xMax, uint32_t* yMin, uint32_t* yMax)
{
    // Example map  "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"
    //              "^ENWWW(NEEE|SSE(EE|N))$";
    //              ^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$
    while (true)
    {
        if (x <= 2 || x >= mapSide - 2 || y <= 2 || y >= mapSide - 2)
        {
            assert(false);
        }

        if (x < *xMin)
        {
            *xMin = x;
        }
        if (x > *xMax)
        {
            *xMax = x;
        }
        if (y < *yMin)
        {
            *yMin = y;
        }
        if (y > *yMax)
        {
            *yMax = y;
        }

        char c = input[inputIndex];
        if (c == '$')
        {
            // Done
            return;
        }
        else if (c == '(')
        {
            // Need to fork and create recursive calls
            inputIndex++;
            BuildMap(map, mapSide, input, inputIndex, x, y, xMin, xMax, yMin, yMax);
            uint32_t levelCount = 0;
            while (true)
            {
                c = input[inputIndex];
                if (levelCount == 0)
                {
                    if (c == '|')
                    {
                        inputIndex++;
                        c = input[inputIndex];
                        if (c == ')')
                        {
                            // This is empty split, continue where we were
                            break;
                        }
                        else if (c == 'N' || c == 'S' || c == 'E' || c == 'W')
                        {
                            // Creat new fork
                            BuildMap(map, mapSide, input, inputIndex, x, y, xMin, xMax, yMin, yMax);
                            continue;
                        }
                        else
                        {
                            assert(false);
                        }
                    }
                    else if (c == ')')
                    {
                        // Splits are done, kill this fork
                        return;
                    }
                }

                if (c == '(')
                {
                    levelCount++;
                }
                else if (c == ')')
                {
                    levelCount--;
                }
                else if (c == 'N' || c == 'S' || c == 'E' || c == 'W' || c == '|')
                {
                    // no op
                }
                else
                {
                    assert(false);
                }

                inputIndex++;
            }
        }
        else if (c == '|')
        {
            // We were a fork, skip over these forks
            inputIndex++;
            uint32_t levelCount = 0;
            while (true)
            {
                c = input[inputIndex];
                if (levelCount == 0)
                {
                    if (c == ')')
                    {
                        // Forks are done, continue work
                        break;
                    }
                }

                if (c == '(')
                {
                    levelCount++;
                }
                else if (c == ')')
                {
                    levelCount--;
                }
                else if (c == 'N' || c == 'S' || c == 'E' || c == 'W' || c == '|')
                {
                    // no op
                }
                else
                {
                    assert(false);
                }

                inputIndex++;
            }
        }
        else if (c == 'N')
        {
            y = y - 1;
            map[x + y * mapSide] = '-';
            y = y - 1;
            map[x + y * mapSide] = '.';
        }
        else if (c == 'S')
        {
            y = y + 1;
            map[x + y * mapSide] = '-';
            y = y + 1;
            map[x + y * mapSide] = '.';
        }
        else if (c == 'E')
        {
            x = x + 1;
            map[x + y * mapSide] = '|';
            x = x + 1;
            map[x + y * mapSide] = '.';
        }
        else if (c == 'W')
        {
            x = x - 1;
            map[x + y * mapSide] = '|';
            x = x - 1;
            map[x + y * mapSide] = '.';
        }
        else if (c == ')')
        {
            // We were a fork, done with fork. But keep going with normal logic.
        }
        else
        {
            assert(false);
        }

        inputIndex++;
    }
}

void DrawMap(char* map, uint32_t mapSide, uint32_t xMin, uint32_t xMax, uint32_t yMin, uint32_t yMax)
{
    for (uint32_t y = yMin; y <= yMax; y++)
    {
        for (uint32_t x = xMin; x <= xMax; x++)
        {
            printf("%c", map[x + y * mapSide]);
        }
        printf("\n");
    }
}


uint32_t Problem1(char* input)
{
    uint32_t mapSide = 1000;
    uint32_t mapSize = mapSide * mapSide;
    char* map = malloc(mapSize);
    uint32_t x = mapSide / 2;
    uint32_t y = mapSide / 2;
    uint32_t xMin = x;
    uint32_t xMax = x;
    uint32_t yMin = y;
    uint32_t yMax = y;
    memset(map, '#', mapSize);
    map[x + y * mapSide] = '.';
    BuildMap(map, mapSide, input, 1, x, y, &xMin, &xMax, &yMin, &yMax);
    xMin--;
    xMax++;
    yMin--;
    yMax++;
    DrawMap(map, mapSide, xMin, xMax, yMin, yMax);

    return 0;
}

int main()
{
    // printf("%s\n", input);
    printf("Result: %d\n", Problem1(testData1));
    printf("Hello world\n");
    return 0;
}