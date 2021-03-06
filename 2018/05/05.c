#include "..\common.h"
#include "05input.h"

void RemoveChar(char* string, char c)
{
    uint32_t i = 1;
    while (string[i] != '\0')
    {
        if (string[i] == c || string[i] == c - ('a' -'A') || string[i] == c + ('a' - 'A'))
        {
            string[i] = '_';
        }
        i++;
    }
}

bool Reacted(char c1, char c2)
{
    // printf("%d vs %d\n", abs(c1 - c2), 'a' - 'A');
    if (abs(c1 - c2) == ('a' - 'A'))
    {
        return true;
    }
    return false;
}

uint32_t GetPostReactionCount(char* string)
{
    uint32_t lastGoodI = 0;
    uint32_t i = 1;
    while (string[i] != '\0')
    {
        if (string[i] != '_')
        {
            if (Reacted(string[i], string[lastGoodI]))
            {
                // Deleted these two
                string[i] = '_';
                string[lastGoodI] = '_';

                // Move lastGoodI backwards
                while (lastGoodI > 0 && string[lastGoodI] == '_')
                {
                    lastGoodI--;
                }

                // Edge case where all previous letters are gone
                if (lastGoodI == 0 && string[lastGoodI] == '_')
                {
                    lastGoodI = i;
                }
            }
            else
            {
                lastGoodI = i;
            }
        }
        
        i++;
    }

    uint32_t count = 0;
    i = 0;
    while (string[i] != '\0')
    {
        if (string[i] != '_')
        {
            count++;
        }
        i++;
    }

    return count;
}

int main(int argc, char* argv[])
{
    // Build a modifyable input data
    uint32_t length = strlen(input);
    char* data = malloc(length + 1);

    // Try all chars
    uint32_t countMin = UINT32_MAX;
    for (char c = 'a'; c <= 'z'; c++)
    {
        memcpy(data, input, length);
        data[length] = '\0';

        RemoveChar(data, c);
        uint32_t count = GetPostReactionCount(data);
        if (count < countMin)
        {
            countMin = count;
        }

        // printf("GetPostReactionCount gives %d of %d\n", GetPostReactionCount(data), length);
    }
    // printf("%s\n", data);
    printf("Min size is %d", countMin);
    return 0;
}