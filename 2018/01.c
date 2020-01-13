#include "common.h"

int32_t SumOfArray(int32_t x[], uint32_t length)
{
    int32_t sum = 0;
    for (uint32_t i = 0; i < length; i++)
    {
        sum += x[i];
    }

    return sum;
}

int32_t values[10000];
uint32_t valueCount = 0;
bool FindValue(int32_t sum)
{
    uint32_t i = valueCount / 2;
    uint32_t min = 0;
    uint32_t max = valueCount - 1;
    while (true)
    {
        int32_t value = values[i];
        if (value == sum)
        {
            return true;
        }

        if (value > sum)
        {
            // look lower
            max = i - 1;
        }
        else
        {
            // look higher
            min = i + 1;
        }

    }
}

int32_t FindFirstRepeatSum(int32_t x[], uint32_t length)
{
    int32_t sum = 0;
    uint32_t i = 0;
    while (true)
    {
        sum += x[i];

        i++;
        if (i == length)
        {
            printf("loop\n");
            i = 0;
        }
    }
}