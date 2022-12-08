#include "..\common.h"
#include "10input.h"

typedef struct _data {
    int32_t x;
    int32_t y;
    int32_t velocityX;
    int32_t velocityY;
} data;

void PrintTheMessage(char* x[], uint32_t length)
{
    uint32_t stringLength = strlen(x[0]);
    char* z = malloc(stringLength);
    data* d = malloc(sizeof(data) * length);
    for (uint32_t i = 0; i < length; i++)
    {
        // Add in NULLs to make atoi smart
        memcpy(z, x[i], stringLength);
        z[16] = '\0';
        z[24] = '\0';
        z[38] = '\0';
        z[42] = '\0';
        d[i].x = atoi(&z[10]);
        d[i].y = atoi(&z[18]);
        d[i].velocityX = atoi(&z[36]);
        d[i].velocityY = atoi(&z[40]);
        // printf("%d %d %d %d\n", d[i].x, d[i].y, d[i].velocityX, d[i].velocityY);
    }

    int32_t mostMinX = INT32_MAX;
    int32_t mostMinY = INT32_MAX;
    int32_t minX = INT32_MAX;
    int32_t maxX = INT32_MIN;
    int32_t minY = INT32_MAX;
    int32_t maxY = INT32_MIN;
    uint32_t j = 0;
    while (true)
    {
        minX = INT32_MAX;
        maxX = INT32_MIN;
        minY = INT32_MAX;
        maxY = INT32_MIN;
        for (uint32_t i = 0; i < length; i++)
        {
            d[i].x += d[i].velocityX;
            d[i].y += d[i].velocityY;
            if (d[i].x < minX)
            {
                minX = d[i].x;
            }

            if (d[i].x > maxX)
            {
                maxX = d[i].x;
            } 

            if (d[i].y < minY)
            {
                minY = d[i].y;
            }

            if (d[i].y > maxY)
            {
                maxY = d[i].y;
            }
        }

        if (j != 0 && (minX < mostMinX || minY < mostMinY))
        {
            printf("Stopping after %d loops %d vs %d; %d vs %d; %d %d\n", j, minX, mostMinX, minY, mostMinY, minX > mostMinX, minY > mostMinY);
            printf("%d to %d, %d to %d\n", minX, maxX, minY, maxY);
            break;
        }

        mostMinX = minX;
        mostMinY = minY;
        j++;
    }

    // Backup one
    minX = INT32_MAX;
    maxX = INT32_MIN;
    minY = INT32_MAX;
    maxY = INT32_MIN;
    for (uint32_t i = 0; i < length; i++)
    {
        d[i].x -= d[i].velocityX;
        d[i].y -= d[i].velocityY;
        if (d[i].x < minX)
        {
            minX = d[i].x;
        }

        if (d[i].x > maxX)
        {
            maxX = d[i].x;
        } 

        if (d[i].y < minY)
        {
            minY = d[i].y;
        }

        if (d[i].y > maxY)
        {
            maxY = d[i].y;
        }
    }

    // Print the message
    
    for (uint32_t y = minY; y <= maxY; y++)
    {
        for (uint32_t x = minX; x <= maxX; x++)    
        {
            bool printDot = true;
            for (uint32_t i = 0; i < length; i++)
            {
                if (d[i].x == x && d[i].y == y)
                {
                    printDot = false;
                    printf("#");
                    break;
                }
            }
            if (printDot)
            {
                printf(".");
            }
        }
        printf("\n");
    }
}

int main(int argc, char* argv[])
{
    PrintTheMessage(input, ARRAY_SIZE(input));
    return 0;
}