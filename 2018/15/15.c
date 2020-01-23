#include "..\common.h"
#include "15input.h"

typedef enum _direction {
    invalid,
    up,
    left,
    right,
    down,
    nomovebecausenearenemy,
} direction;

typedef struct _player {
    uint32_t power;
    int32_t hitPoints;
    uint32_t x;
    uint32_t y;
    char type;
    direction mostRecentMove;
    bool attacked;
} player;

const uint32_t MAP_Y = ARRAY_SIZE(map);
uint32_t MAP_X = 0;
uint32_t MAP_SIZE = 0;
char* mapNow;
uint8_t* mapGoal;
uint32_t rounds = 0;

player* players = NULL;
uint32_t playerCount = 0;

// Returns  -1 if a < b
//           0 if a == b
//          +1 if a > b
int ComparePlayers(const void* a, const void* b)
{
    const player* A = a;
    const player* B = b;

    // First check if players are dead
    if (A->hitPoints > 0 && B->hitPoints <= 0)
    {
        // printf("B is dead\n");
        return -1;
    }
    else if (A->hitPoints <= 0 && B->hitPoints > 0)
    {
        // printf("A is dead\n");
        return 1;
    }
    else if (A->hitPoints <= 0 && B->hitPoints <= 0)
    {
        // printf("Both are dead\n");
        return 0;
    }

    // Now return reading order based
    if (A->y < B->y)
    {
        // printf("%d < %d\n", A->y, B->y);
        return -1;
    }
    else if (A->y > B->y)
    {
        // printf("%d > %d\n", A->y, B->y);
        return 1;
    }
    else if (A->x < B->x)
    {
        return -1;
    }
    else if (A->x > B->x)
    {
        return 1;
    }

    assert(false);
}

player* GetPlayer(uint32_t x, uint32_t y)
{
    for (uint32_t i = 0; i < playerCount; i++)
    {
        if (players[i].x == x && players[i].y == y)
        {
            return &players[i];
        }
    }

    return NULL;
}

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
    printf("%d\n", rounds);
    player* drawMe[10];
    for (uint32_t y = 0; y < MAP_Y; y++)
    {
        uint32_t i = 0;
        for (uint32_t x = 0; x < MAP_X; x++)
        {
            char c = mapNow[y * MAP_X + x];
            printf("%c", c);
            if ('E' == c || 'G' == c)
            {
                drawMe[i++] = GetPlayer(x, y);
            }
        }

        // Draw hit points
        if (i > 0)
        {
            printf("    ");
            for (uint32_t p = 0; p < i; p++)
            {
                printf(" %d", drawMe[p]->hitPoints);
            }
        }

        // Draw move
        if (i > 0)
        {
            for (uint32_t p = 0; p < i; p++)
            {
                printf(" %d", drawMe[p]->mostRecentMove);
            }
        }

        // Draw attack
        if (i > 0)
        {
            for (uint32_t p = 0; p < i; p++)
            {
                printf(" %c", drawMe[p]->attacked ? 'a' : 'p');
            }
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

uint32_t goalMaxTry = 25;
char enemyRightNow;
void UpdateGoalMap(uint32_t x, uint32_t y, uint8_t count)
{
    // Can only move if this is a space or this is the first square
    assert(count < UINT8_MAX);
    char thisSpot = mapNow[y * MAP_Y + x];
    if (count == 0)
    {
        // assert(thisSpot == enemyRightNow);
        if (thisSpot != enemyRightNow)
        {
            printf("!!!!!!!!!!!!!\n");
        }
    }
    if (thisSpot != '.' && count != 0)
    {
        if (mapGoal[y * MAP_Y + x] == 0)
        {
            if (mapNow[y * MAP_Y + x] == '#')
            {
                mapGoal[y * MAP_Y + x] = 99;
            }
            else if (mapNow[y * MAP_Y + x] == 'G')
            {
                mapGoal[y * MAP_Y + x] = 98;
            }
            else if (mapNow[y * MAP_Y + x] == 'E')
            {
                mapGoal[y * MAP_Y + x] = 97;
            }
        }

        return;
    }

    if (count > goalMaxTry)
    {
        return;
    }

    // Mark this value if it's less and continue looking
    uint8_t currentCount = mapGoal[y * MAP_Y + x];
    if (count <= currentCount || currentCount == 0)
    {
        if (count != 0)
        {
            mapGoal[y * MAP_Y + x] = count;
        }
        else
        {
            mapGoal[y * MAP_Y + x] = 96;
        }

        // Move up, left, right, down
        count++;
        UpdateGoalMap(x + 0, y - 1, count); // Up
        UpdateGoalMap(x - 1, y + 0, count); // Left
        UpdateGoalMap(x + 1, y + 0, count); // Right
        UpdateGoalMap(x + 0, y + 1, count); // Down
    }
}

// Returns true if enemy was killed
bool Attack(player* p)
{
    // Are we dead?
    if (p->hitPoints <= 0)
    {
        return false;
    }

    p->attacked = false;

    // Find who to attack, one with lowest HP
    char enemy = p->type == 'E' ? 'G' : 'E';
    uint8_t hp = UINT8_MAX;
    player* enemyToAttack = NULL;
    if (mapNow[(p->y - 1) * MAP_X + (p->x + 0)] == enemy)
    {
        player* e = GetPlayer(p->x + 0, p->y - 1);
        if (e->hitPoints < hp)
        {
            hp = e->hitPoints;
            enemyToAttack = e;
        }
    }
    if (mapNow[(p->y + 0) * MAP_X + (p->x - 1)] == enemy)
    {
        player* e = GetPlayer(p->x - 1, p->y + 0);
        if (e->hitPoints < hp)
        {
            hp = e->hitPoints;
            enemyToAttack = e;
        }
    }
    if (mapNow[(p->y + 0) * MAP_X + (p->x + 1)] == enemy)
    {
        player* e = GetPlayer(p->x + 1, p->y + 0);
        if (e->hitPoints < hp)
        {
            hp = e->hitPoints;
            enemyToAttack = e;
        }
    }
    if (mapNow[(p->y + 1) * MAP_X + (p->x + 0)] == enemy)
    {
        player* e = GetPlayer(p->x + 0, p->y + 1);
        if (e->hitPoints < hp)
        {
            hp = e->hitPoints;
            enemyToAttack = e;
        }
    }

    // Now attack the enemy!
    if (enemyToAttack != NULL)
    {
        enemyToAttack->hitPoints -= 3;
        p->attacked = true;
        if (enemyToAttack->hitPoints <= 0)
        {
            mapNow[enemyToAttack->y * MAP_X + enemyToAttack->x] = '.';
            goalMaxTry = 50;
            return true;
        }
    }

    return false;
}

void MovePlayer(player* p)
{
    // Are we dead?
    if (p->hitPoints <= 0)
    {
        return;
    }

    // Is this player next to an enemy?
    char enemy = p->type == 'E' ? 'G' : 'E';
    if ((mapNow[(p->y + 0) * MAP_X + (p->x + 1)] == enemy) ||
        (mapNow[(p->y + 0) * MAP_X + (p->x - 1)] == enemy) ||
        (mapNow[(p->y + 1) * MAP_X + (p->x + 0)] == enemy) ||
        (mapNow[(p->y - 1) * MAP_X + (p->x + 0)] == enemy))
    {
        // printf("Attack1\n");
        p->mostRecentMove = nomovebecausenearenemy;
        return;
    }

    // Find path to each enemy
    memset(mapGoal, 0, MAP_SIZE);
    for (uint32_t i = 0; i < playerCount; i++)
    {
        if (players[i].type == enemy && players[i].hitPoints > 0)
        {
            // printf("?");
            // printf("Enemy at %d,%d\n", players[i].x, players[i].y);
            enemyRightNow = enemy;
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

    p->mostRecentMove = d;
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
    while (elfCount > 0 && goblinCount > 0)
    // for (uint32_t aaa = 0; aaa < 4; aaa++)
    {
        // printf(".");
        DrawMap();

        // Move players
        bool quitEarly = false;
        for (uint32_t i = 0; i < playerCount; i++)
        {
            if (elfCount == 0 || goblinCount == 0)
            {
                quitEarly = true;
                break;
            }

            // printf("!");
            MovePlayer(&players[i]);

            if (rounds == 23 && players[i].mostRecentMove == up)
            {
                DrawGoal();
            }

            // ATTACK
            // printf("x");
            if (Attack(&players[i]))
            {
                // The enemy died!
                if (players[i].type == 'E')
                {
                    goblinCount--;
                }
                else
                {
                    elfCount--;
                }
            }
        }

        if (quitEarly)
        {
            break;
        }

        // Sort!
        qsort(players, playerCount, sizeof(player), ComparePlayers);
        playerCount = goblinCount + elfCount;

        rounds++;
    }

    printf("After %d rounds\n", rounds);
    DrawMap();
    uint32_t sumHp = 0;
    for (uint32_t i = 0; i < playerCount; i++)
    {
        if (players[i].hitPoints > 0)
        {
            printf("HP: %d\n", players[i].hitPoints);
            sumHp += players[i].hitPoints;
        }
    }
    printf("Outcome: %d * %d = %d\n", rounds, sumHp, rounds * sumHp);
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