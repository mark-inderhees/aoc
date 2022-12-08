#include "..\common.h"
#include "09input.h"

typedef struct node node;
struct node {
    node* pPrevious;
    node* pNext;
    uint64_t marbleNumber;
};

void MarbleGame(uint32_t playerCount, uint64_t maxMarble)
{
    uint64_t* score = malloc(sizeof(uint64_t) * playerCount);
    memset(score, 0, sizeof(uint64_t) * playerCount);
    node* marbles = malloc(sizeof(node) * (maxMarble + 1));
    uint32_t playerI = 1;
    node root = {0};
    node* pNode = &root;
    pNode->pPrevious = pNode;
    pNode->pNext = pNode;
    for(uint64_t marbleI = 1; marbleI <= maxMarble; marbleI++)
    {
        if (marbleI % 23 != 0)
        {
            // Move one right and insert this new marble to the right
            pNode = pNode->pNext;
            node* pOldRight = pNode->pNext;
            pNode->pNext = &(marbles[marbleI]);
            pNode->pNext->pPrevious = pNode;
            pNode = pNode->pNext;
            pNode->pNext = pOldRight;
            pNode->pNext->pPrevious = pNode;
            pNode->marbleNumber = marbleI;
            // printf("New: %d %d\n", marbleI, pNode->marbleNumber);
        }
        else
        {
            // Score one for this player! Move 7 left and remove that node
            for (uint32_t j = 0; j < 7; j++)
            {
                pNode = pNode->pPrevious;
            }
            // printf("Using: %d %d\n", marbleI, pNode->marbleNumber);
            score[playerI] += marbleI + (pNode->marbleNumber);
            // printf("Player %d score is %d, just got marbles %d and %d\n",
            //     playerI,
            //     score[playerI],
            //     marbleI,
            //     pNode->marbleNumber);
            pNode->pPrevious->pNext = pNode->pNext;
            pNode->pNext->pPrevious = pNode->pPrevious;
            pNode = pNode->pNext;
        }
        playerI = (playerI + 1) % playerCount;
    }

    uint64_t maxScore = 0;
    for (uint32_t i = 0; i < playerCount; i++)
    {
        if (score[i] > maxScore)
        {
            maxScore = score[i];
        }
    }

    printf("Max score is %lld\n", maxScore);
}

int main(int argc, char* argv[])
{
    MarbleGame(473, 70904 * 100);
    // MarbleGame(9, 25);
    return 0;
}