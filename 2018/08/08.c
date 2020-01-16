#include "..\common.h"
#include "08input.h"

uint32_t SumMetadata2(uint8_t x[], uint32_t length)
{
    uint8_t* stackChildTodo = malloc(length);
    uint8_t* stackMetadaCount = malloc(length);
    uint32_t stackI = 0;
    uint32_t sum = 0;
    uint32_t i = 0;
    bool processStack = false;
    uint32_t nodeI = 0;
    while (i < length)
    {
        if (!processStack)
        {
            uint8_t children = x[i++];
            uint8_t metadataCount = x[i++];
            printf("Node %c children %d metadataCount %d\n", nodeI + 'A', children, metadataCount);
            nodeI++;
            if (children > 0)
            {
                // Save work to do and look for another child
                stackChildTodo[stackI] = children - 1;
                stackMetadaCount[stackI] = metadataCount;
                stackI++;
            }
            else
            {
                // Get metadata now and then process work on the stack
                printf("Reading %d immediately\n", metadataCount);
                for (uint32_t m = 0; m < metadataCount; m++)
                {
                    sum += x[i++];
                }
                processStack = true;
            }
        }
        else
        {
            processStack = false;
            if (stackChildTodo[stackI - 1] > 0)
            {
                // Look for another child now
                stackChildTodo[stackI - 1]--;
            }
            else
            {
                // Pop the stack and read the values now
                stackI--;
                printf("Reading %d now from stack work\n", stackMetadaCount[stackI]);
                for (uint32_t m = 0; m < stackMetadaCount[stackI]; m++)
                {
                    sum += x[i++];
                }
                processStack = true;
            }
        }
    }

    printf("stack %d\n", stackI);
    
    return sum;
}

int main(int argc, char* argv[])
{
    // 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    // A----------------------------------
    //     B----------- C-----------
    //                      D-----

    // 2 3 1 3 0 1 1 10 11 12 1 1 0 1 99 2 1 1 2
    // A----------------------------------------
    //     B----------------- C-----------
    //         x-----             D-----
    // uint8_t testData[] = {2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2};
    uint8_t testData[] = {2,3,1,3,0,1,1,10,11,12,1,1,0,1,99,2,1,1,2};
    // printf("Sum of metadata %d", SumMetadata2(testData, ARRAY_SIZE(testData)));
    printf("Sum of metadata %d", SumMetadata2(input, ARRAY_SIZE(input)));
    return 0;
}