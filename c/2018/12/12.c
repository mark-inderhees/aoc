#include "..\common.h"
#include "12input.h"

void SpreadPlants(char* input[], uint32_t length)
{
    // Get initial plan state
    const uint32_t bucketZeroI = 10;
    #define maxPlants 1000
    bool plants[maxPlants] = {0};
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
    bool plants2[maxPlants] = {0};
    bool* plantsNow = plants;
    bool* plantsNext = plants2;
    uint32_t previousSum = 0;
    for (uint32_t i = 0; i < 200; i++)
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
        for (uint32_t x = 2; x < maxPlants - 2; x++)
        {
            uint32_t id = 0;
            for (int32_t j = -2; j <= 2; j++)
            {
                id |= plantsNow[x + j] << (j + 2);
            }
            plantsNext[x] = lookup[id];
        }

        // Now calculate the value of the plants
        for (uint32_t m = i; m < i + 150; m++)
        {
            int32_t value = plantsNext[m] ? 1 : 0;
            printf("%d", value);
        }
        int32_t sum = 0;
        for (uint32_t m = 0; m < maxPlants; m++)
        {
            int32_t value = plantsNext[m] ? 1 : 0;
            sum += value * (m - bucketZeroI);
        }
        printf(" Sum: %d (diff %d)\n", sum, sum - previousSum);
        previousSum = sum;
    }

    // Now calculate the value of the plants
    int32_t sum = 0;
    for (uint32_t i = 0; i < maxPlants; i++)
    {
        int32_t value = plantsNext[i] ? 1 : 0;
        printf("%d", value);
        sum += value * (i - bucketZeroI);
    }
    printf("Sum: %d\n", sum);


    // This is generation 200. All future generations will shift right one:
    // 00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010110110110011011011011011011011011001101101000000000000000000001011011011010011011001101101101101101101000000010110110100000000000000000001011011011011011011011010000000101101101
    // Bucket zero is at index 10
    // Sum is 14775 at geneartion 200. Each generation, sum increases by 81
    uint64_t sum2 = sum;
    printf("Sum at 50 billion %lld\n", sum2 + (81ull * (50000000000ull - 200ull)));
}

int main(int argc, char* argv[])
{
    SpreadPlants(input, ARRAY_SIZE(input));
    return 0;
}