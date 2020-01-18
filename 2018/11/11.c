#include "..\common.h"
#include "11input.h"

void MaxPower(uint32_t serialNumber)
{
    int32_t grid[300][300] = {0};
    for (uint32_t x = 0; x < 300; x++)
    {
        for (uint32_t y = 0; y < 300; y++)
        {
            uint32_t rackId = x + 1 + 10;
            uint32_t powerLevel = rackId * (y + 1) + serialNumber;
            powerLevel = powerLevel * rackId;
            int32_t hundredsDigit = (powerLevel % 1000) / 100;
            grid[x][y] = hundredsDigit - 5;
        }
    }

    // Find the max subgrid
    int32_t max = INT32_MIN;
    uint32_t maxX = 0;
    uint32_t maxY = 0;
    uint32_t maxG = 0;
    for (uint32_t g = 1; g <= 300; g++)
    {
        for (uint32_t x = 0; x < 300 - g + 1; x++)
        {
            for (uint32_t y = 0; y < 300 - g + 1; y++)
            {
                int32_t value = 0;
                for (uint32_t gx = 0; gx < g; gx++)
                {
                    for (uint32_t gy = 0; gy < g; gy++)
                    {
                        value += grid[x + gx][y + gy];
                    }
                }

                if (value > max)
                {
                    max = value;
                    maxX = x;
                    maxY = y;
                    maxG = g;
                }
            }
        }
        printf("%d ", g);
    }

    printf("\n%d at %d,%d,%d\n", max, maxX+1, maxY+1, maxG);
}

int main(int argc, char* argv[])
{
    // MaxPower(18);
    // MaxPower(42);
    MaxPower(9221);
    return 0;
}