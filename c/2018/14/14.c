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

void FindScore(char* input, char* input2)
{
    uint32_t nodeCount = 100000000;
    node* nodes = malloc(sizeof(node) * (nodeCount));
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
    node* startHereMark = NULL;
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

        // Check if new data matches input2
        node* n = end;
        uint32_t stringLen = strlen(input2);
        bool match = true;
        for (int32_t j = stringLen - 1; j >= 0; j--)
        {
            if (n->value != input2[j] - '0')
            {
                match = false;
            }
            n = n->pPrevious;
        }

        if (match)
        {
            startHereMark = n;
            break;
        }

        // PrintNodes(start, elf1, elf2, nodeI);
    }

    // Print ten nodes after so many nodes
    // node* n = start;
    // for (uint32_t i = 0; i < afterX; i++)
    // {
    //     n = n->pNext;
    // }
    // for (uint32_t i = 0; i < 10; i++)
    // {
    //     printf("%d", n->value);
    //     n = n->pNext;
    // }
    // printf("\n");

    // Count nodes before startHereMark
    uint32_t someCount = 1;
    while (startHereMark != start)
    {
        startHereMark = startHereMark->pPrevious;
        someCount++;
    }
    printf("%d\n", someCount);
}

void PrintStuff(char* str, uint32_t elf1, uint32_t elf2)
{
    uint32_t strLen = strlen(str);
    for (uint32_t i = 0; i < strLen; i++)
    {
        if (i == elf1)
        {
            printf("(%c)", str[i]);
        }
        else if (i == elf2)
        {
            printf("[%c]", str[i]);
        }
        else
        {
            printf(" %c ", str[i]);
        }
    }
    printf("\n");
}

uint32_t FindScore2(char* input1, char* input2)
{
    #define DIGIT_MAX 100000000
    char* str = malloc(DIGIT_MAX);
    memset(str, 0, DIGIT_MAX);
    memcpy(str, input1, strlen(input1));
    uint32_t digitI = strlen(input1);
    uint32_t input2Len = strlen(input2);

    uint32_t elf1 = 0;
    uint32_t elf2 = 1;
    char strScore1[2] = {0};
    char strScore2[2] = {0};
    while (true)
    {
        // PrintStuff(str, elf1, elf2);
        assert(digitI < DIGIT_MAX - 2);
        strScore1[0] = str[elf1];
        strScore2[0] = str[elf2];
        uint32_t score1 = atoi(strScore1);
        uint32_t score2 = atoi(strScore2);
        uint32_t score = score1 + score2;
        // Scores can be 0 to 18
        if (score >= 10)
        {
            str[digitI++] = '1';
        }
        str[digitI++] = (score % 10) + '0';

        // Move the elfs
        elf1 = (elf1 + score1 + 1) % digitI;
        elf2 = (elf2 + score2 + 1) % digitI;

        // Check if string matches target, need to check two strings
        if (0 == memcmp(input2, &str[digitI - 1 - input2Len], input2Len))
        {
            return digitI - 1 - input2Len;
        }
        else if (0 == memcmp(input2, &str[digitI - 2 - input2Len], input2Len))
        {
            return digitI - 2 - input2Len;
        }
    }

    return -1;
}

int main(int argc, char* argv[])
{
    printf("Hello world\n");

    char* input = "37";

    printf("%d\n", FindScore2(input, "51589"));
    printf("%d\n", FindScore2(input, "01245"));
    printf("%d\n", FindScore2(input, "92510"));
    printf("%d\n", FindScore2(input, "59414"));

    printf("%d\n", FindScore2(input, "505961"));
    return 0;
}