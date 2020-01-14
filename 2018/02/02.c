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

void FindNearlyMatchingInputs(char* x[], uint32_t length)
{
    uint32_t* miss = (uint32_t*)malloc(length * sizeof(uint32_t));
    char* string1 = NULL;
    char* string2 = NULL;
    for (uint32_t stringI1 = 0; stringI1 < length; stringI1++)
    {
        string1 = x[stringI1];
        memset(miss, 0, length * sizeof(uint32_t));
        uint32_t charI = 0;
        bool hit = false;
        while (string1[charI] != '\0')
        {
            hit = false;
            for (uint32_t stringI2 = stringI1 + 1; stringI2 < length; stringI2++)
            {
                if (miss[stringI2] < 2)
                {
                    hit = true;
                    string2 = x[stringI2];
                    if (string1[charI] != x[stringI2][charI])
                    {
                        miss[stringI2]++;
                    }
                }
            }

            if (!hit)
            {
                break;
            }
            charI++;
        }

        if (hit)
        {
            printf("Found match with %s\n", string1);
            printf("%s\n", string1);
            printf("%s\n", string2);
            charI = 0;
            while (string1[charI] != '\0')
            {
                if (string1[charI] == string2[charI])
                {
                    printf("%c", string1[charI]);
                }
                charI++;
            }
        }
    }
}

int main(int argc, char* argv[])
{
    printf("Checksum is %d\n", GetChecksum(input, ARRAY_SIZE(input)));
    FindNearlyMatchingInputs(input, ARRAY_SIZE(input));
    return 0;
}