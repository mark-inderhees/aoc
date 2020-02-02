#include "..\common.h"
#include "20input.h"

void BuildMap(char map[], uint32_t mapSide, char* input, uint32_t inputIndex, uint32_t x, uint32_t y)
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
            BuildMap(map, mapSide, input, inputIndex, x, y);
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
                            BuildMap(map, mapSide, input, inputIndex, x, y);
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
                        inputIndex++;
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
            map[x + y * mapSide] = '-';
            x = x + 1;
            map[x + y * mapSide] = '.';
        }
        else if (c == 'W')
        {
            x = x - 1;
            map[x + y * mapSide] = '-';
            x = x - 1;
            map[x + y * mapSide] = '.';
        }
        else
        {
            assert(false);
        }
    }
}

#define MAP_SIDE 1000
#define MAP_SIZE (MAP_SIDE * MAP_SIDE)
char _map[MAP_SIZE];
uint32_t Problem1(char* input)
{

    uint32_t mapSide = MAP_SIDE;
    uint32_t mapSize = MAP_SIZE;
    uint32_t x = mapSide / 2;
    uint32_t y = mapSide / 2;
    memset(_map, '#', mapSize);
}

int main()
{
    // printf("%s\n", input);
    printf("Hello world\n");
    return 0;
}