#include "..\common.h"
#include "14input.h"

typedef struct node node;
struct node {
    uint8_t value;
    node* pNext;
    node* pPrevious;
};

void PrintNodes(node* start, node* elf1, node* elf2, uint32_t count)
{
    node* n = start;
    for (uint32_t i = 0; i < count; i++)
    {
        if (n == elf1)
        {
            printf("(%d)", n->value);
        }
        else if (n == elf2)
        {
            printf("[%d]", n->value);
        }
        else
        {
            printf(" %d ", n->value);
        }
        
        n = n->pNext;
    }

    printf("\n");
}

void FindScore(char* input, uint32_t afterX)
{
    uint32_t nodeCount = afterX + 10;
    uint32_t someBuffer = 10;
    node* nodes = malloc(sizeof(node) * (nodeCount + someBuffer));
    node* start = nodes;
    node* end = nodes;
    uint32_t nodeI = 0;
    for (char* c = input; *c != '\0'; c++)
    {
        // Creat the new node
        node* n = &nodes[nodeI];
        n->value = (*c) - '0';

        // printf("%d ", n->value);

        // Instert node into end
        n->pNext = start;
        n->pNext->pPrevious = n;
        n->pPrevious = end;
        n->pPrevious->pNext = n;

        // Update global info
        nodeI++;
        end = n;
    }
    // printf("\n");

    node* lastInputNode = end;

    // node* n = start;
    // for (uint32_t i = 0; i < nodeI; i++)
    // {
    //     printf("%d ", n->value);
    //     n = n->pNext;
    // }
    // printf("\n");

    node* elf1 = start;
    node* elf2 = start->pNext;
    // PrintNodes(start, elf1, elf2, nodeI);

    while (nodeI < nodeCount)
    {
        uint32_t score = elf1->value + elf2->value;

        // Create nodes for the score
        node* tempEnd = &nodes[nodeI];
        node* tempEnd2 = &nodes[nodeI];
        do
        {
            // Create the new node
            node* n = &nodes[nodeI];
            n->value = score % 10;
            score = score / 10;

            // Connect node to temp end
            n->pNext = tempEnd;
            n->pNext->pPrevious = n;

            // Update global info
            nodeI++;
            tempEnd = n;
        } while (score > 0);

        tempEnd2->pNext = start;
        tempEnd2->pNext->pPrevious = tempEnd2;
        tempEnd->pPrevious = end;
        tempEnd->pPrevious->pNext = tempEnd;
        end = tempEnd2;

        // Move the elfs
        uint32_t elf1Move = elf1->value + 1;
        for (uint32_t j = 0; j < elf1Move; j++)
        {
            // printf(".%d.", elf1->value);
            elf1 = elf1->pNext;
        }
        // printf("\n");

        uint32_t elf2Move = elf2->value + 1;
        for (uint32_t j = 0; j < elf2Move; j++)
        {
            elf2 = elf2->pNext;
        }

        // PrintNodes(start, elf1, elf2, nodeI);
    }

    // Print ten nodes after so many nodes
    node* n = start;
    for (uint32_t i = 0; i < afterX; i++)
    {
        n = n->pNext;
    }
    for (uint32_t i = 0; i < 10; i++)
    {
        printf("%d", n->value);
        n = n->pNext;
    }
    printf("\n");
}

int main(int argc, char* argv[])
{
    printf("Hello world\n");
    char* testData = "37";
    // FindScore(testData, 9);
    // FindScore(testData, 5);
    // FindScore(testData, 18);
    // FindScore(testData, 2018);

    char* input = "37";
    FindScore(input, 505961);
    return 0;
}