#include "..\common.h"
#include "08input.h"

uint32_t SumMetadata2(uint8_t x[], uint32_t length)
{
    // First, figure out how many children and what the metadata are for root node
    uint8_t rootChildCount = x[0];
    uint8_t rootMetadataCount = x[1];

    // Now get the metadata for the rootnode, it's at the way end
    uint8_t* rootMetadata = malloc(rootMetadataCount);
    for (uint32_t j = 0; j < rootMetadataCount; j++)
    {
        rootMetadata[j] = x[length - 1 - j];
    }

    // Now build a list for child metadata sums
    uint8_t* rootChildMetadataSums = malloc(rootChildCount);
    memset(rootChildMetadataSums, 0, rootChildCount);
    uint32_t rootChildMetadataSumsI = 0;

    // Walk the tree as an array
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
            // printf("Node %c children %d metadataCount %d\n", nodeI + 'A', children, metadataCount);
            nodeI++;
            // Save work to do to the stack
            stackChildTodo[stackI] = children == 0 ? 0 : children - 1;
            stackMetadaCount[stackI] = metadataCount;
            stackI++;

            if (children == 0)
            {
                // Do work now
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
                // printf("Reading %d now from stack work\n", stackMetadaCount[stackI]);
                uint32_t tempSum = 0;
                for (uint32_t m = 0; m < stackMetadaCount[stackI]; m++)
                {
                    tempSum += x[i++];
                }
                sum += tempSum;
                processStack = true;

                if (stackI == 1)
                {
                    // We just read a root node child's sum, cache it
                    printf("Found sum %d\n", tempSum);
                    rootChildMetadataSums[rootChildMetadataSumsI++] = tempSum;
                }
            }
        }
    }

    // printf("stack %d\n", stackI);
    printf("Sum was %d\n", sum);
    printf("rootChildMetadataSumsI %d\n", rootChildMetadataSumsI);

    // Now find sums of desired children
    uint32_t sum2 = 0;
    for (uint32_t j = 0; j < rootMetadataCount; j++)
    {
        uint32_t nodeI = rootMetadata[j];
        if (nodeI <= rootChildCount)
        {
            printf("using sum %d\n", rootChildMetadataSums[nodeI-1]);
            sum2 += rootChildMetadataSums[nodeI-1];
        }
    }
    
    return sum2;
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
    uint8_t testData[] = {2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2};
    // uint8_t testData[] = {2,3,1,3,0,1,1,10,11,12,1,1,0,1,99,2,1,1,2};
    printf("Sum of metadata %d", SumMetadata2(testData, ARRAY_SIZE(testData)));
    // printf("Sum of metadata %d", SumMetadata2(input, ARRAY_SIZE(input)));
    return 0;
}