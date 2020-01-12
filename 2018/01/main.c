#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

#include "input.h"

#define ARRAY_SIZE(x) (sizeof(x)/sizeof(x[0]))

void PartA()
{
    int32_t sum = 0;
    for (uint32_t i = 0; i < ARRAY_SIZE(input); i++)
    {
        sum += input[i];
    }

    printf("%d\n", sum);
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

void PartB()
{
    int32_t sum = 0;
    uint32_t i = 0;
    while (true)
    {
        sum += input[i];

        i++;
        if (i == ARRAY_SIZE(input))
        {
            printf("loop\n");
            i = 0;
        }
    }


}

int main(int argc, char* argv[])
{
    // PartA();
    PartB();
    return 0;
}