#include "..\common.h"
#include "07input.h"

typedef struct _item {
    bool provides[26];
    uint32_t dependsCount;
} item;

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
    uint32_t i = 0;
    while (i < 26)
    {
        if (todo[i])
        {
            // Do this work!
            printf("%c", i + 'A');
            todo[i] = false;

            // Mark this dependency clear from all that it provides
            for (uint32_t p = 0; p < 26; p++)
            {
                if (items[i].provides[p])
                {
                    items[p].dependsCount--;

                    // If this provided item is ready for work, then set it up
                    if (items[p].dependsCount == 0)
                    {
                        todo[p] = true;
                    }
                }
            }

            // Start this work list from the begining
            i = 0;
            continue;
        }
        i++;
    }
}

int main(int argc, char* argv[])
{
    FindTheOrder(input, ARRAY_SIZE(input));
    return 0;
}