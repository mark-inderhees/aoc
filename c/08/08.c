#include "..\common.h"
#include "08input.h"

typedef struct node node;
struct node {
    node* pParent;
    uint8_t childrenCount;
    node** pChildren;
    uint8_t childI;
    uint8_t childI2;
    uint8_t dataCount;
    uint8_t* data;
    uint32_t dataSum;
    uint32_t funkySum;
    uint32_t id;
};

void CreateNode(node* pParent, uint8_t childrenCount, uint32_t dataCount)
{
    pParent->childrenCount = childrenCount;
    if (childrenCount > 0)
    {
        pParent->pChildren = malloc(sizeof(node*) * childrenCount);
        for (uint32_t i = 0; i < childrenCount; i++)
        {
            pParent->pChildren[i] = malloc(sizeof(node));
            memset(pParent->pChildren[i], 0, sizeof(node));
            pParent->pChildren[i]->pParent = pParent;
        }
    }
    pParent->dataCount = dataCount;
    pParent->data = malloc(dataCount);
}

void BuildATree(uint8_t x[], uint32_t length)
{
    node root = {0};
    node* pNode = &root;
    uint32_t i = 0;
    bool createNode = true;
    uint32_t sum = 0;
    uint32_t id = 0;
    while (i < length)
    {
        if (createNode)
        {
            uint8_t childrenCount = x[i++];
            uint8_t dataCount = x[i++];
            CreateNode(pNode, childrenCount, dataCount);
            pNode->id = id++;
            createNode = false;
        }

        if (pNode->childI < pNode->childrenCount)
        {
            // There are more children to read in
            pNode = pNode->pChildren[pNode->childI++];
            createNode = true;
        }
        else
        {
            // Read data
            for (uint32_t m = 0; m < pNode->dataCount; m++)
            {
                pNode->data[m] = x[i++];
                pNode->dataSum += pNode->data[m];
            }
            sum += pNode->dataSum;

            pNode = pNode->pParent;
        }
    }

    printf("Add data sum %d\n", sum);

    // Find the funky sum, walk the tree depth first
    pNode = &root;
    while (pNode != NULL)
    {
        if (pNode->childI2 < pNode->childrenCount)
        {
            // Go to children
            pNode = pNode->pChildren[pNode->childI2++];
        }
        else if (pNode->childrenCount == 0)
        {
            // Leaf node, use sum as funky sum
            pNode->funkySum = pNode->dataSum;
            // printf("Got Funky sum for leaf id %d = %d\n", pNode->id, pNode->funkySum);
            pNode = pNode->pParent;
        }
        else
        {
            // Visited all the children, find the funky sum
            for (uint32_t m = 0; m < pNode->dataCount; m++)
            {
                if (pNode->data[m] <= pNode->childrenCount)
                {
                    pNode->funkySum += pNode->pChildren[pNode->data[m] - 1]->funkySum;
                }
            }
            // printf("Got Funky sum for id %d = %d\n", pNode->id, pNode->funkySum);

            pNode = pNode->pParent;
        }
    }

    printf("Funky sum is %d\n", root.funkySum);
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
    // uint8_t testData[] = {2,3,1,3,0,1,1,10,11,12,1,1,0,1,99,2,1,1,2};
    // BuildATree(testData, ARRAY_SIZE(testData));

    BuildATree(input, ARRAY_SIZE(input));

    return 0;
}