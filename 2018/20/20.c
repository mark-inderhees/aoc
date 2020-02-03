#include "..\common.h"
#include "20input.h"

typedef enum _direction {
    up,
    left,
    right,
    down
} direction;

typedef struct item item;
struct item {
    uint32_t index;
    item* pNext;
};

void UpdateGoalMap(char* map, uint32_t* mapGoal, uint32_t mapSide, uint32_t x, uint32_t y, uint32_t count, direction d)
{
    // assert(count < UINT32_MAX);
    if (d == up)
    {
        y--;
    }
    else if (d == left)
    {
        x--;
    }
    else if (d == right)
    {
        x++;
    }
    else if (d == down)
    {
        y++;
    }

    // Can only move if this is a door
    char thisSpot = map[x + y * mapSide];
    if (thisSpot != '|' && thisSpot != '-')
    {
        return;
    }

    // Move into the room
    if (d == up)
    {
        y--;
    }
    else if (d == left)
    {
        x--;
    }
    else if (d == right)
    {
        x++;
    }
    else if (d == down)
    {
        y++;
    }
    thisSpot = map[x + y * mapSide];
    assert(thisSpot == '.');

    // Mark this value if it's less and continue looking
    uint32_t currentCount = mapGoal[x + y * mapSide];
    if (count < currentCount || currentCount == 0)
    {
        mapGoal[x + y * mapSide] = count;

        // Move up, left, right, down
        count++;
        UpdateGoalMap(map, mapGoal, mapSide, x, y, count, up);
        UpdateGoalMap(map, mapGoal, mapSide, x, y, count, left);
        UpdateGoalMap(map, mapGoal, mapSide, x, y, count, right);
        UpdateGoalMap(map, mapGoal, mapSide, x, y, count, down);
    }
}

bool HaveWeBeenHere(item* mapHistory[], uint32_t mapSide, uint32_t index, uint32_t x, uint32_t y)
{
    item* pItem = mapHistory[x + y * mapSide];
    if (pItem == NULL)
    {
        pItem = malloc(sizeof(item));
        pItem->index = index;
        pItem->pNext = NULL;
        mapHistory[x + y * mapSide] = pItem;
        return false;
    }

    while (true)
    {
        if (pItem->index == index)
        {
            return true;
        }

        if (pItem->pNext == NULL)
        {
            pItem->pNext = malloc(sizeof(item));
            pItem->pNext->index = index;
            pItem->pNext->pNext = NULL;
            return false;
        }
        pItem = pItem->pNext;
    }

    return false;
}

void BuildMap(item* mapHistory[], char map[], uint32_t mapSide, char* input, uint32_t inputIndex, uint32_t x, uint32_t y,
    uint32_t* xMin, uint32_t* xMax, uint32_t* yMin, uint32_t* yMax)
{
    // Example map  "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"
    //              "^ENWWW(NEEE|SSE(EE|N))$";
    //              ^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$
    if (HaveWeBeenHere(mapHistory, mapSide, inputIndex, x, y))
    {
        // printf("We have been here\n");
        return;
    }

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
            BuildMap(mapHistory, map, mapSide, input, inputIndex, x, y, xMin, xMax, yMin, yMax);
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
                            BuildMap(mapHistory, map, mapSide, input, inputIndex, x, y, xMin, xMax, yMin, yMax);
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

void DrawMapGoal(uint32_t* map, uint32_t mapSide, uint32_t xMin, uint32_t xMax, uint32_t yMin, uint32_t yMax)
{
    for (uint32_t y = yMin; y <= yMax; y++)
    {
        for (uint32_t x = xMin; x <= xMax; x++)
        {
            printf("%03d,", map[x + y * mapSide]);
        }
        printf("\n");
    }
}

uint32_t FindLargestGoal(uint32_t* map, uint32_t mapSide, uint32_t xMin, uint32_t xMax, uint32_t yMin, uint32_t yMax)
{
    uint32_t goal = 0;
    for (uint32_t y = yMin; y <= yMax; y++)
    {
        for (uint32_t x = xMin; x <= xMax; x++)
        {
            uint32_t value = map[x + y * mapSide];
            if (value != UINT32_MAX && value > goal)
            {
                goal = value;
            }
        }
    }

    return goal;
}

uint32_t Problem1(char* input)
{
    uint32_t mapSide = 1000;
    uint32_t mapSize = mapSide * mapSide;
    char* map = malloc(mapSize);
    uint32_t* mapGoal = malloc(mapSize * sizeof(uint32_t));
    memset(mapGoal, 0, mapSize * sizeof(uint32_t));
    uint32_t x = mapSide / 2;
    uint32_t y = mapSide / 2;
    uint32_t xMin = x;
    uint32_t xMax = x;
    uint32_t yMin = y;
    uint32_t yMax = y;
    memset(map, '#', mapSize);
    map[x + y * mapSide] = '.';
    item** mapHistory = malloc(sizeof(item*) * mapSize);
    memset(mapHistory, 0, sizeof(item*) * mapSize);
    BuildMap(mapHistory, map, mapSide, input, 1, x, y, &xMin, &xMax, &yMin, &yMax);
    xMin--;
    xMax++;
    yMin--;
    yMax++;
    // DrawMap(map, mapSide, xMin, xMax, yMin, yMax);

    UpdateGoalMap(map, mapGoal, mapSide, x, y, 1, up);
    UpdateGoalMap(map, mapGoal, mapSide, x, y, 1, left);
    UpdateGoalMap(map, mapGoal, mapSide, x, y, 1, right);
    UpdateGoalMap(map, mapGoal, mapSide, x, y, 1, down);

    // DrawMapGoal(mapGoal, mapSide, xMin, xMax, yMin, yMax);

    return FindLargestGoal(mapGoal, mapSide, xMin, xMax, yMin, yMax);
}

int main()
{
    // printf("%s\n", input);
    printf("Result: %d\n", Problem1(testData1));
    printf("Result: %d\n", Problem1(testData2));
    printf("Result: %d\n", Problem1(testData3));
    printf("Result: %d\n", Problem1(testData4));
    printf("Result: %d\n", Problem1(inputData));
    printf("Hello world\n");
    return 0;
}