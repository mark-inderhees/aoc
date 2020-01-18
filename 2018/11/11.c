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

    // Find the max 3x3
    int32_t max = INT32_MIN;
    uint32_t maxX = 0;
    uint32_t maxY = 0;
    for (uint32_t x = 0; x < 300 - 2; x++)
    {
        for (uint32_t y = 0; y < 300 - 2; y++)
        {
            int32_t value =
                grid[x][y]   + grid[x+1][y]   + grid[x+2][y] +
                grid[x][y+1] + grid[x+1][y+1] + grid[x+2][y+1] +
                grid[x][y+2] + grid[x+1][y+2] + grid[x+2][y+2];
            if (value > max)
            {
                max = value;
                maxX = x;
                maxY = y;
            }
        }
    }

    printf("%d at %d,%d\n", max, maxX+1, maxY+1);
}

int main(int argc, char* argv[])
{
    MaxPower(18);
    MaxPower(42);
    MaxPower(9221);
    return 0;
}