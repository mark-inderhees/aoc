#include <stdio.h>
#include <stdint.h>
#include "input.h"

#define ARRAY_SIZE(x) (sizeof(x)/sizeof(x[0]))

int main(int argc, char* argv[])
{
    int32_t sum = 0;
    for (uint32_t i = 0; i < ARRAY_SIZE(input); i++)
    {
        sum += input[i];
    }

    printf("%d\n", sum);
    return 0;
}