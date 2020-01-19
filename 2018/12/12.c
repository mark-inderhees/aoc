#include "..\common.h"
#include "12input.h"

void SpreadPlants(char* input[], uint32_t length)
{
    // Get initial plan state
    const uint32_t bucketZeroI = 100;
    bool plants[300] = {0};
    uint32_t bucketCount = strlen(&input[0][15]);
    for (uint32_t i = 0; i < bucketCount; i++)
    {
        plants[bucketZeroI + i] = input[0][15 + i] == '#';
        // printf("%d", plants[bucketZeroI + i]);
    }

    // Get lookup table values, 5 bits --> 2^5 = 32 lookups
    bool lookup[32] = {0};
    assert(length == 2 + 32);
    for (uint32_t i = 2; i < length; i++)
    {
        uint32_t id = 0;
        for (uint32_t j = 0; j < 5; j++)
        {
            uint32_t temp = input[i][j] == '#' ? 1 : 0;
            id |= temp << j;
        }
        lookup[id] = input[i][9] == '#';
    }

    // Now do 20 generations! BUT don't change values until end of generation, need flip flop
    bool plants2[300] = {0};
    bool* plantsNow = plants;
    bool* plantsNext = plants2;
    for (uint32_t i = 0; i < 20; i++)
    {
        if (i % 2 == 0)
        {
            plantsNow =  plants;
            plantsNext = plants2;
        }
        else
        {
            plantsNow =  plants2;
            plantsNext = plants;
        }

        // Loop through each plant and lookup its next value
        for (uint32_t x = 2; x < 300 - 2; x++)
        {
            uint32_t id = 0;
            for (int32_t j = -2; j <= 2; j++)
            {
                id |= plantsNow[x + j] << (j + 2);
            }
            plantsNext[x] = lookup[id];
        }
    }

    // Now calculate the value of the plants
    int32_t sum = 0;
    for (uint32_t i = 0; i < 300; i++)
    {
        int32_t value = plantsNext[i] ? 1 : 0;
        sum += value * (i - bucketZeroI);
    }
    printf("Sum: %d\n", sum);
}

int main(int argc, char* argv[])
{
    SpreadPlants(input, ARRAY_SIZE(input));
    return 0;
}