#include "..\common.h"
#include "15input.h"

typedef struct _player {
    uint32_t power;
    int32_t hitPoints;
    uint32_t x;
    uint32_t y;
    char type;
} player;

typedef enum _direction {
    invalid,
    up,
    left,
    right,
    down
} direction;

const uint32_t MAP_Y = ARRAY_SIZE(map);
uint32_t MAP_X = 0;
uint32_t MAP_SIZE = 0;
char* mapNow;
uint8_t* mapGoal;

player* players = NULL;
uint32_t playerCount = 0;

uint32_t CountInstance(char instance)
{
    uint32_t count = 0;
    for (uint32_t y = 0; y < MAP_Y; y++)
    {
        for (uint32_t x = 0; x < MAP_X; x++)
        {
            if (map[y][x] == instance)
            {
                count++;
            }
        }
    }

    return count;
}

void DrawMap()
{
    printf("----------MAP----------\n");
    for (uint32_t y = 0; y < MAP_Y; y++)
    {
        for (uint32_t x = 0; x < MAP_X; x++)
        {
            printf("%c", mapNow[y * MAP_X + x]);
        }
        printf("\n");
    }
}

void DrawGoal()
{
    printf("----------GOAL----------\n");
    for (uint32_t y = 0; y < MAP_Y; y++)
    {
        for (uint32_t x = 0; x < MAP_X; x++)
        {
            printf("%02d,", mapGoal[y * MAP_X + x]);
        }
        printf("\n");
    }
}

void InitPlayer(player* p, uint32_t x, uint32_t y, char type)
{
    p->power = 3;
    p->hitPoints = 200;
    p->x = x;
    p->y = y;
    p->type = type;
}

void UpdateGoalMap(uint32_t x, uint32_t y, uint8_t count)
{
    // Can only move if this is a space or this is the first square
    assert(count < 100);
    if (mapNow[y * MAP_Y + x] != '.' && count != 0)
    {
        return;
    }

    // Mark this value if it's less and continue looking
    uint8_t currentCount = mapGoal[y * MAP_Y + x];
    if (count <= currentCount || currentCount == 0)
    {
        mapGoal[y * MAP_Y + x] = count;

        // Move up, left, right, down
        count++;
        UpdateGoalMap(x + 0, y - 1, count); // Up
        UpdateGoalMap(x - 1, y + 0, count); // Left
        UpdateGoalMap(x + 1, y + 0, count); // Right
        UpdateGoalMap(x + 0, y + 1, count); // Down
    }
}

// Returns true if next to enemy
bool MovePlayer(player* p)
{
    // Is this player next to an enemy?
    char enemy = p->type == 'E' ? 'G' : 'E';
    if ((mapNow[(p->y + 0) * MAP_X + (p->x + 1)] == enemy) ||
        (mapNow[(p->y + 0) * MAP_X + (p->x - 1)] == enemy) ||
        (mapNow[(p->y + 1) * MAP_X + (p->x + 0)] == enemy) ||
        (mapNow[(p->y - 1) * MAP_X + (p->x + 0)] == enemy))
    {
        // printf("Attack1\n");
        return true;
    }

    // Find path to each enemy
    memset(mapGoal, 0, MAP_SIZE);
    for (uint32_t i = 0; i < playerCount; i++)
    {
        if (players[i].type == enemy)
        {
            // printf("Enemy at %d,%d\n", players[i].x, players[i].y);
            UpdateGoalMap(players[i].x, players[i].y, 0);
        }
    }

    // DrawMap();
    // DrawGoal();

    // Find lowest goal value and desired direction
    uint8_t goal = UINT8_MAX;
    uint8_t testValue = 0;
    direction d = invalid;
    testValue = mapGoal[(p->y - 1) * MAP_X + (p->x + 0)];
    if (testValue < goal && testValue != 0)
    {
        goal = testValue;
        d = up;
    }
    testValue = mapGoal[(p->y + 0) * MAP_X + (p->x - 1)];
    if (testValue < goal && testValue != 0)
    {
        goal = testValue;
        d = left;
    }
    testValue = mapGoal[(p->y + 0) * MAP_X + (p->x + 1)];
    if (testValue < goal && testValue != 0)
    {
        goal = testValue;
        d = right;
    }
    testValue = mapGoal[(p->y + 1) * MAP_X + (p->x + 0)];
    if (testValue < goal && testValue != 0)
    {
        goal = testValue;
        d = down;
    }

    // Now do the move
    if (d == up)
    {
        mapNow[p->y * MAP_X + p->x] = '.';
        p->y = p->y - 1;
        mapNow[p->y * MAP_X + p->x] = p->type;
    }
    else if (d == left)
    {
        mapNow[p->y * MAP_X + p->x] = '.';
        p->x = p->x - 1;
        mapNow[p->y * MAP_X + p->x] = p->type;
    }
    else if (d == right)
    {
        mapNow[p->y * MAP_X + p->x] = '.';
        p->x = p->x + 1;
        mapNow[p->y * MAP_X + p->x] = p->type;
    }
    else if (d == down)
    {
        mapNow[p->y * MAP_X + p->x] = '.';
        p->y = p->y + 1;
        mapNow[p->y * MAP_X + p->x] = p->type;
    }
    else
    {
        // printf("NO MOVE\n");
    }
    

    // Is this player next to an enemy?
    if ((mapNow[(p->y + 0) * MAP_X + (p->x + 1)] == enemy) ||
        (mapNow[(p->y + 0) * MAP_X + (p->x - 1)] == enemy) ||
        (mapNow[(p->y + 1) * MAP_X + (p->x + 0)] == enemy) ||
        (mapNow[(p->y - 1) * MAP_X + (p->x + 0)] == enemy))
    {
        // printf("Attack2\n");
        return true;
    }

    return false;
}

void DoBattle()
{
    // Init elfs and goblins, they will be sorted initially
    uint32_t elfCount = CountInstance('E');
    uint32_t goblinCount = CountInstance('G');
    playerCount = elfCount + goblinCount;
    printf("%d elfs vs %d goblins\n", elfCount, goblinCount);
    players = malloc(sizeof(player) * playerCount);
    uint32_t playerI = 0;
    for (uint32_t y = 0; y < MAP_Y; y++)
    {
        for (uint32_t x = 0; x < MAP_X; x++)
        {
            mapNow[y * MAP_X + x] = map[y][x];
            if (map[y][x] == 'E')
            {
                InitPlayer(&players[playerI++], x, y, 'E');
            }
            else if (map[y][x] == 'G')
            {
                InitPlayer(&players[playerI++], x, y, 'G');
            }
        }
    }

    // Do battle!
    // while (elfCount > 0 && goblinCount > 0)
    for (uint32_t aaa = 0; aaa < 4; aaa++)
    {
        DrawMap();
        // Move players
        for (uint32_t i = 0; i < playerCount; i++)
        {
            MovePlayer(&players[i]);
        }
    }
}

int main()
{
    printf("Hello world\n");

    MAP_X = strlen(map[0]);
    MAP_SIZE = MAP_X * MAP_Y;
    mapNow = malloc(MAP_SIZE);
    mapGoal = malloc(MAP_SIZE);
    DoBattle();
    return 0;
}