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

typedef struct _cart {
    direction d;
    uint32_t count;
} cart;

cart ProcessTurn(piece* map, cart car, uint32_t x, uint32_t y, uint32_t width)
{
    piece p = map[x + y * width];
    if (p == horizontalLeftTurn_verticalRightTurn)
    {
        // Piece '/'
        if (car.d == up)
        {
            car.d = right;
            return car;
        }
        else if (car.d == down)
        {
            car.d = left;
            return car;
        }
        else if (car.d == left)
        {
            car.d = down;
            return car;
        }
        else if (car.d == right)
        {
            car.d = up;
            return car;
        }
    }
    else if (p == horizontalRightTurn_verticalLeftTurn)
    {
        // Piece '\'
        if (car.d == up)
        {
            car.d = left;
            return car;
        }
        else if (car.d == down)
        {
            car.d = right;
            return car;
        }
        else if (car.d == left)
        {
            car.d = up;
            return car;
        }
        else if (car.d == right)
        {
            car.d = down;
            return car;
        }
    }
    else if (p == intersection)
    {
        switch (car.count % 3)
        {
        case 0:
        {
            // Left turn
            if (car.d == up)
            {
                car.d = left;
                car.count++;
                return car;
            }
            else if (car.d == down)
            {
                car.d = right;
                car.count++;
                return car;
            }
            else if (car.d == left)
            {
                car.d = down;
                car.count++;
                return car;
            }
            else if (car.d == right)
            {
                car.d = up;
                car.count++;
                return car;
            }
            break;
        }

        case 1:
        {
            // Go stright
            car.count++;
            return car;
            break;
        }

        case 2:
        {
            // Right turn
            if (car.d == up)
            {
                car.d = right;
                car.count++;
                return car;
            }
            else if (car.d == down)
            {
                car.d = left;
                car.count++;
                return car;
            }
            else if (car.d == left)
            {
                car.d = up;
                car.count++;
                return car;
            }
            else if (car.d == right)
            {
                car.d = down;
                car.count++;
                return car;
            }
            break;
        }

        default:
            break;
        }
    }
    else
    {
        // Do nothing
        return car;
    }
}

void DrawCars(cart* cars, piece* map, uint32_t width, uint32_t height)
{
    printf("-------------------\n");
    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            direction car = cars[x + y * width].d;
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
                printf(" ");
                break;
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
    cart* cars = malloc(sizeof(cart) * width * height);
    cart* cars2 = malloc(sizeof(cart) * width * height);
    memset(cars, 0, sizeof(cart) * width * height);
    memset(cars2, 0, sizeof(cart) * width * height);
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
                cars[x + y * width].d = up;
                break;
            }

            case 'v':
            {
                map[x + y * width] = vertical;
                cars[x + y * width].d = down;
                break;
            }

            case '<':
            {
                map[x + y * width] = horizontal;
                cars[x + y * width].d = left;
                break;
            }

            case '>':
            {
                map[x + y * width] = horizontal;
                cars[x + y * width].d = right;
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
    cart* carsNow = NULL;
    cart* carsNext = NULL;
    uint32_t x2 = 0;
    uint32_t y2 = 0;
    cart car = {0};
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
        memset(carsNext, 0, sizeof(cart) * width * height);

        // DrawCars(carsNow, map, width, height);

        // Move the cars
        for (uint32_t y = 0; y < height; y++)
        {
            for (uint32_t x = 0; x < width; x++)
            {
                car = carsNow[x + y * width];
                switch (car.d)
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

                if (carsNext[x2 + y2 * width].d != direction_empty)
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
    FindTheCrash(input, ARRAY_SIZE(input));
    printf("Hello world\n");
    return 0;
}