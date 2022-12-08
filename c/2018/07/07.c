#include "..\common.h"
#include "07input.h"

typedef struct _item {
    bool provides[26];
    uint32_t dependsCount;
} item;

typedef struct _worker
{
    uint32_t count;
    uint32_t workingOn;
} worker;

void FindTheOrder(char* input[], uint32_t length)
{
    // Populate data from input
    item items[26] = {0};
    for (uint32_t i = 0; i < length; i++)
    {
        char thisItem = input[i][5];
        char provides = input[i][36];
        // printf("%c provides %c\n", thisItem, provides);
        items[thisItem - 'A'].provides[provides - 'A'] = true;
        items[provides - 'A'].dependsCount++;
    }

    // Find intial work to do
    bool todo[26] = {0};
    for (uint32_t i = 0; i < 26; i++)
    {
        if (items[i].dependsCount == 0)
        {
            todo[i] = true;
        }
    }

    // Do all the work!
    worker workers[5] = {0};
    uint32_t doneCount = 0;
    uint32_t time = 0;
    while (doneCount != 26)
    {
        // Check if the workers finished work
        for (uint32_t workerI = 0; workerI < 5; workerI++)
        {
            if (workers[workerI].count > 0)
            {
                // Complete one second of work
                workers[workerI].count--;
                if (workers[workerI].count == 0)
                {
                    // Work is done!
                    doneCount++;
                    uint32_t itemDone = workers[workerI].workingOn;
                    // printf("%d %c\n", time, itemDone + 'A');

                    // Mark this dependency clear from all that it provides
                    for (uint32_t p = 0; p < 26; p++)
                    {
                        if (items[itemDone].provides[p])
                        {
                            items[p].dependsCount--;

                            // If this provided item is ready for work, then set it up
                            if (items[p].dependsCount == 0)
                            {
                                todo[p] = true;
                            }
                        }
                    }
                }
            }
        }

        // See if there is work to do
        for (uint32_t workerI = 0; workerI < 5; workerI++)
        {
            if (workers[workerI].count == 0)
            {
                // find work to do
                for (uint32_t todoI = 0; todoI < 26; todoI++)
                {
                    if (todo[todoI])
                    {
                        // Do this work!
                        workers[workerI].workingOn = todoI;
                        todo[todoI] = false;
                        workers[workerI].count = 60 + todoI + 1;
                        break;
                    }
                }
            }
        }
        time++;
    }

    // Step time back one
    time--;

    printf("Done in %d\n", time);
}

int main(int argc, char* argv[])
{
    FindTheOrder(input, ARRAY_SIZE(input));
    return 0;
}