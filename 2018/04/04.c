#include "..\common.h"
#include "04input.h"

typedef enum _GuardAction {
    GuardBeginShift,
    GuardFallsAsleep,
    GuardWakesUp,
} GuardAction;

typedef struct _GuardData {
    uint8_t month;
    uint8_t day;
    uint8_t hour;
    uint8_t minute;
    int32_t id;
    int32_t guardSeenIndex;
    GuardAction action;
} GuardData;

GuardData data[ARRAY_SIZE(input)];

typedef struct _GuardInfo {
    int32_t id;
    uint32_t minute[60];
    uint32_t sleep;
} GuardInfo;

// Returns  -1 if a < b
//           0 if a == b
//          +1 if a > b
int CompareGuardData(const void* a, const void* b)
{
    const GuardData* A = a;
    const GuardData* B = b;
    if (A->month > B->month)
    {
        return 1;
    }
    else if (A->month < B->month)
    {
        return -1;
    }

    if (A->day > B->day)
    {
        return 1;
    }
    else if (A->day < B->day)
    {
        return -1;
    }

    if (A->hour > B->hour)
    {
        return 1;
    }
    else if (A->hour < B->hour)
    {
        return -1;
    }

    if (A->minute > B->minute)
    {
        return 1;
    }
    else if (A->minute < B->minute)
    {
        return -1;
    }

    printf("ERROR: unexpected equal");

    return 0;
}

uint32_t GuardsSeenIndex = 0;
int32_t GuardsSeen[ARRAY_SIZE(input)] = {0};
int32_t GetGuardSeenIndex(int32_t id)
{
    for (uint32_t i = 0; i < GuardsSeenIndex; i++)
    {
        if (GuardsSeen[i] == id)
        {
            return i;
        }
    }

    return -1;
}

void FindGuardSleepsMostAndWhatMin(char* x[], uint32_t length, uint32_t* sleepyId, uint32_t* sleepyMinute)
{
    // Parse the data, eg "[1518-05-01 23:46] Guard #2503 begins shift"
    for (uint32_t actionI = 0; actionI < length; actionI++)
    {
        char* string = x[actionI];
        uint32_t stringI = 0;
        char c = string[stringI];
        uint32_t state = 0;
        char month[3];
        char day[3];
        char hour[3];
        char minute[3];
        uint32_t charI = 0;
        GuardAction action;
        char id[10];
        while (c != '\0')
        {
            switch (state)
            {
            case 0:
            {
                // Find first -
                if (c == '-')
                {
                    state++;
                    charI = 0;
                }
                break;
            }
            
            case 1:
            {
                // Read in month
                if (c == '-')
                {
                    month[charI] = '\0';
                    state++;
                    charI = 0;
                }
                else
                {
                    month[charI++] = c;
                }
                break;
            }

            case 2:
            {
                // Read in day
                if (c == ' ')
                {
                    day[charI] = '\0';
                    state++;
                    charI = 0;
                }
                else
                {
                    day[charI++] = c;
                }
                break;
            }

            case 3:
            {
                // Read in hour
                if (c == ':')
                {
                    hour[charI] = '\0';
                    state++;
                    charI = 0;
                }
                else
                {
                    hour[charI++] = c;
                }
                break;
            }

            case 4:
            {
                // Read in minute
                if (c == ']')
                {
                    minute[charI] = '\0';
                    state++;
                    stringI++; // Skip an extra space
                    charI = 0;
                }
                else
                {
                    minute[charI++] = c;
                }
                break;
            }

            case 5:
            {
                // Determine action
                if (c == 'G')
                {
                    action = GuardBeginShift;
                    // Need to find id
                    stringI += 6; // Skip to first index of id
                    state++;
                    charI = 0;
                }
                else if (c == 'w')
                {
                    action = GuardWakesUp;
                    // done
                    state++;
                    state++;
                }
                else if (c == 'f')
                {
                    action = GuardFallsAsleep;
                    // done
                    state++;
                    state++;
                }
                break;
            }

            case 6:
            {
                // Need to find id
                if (c == ' ')
                {
                    id[charI] = '\0';
                    state++;
                    charI = 0;   
                }
                else
                {
                    id[charI++] = c;
                }
                break;
            }

            case 7:
            {
                // do nothing
                break;
            }
                
            default:
                printf("ERROR: unepxected state");
                return;
            }
            stringI++;
            c = string[stringI];
        }

        // printf("M %s D %s H %s M %s action %d\n", month, day, hour, minute, action);
        // if (action == GuardBeginShift)
        // {
        //     printf("id %s\n", id);
        // }

        data[actionI].month = atoi(month);
        data[actionI].day = atoi(day);
        data[actionI].hour = atoi(hour);
        data[actionI].minute = atoi(minute);
        int32_t tempId = -1;
        if (action == GuardBeginShift)
        {
            tempId = atoi(id);
            data[actionI].id = tempId;

            if (tempId == 0)
            {
                printf("Unexpected ID 0");
                return;
            }

            int32_t guardSeenIndex = GetGuardSeenIndex(tempId);
            if (guardSeenIndex == -1)
            {
                GuardsSeen[GuardsSeenIndex] = tempId;
                guardSeenIndex = GuardsSeenIndex;
                GuardsSeenIndex++;
            }

            data[actionI].guardSeenIndex = guardSeenIndex;
        }
        else
        {
            data[actionI].id = -1;
            data[actionI].guardSeenIndex = -1;
        }
        data[actionI].action = action;
    }

    // Sort the data
    qsort(data, ARRAY_SIZE(data), sizeof(GuardData), CompareGuardData);

    // for (uint32_t i = 0; i < length; i++)
    // {
    //     printf("M %d D %d H %d M %d action %d id %d\n",
    //         data[i].month,
    //         data[i].day,
    //         data[i].hour,
    //         data[i].minute,
    //         data[i].action,
    //         data[i].id
    //         );
    // }

    // Build a guard DB
    GuardInfo* pGuardInfo = malloc(GuardsSeenIndex * sizeof(GuardInfo));
    memset(pGuardInfo, 0, GuardsSeenIndex * sizeof(GuardInfo));
    int32_t currentGuardI = -1;
    uint32_t sleepStart = 0;
    uint32_t mostSleep = 0;
    uint32_t sleepestGuardId = 0;
    uint32_t sleepestGuardSeenId = 0;
    for (uint32_t actionI = 0; actionI < length; actionI++)
    {
        if (data[actionI].action == GuardBeginShift)
        {
            currentGuardI = data[actionI].guardSeenIndex;
            pGuardInfo[currentGuardI].id = data[actionI].id;
        }
        else if (data[actionI].action == GuardFallsAsleep)
        {
            sleepStart = data[actionI].minute;
        }
        else if (data[actionI].action == GuardWakesUp)
        {
            uint32_t sleepEnd = data[actionI].minute;
            pGuardInfo[currentGuardI].sleep += sleepEnd - sleepStart;
            for (uint32_t m = sleepStart; m < sleepEnd; m++)
            {
                pGuardInfo[currentGuardI].minute[m]++;
            }

            if (pGuardInfo[currentGuardI].sleep > mostSleep)
            {
                mostSleep = pGuardInfo[currentGuardI].sleep;
                sleepestGuardId = pGuardInfo[currentGuardI].id;
                sleepestGuardSeenId = currentGuardI;
            }
        }
    }

    // Find sleepiest minute
    uint32_t largestMinuteCount = 0;
    uint32_t largestMinute = 0;
    for (uint32_t m = 0; m < 60; m++)
    {
        uint32_t count = pGuardInfo[sleepestGuardSeenId].minute[m];
        if (count > largestMinuteCount)
        {
            largestMinuteCount = count;
            largestMinute = m;
        }
    }

    *sleepyId = sleepestGuardId;
    *sleepyMinute = largestMinute;
}

int main(int argc, char* argv[])
{
    uint32_t id, minute;
    FindGuardSleepsMostAndWhatMin(input, ARRAY_SIZE(input), &id, &minute);
    printf("Most sleepy Guard %d sleeps most at %d --> %d", id, minute, id*minute);
    return 0;
}