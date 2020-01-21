#include "..\common.h"
#include "13input.h"

typedef enum _piece {
    piece_empty = 0,
    horizontal,
    vertical,
    intersection,
    horizontalLeftTurn_verticalRightTurn,
    horizontalRightTurn_verticalLeftTurn,
} piece;

typedef enum _direction {
    direction_empty = 0,
    up,
    down,
    left,
    right
} direction;

direction ProcessTurn(piece* map, direction car, uint32_t x, uint32_t y, uint32_t width)
{
    piece p = map[x + y * width];
    if (p == horizontalLeftTurn_verticalRightTurn)
    {
        // Piece '/'
        if (car == up)
        {
            return right;
        }
        else if (car == down)
        {
            return left;
        }
        else if (car == left)
        {
            return down;
        }
        else if (car == right)
        {
            return up;
        }
    }
    else if (p == horizontalRightTurn_verticalLeftTurn)
    {
        // Piece '\'
        if (car == up)
        {
            return left;
        }
        else if (car == down)
        {
            return right;
        }
        else if (car == left)
        {
            return up;
        }
        else if (car == right)
        {
            return down;
        }
    }
    else
    {
        // Do nothing
        return car;
    }
}

void DrawCars(direction* cars, piece* map, uint32_t width, uint32_t height)
{
    printf("-------------------\n");
    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            direction car = cars[x + y * width];
            switch (car)
            {
            case up:
            {
                printf("^");
                break;
            }

            case down:
            {
                printf("v");
                break;
            }

            case left:
            {
                printf("<");
                break;
            }

            case right:
            {
                printf(">");
                break;
            }

            default:
            {
                piece p = map[x + y * width];
                switch (p)
                {
                case horizontal:
                {
                    printf("-");
                    break;
                }

                case vertical:
                {
                    printf("|");
                    break;
                }

                case intersection:
                {
                    printf("+");
                    break;
                }

                case horizontalLeftTurn_verticalRightTurn:
                {
                    printf("/");
                    break;
                }

                case horizontalRightTurn_verticalLeftTurn:
                {
                    printf("\\");
                    break;
                }

                default:
                {
                    printf(" ");
                    break;
                }
                }

                break;
            }
            }
        }
        printf("\n");
    }
}

void FindTheCrash(char* input[], uint32_t height)
{
    // Populate map and cars
    uint32_t width = strlen(input[0]);
    piece* map = malloc(sizeof(piece) * width * height);
    memset(map, 0, sizeof(piece) * width * height);
    direction* cars = malloc(sizeof(direction) * width * height);
    direction* cars2 = malloc(sizeof(direction) * width * height);
    memset(cars, 0, sizeof(direction) * width * height);
    memset(cars2, 0, sizeof(direction) * width * height);
    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            char d = input[y][x];
            switch (d)
            {
            case ' ':
            {
                map[x + y * width] = piece_empty;
                break;
            }

            case '-':
            {
                map[x + y * width] = horizontal;
                break;
            }

            case '|':
            {
                map[x + y * width] = vertical;
                break;
            }

            case '+':
            {
                map[x + y * width] = intersection;
                break;
            }

            case '/':
            {
                map[x + y * width] = horizontalLeftTurn_verticalRightTurn;
                break;
            }

            case '\\':
            {
                map[x + y * width] = horizontalRightTurn_verticalLeftTurn;
                break;
            }

            case '^':
            {
                map[x + y * width] = vertical;
                cars[x + y * width] = up;
                break;
            }

            case 'v':
            {
                map[x + y * width] = vertical;
                cars[x + y * width] = down;
                break;
            }

            case '<':
            {
                map[x + y * width] = horizontal;
                cars[x + y * width] = left;
                break;
            }

            case '>':
            {
                map[x + y * width] = horizontal;
                cars[x + y * width] = right;
                break;
            }

            default:
            {
                assert(false);
                break;
            }
            }
        }
    }

    // Run until crash!
    uint32_t iteration = 0;
    direction* carsNow = NULL;
    direction* carsNext = NULL;
    uint32_t x2 = 0;
    uint32_t y2 = 0;
    direction car = direction_empty;
    while (true)
    {
        if (iteration % 2 == 0)
        {
            carsNow = cars;
            carsNext = cars2;
        }
        else
        {
            carsNow = cars2;
            carsNext = cars;
        }
        memset(carsNext, 0, sizeof(direction) * width * height);

        DrawCars(carsNow, map, width, height);

        // Move the cars
        for (uint32_t y = 0; y < height; y++)
        {
            for (uint32_t x = 0; x < width; x++)
            {
                car = carsNow[x + y * width];
                switch (car)
                {
                case up:
                {
                    assert(y > 0);
                    x2 = x;
                    y2 = y - 1;
                    break;
                }

                case down:
                {
                    assert(y < height - 1);
                    x2 = x;
                    y2 = y + 1;
                    break;
                }

                case left:
                {
                    assert(x > 0);
                    x2 = x - 1;
                    y2 = y;
                    break;
                }

                case right:
                {
                    assert(x < width - 1);
                    x2 = x + 1;
                    y2 = y;
                    break;
                }

                default:
                {
                    continue;
                }
                }

                if (carsNext[x2 + y2 * width] != direction_empty)
                {
                    // Crash!
                    printf("Crash at %d,%d on iteration %d\n", x2, y2, iteration);
                    return;
                }
                carsNext[x2 + y2 * width] = ProcessTurn(map, car, x2, y2, width);
            }
        }

        iteration++;
    }
}

int main(int argc, char* argv[])
{
    FindTheCrash(testData, ARRAY_SIZE(testData));
    printf("Hello world\n");
    return 0;
}