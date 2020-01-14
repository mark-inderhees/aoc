#include "../common.h"
#include "02input.h"

uint32_t GetFirstCharCount(char* string)
{
    char character = string[0];
    uint32_t i = 1;
    uint32_t count = 1;
    while (string[i] != '\0')
    {
        if (string[i] == character)
        {
            count++;
        }
        i++;
    }

    return count;
}

uint32_t GetChecksum(char* x[], uint32_t length)
{
    bool triedChar['z' - 'a' + 1];
    memset(triedChar, 0, sizeof(triedChar));
    uint32_t count2 = 0;
    uint32_t count3 = 0;
    bool counted2 = false;
    bool counted3 = false;
    for (uint32_t i = 0; i < length; i++)
    {
        char* string = x[i];
        uint32_t j = 0;
        while (string[j] != '\0')
        {
            char character = string[j];
            if (!triedChar[character - 'a'])
            {
                triedChar[character - 'a'] = true;
                uint32_t count = GetFirstCharCount(&string[j]);
                if (count == 2 && !counted2)
                {
                    counted2 = true;
                    count2++;
                }
                else if (count == 3 && !counted3)
                {
                    counted3 = true;
                    count3++;
                }
            }
            j++;
        }
        memset(triedChar, 0, sizeof(triedChar));
        counted2 = false;
        counted3 = false;
    }

    return count2 * count3;
}

int main(int argc, char* argv[])
{
    printf("Checksum is %d\n", GetChecksum(input, ARRAY_SIZE(input)));
    return 0;
}