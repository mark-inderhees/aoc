#include "common.h"

int32_t SumOfArray(int32_t x[], uint32_t length)
{
    int32_t sum = 0;
    for (uint32_t i = 0; i < length; i++)
    {
        sum += x[i];
    }

    return sum;
}

typedef struct entry entry;
struct entry {
    bool set;
    int32_t sum;
    uint32_t index;
    entry* pNext;
};

int32_t FindFirstRepeatSum(int32_t x[], uint32_t length)
{
    // Build an array for each mod of the sum of the array. The first repeat
    // will be some number that has the same mod, as that number will eventually
    // add into the larger number. Find the smallest diff to figure out which
    // will add in first. Of all the smallest diff, find the first index to
    // find which will add in first.

    uint32_t mod = abs(SumOfArray(x, length));
    entry* mods = malloc(sizeof(entry) * mod);
    memset(mods, 0, sizeof(entry) * mod);
    int32_t sum = 0;
    uint32_t m = 0;
    uint32_t diff = UINT32_MAX;
    int32_t diffX1 = INT32_MAX;
    uint32_t diffI1 = UINT32_MAX;
    int32_t diffX2 = INT32_MAX;
    for (uint32_t i = 0; i < length; i++)
    {
        sum += x[i];

        // Caclulate mod, use positive values only
        int32_t tempSum = sum;
        while (tempSum < 0)
        {
            tempSum += mod;
        }
        m = tempSum % mod;

        if (!mods[m].set)
        {
            // This is first entry for that mod
            mods[m].set = true;
            mods[m].sum = sum;
            mods[m].index = i;
            mods[m].pNext = NULL;
        }
        else
        {
            // There is an existing entry, calculate diff with all other entries
            entry* pNext = &mods[m];
            while (true)
            {
                // Check if this new entry is a new minimal diff at a minimal
                // index.
                if ((abs(sum - pNext->sum) < diff) ||
                    ((abs(sum - pNext->sum) == diff) && (pNext->index < diffI1)))
                {
                    // This is a new 'first found'!
                    diff = abs(sum - pNext->sum);
                    diffX1 = pNext->sum;
                    diffI1 = pNext->index;
                    diffX2 = sum;
                }

                // Stop loop when no more entries
                if (pNext->pNext == NULL)
                {
                    break;
                }
                pNext = pNext->pNext;
            }

            // Add this new entry to the list at this mod index
            pNext->pNext = malloc(sizeof(entry));
            assert(pNext->pNext != NULL);
            pNext = pNext->pNext;
            memset(pNext, 0, sizeof(entry));
            pNext->sum = sum;
            pNext->index = i;
            pNext->pNext = NULL;
        }
    }

    // Return larger number
    return diffX1 > diffX2 ? diffX1 : diffX2;
}
