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
    directionMin = -1,
    up,
    right,
    down,
    left,
    directionMax
} direction;

typedef struct _cart {
    direction d;
    uint32_t count;
} cart;

void TurnRight(cart* cart)
{
    cart->d = (cart->d + 1) % directionMax;
}

void TurnLeft(cart* cart)
{
    cart->d -= 1;
    if (cart->d == directionMin)
    {
        cart->d = left;
    }
}

void ProcessTurn(piece* map, cart* cart, uint32_t x, uint32_t y, uint32_t width)
{
    piece p = map[x + y * width];
    if (p == horizontalLeftTurn_verticalRightTurn)
    {
        // Piece '/'
        // printf(" /");
        if (cart->d == right || cart->d == left)
        {
            TurnLeft(cart);
        }
        else if (cart->d == up || cart->d == down)
        {
            TurnRight(cart);
        }
        else
        {
            assert(false);
        }
    }
    else if (p == horizontalRightTurn_verticalLeftTurn)
    {
        // Piece '\'
        // printf(" \\");
        if (cart->d == right || cart->d == left)
        {
            TurnRight(cart);
        }
        else if (cart->d == up || cart->d == down)
        {
            TurnLeft(cart);
        }
        else
        {
            assert(false);
        }
    }
    else if (p == intersection)
    {
        // printf(" +");
        switch (cart->count % 3)
        {
        case 0:
        {
            TurnLeft(cart);
            break;
        }

        case 1:
        {
            // Go stright
            break;
        }

        case 2:
        {
            TurnRight(cart);
            break;
        }

        default:
            assert(false);
            break;
        }

        cart->count++;
    }
    else if (p == horizontal)
    {
        // printf(" -");
        assert(cart->d == left || cart->d == right);
    }
    else if (p == vertical)
    {
        // printf(" |");
        assert(cart->d == up || cart->d == down);
    }
    else
    {
        // printf("%d %d %d\n", p, x, y);
        assert(false);
    }

    // printf(" %d", cart->count % 3);
}

void DrawCars(uint32_t* cartsMap, cart carts[], piece* map, uint32_t width, uint32_t height)
{
    printf("-------------------\n");
    for (uint32_t y = 0; y < height; y++)
    {
        for (uint32_t x = 0; x < width; x++)
        {
            uint32_t cartId = cartsMap[x + y * width];
            direction cartDir = carts[cartId].d;
            switch (cartDir)
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
    // Populate map and carts
    uint32_t width = strlen(input[0]);
    piece* map = malloc(sizeof(piece) * width * height);
    memset(map, 0, sizeof(piece) * width * height);
    uint32_t* cartsMap1 = malloc(sizeof(uint32_t) * width * height);
    uint32_t* cartsMap2 = malloc(sizeof(uint32_t) * width * height);
    memset(cartsMap1, 0, sizeof(uint32_t) * width * height);
    memset(cartsMap2, 0, sizeof(uint32_t) * width * height);
    uint32_t cartId = 1;
    #define MAX_CARTS 100
    cart carts[MAX_CARTS] = {0};
    carts[0].d = directionMin; // cart zero is special, it is "no cart"
    for (uint32_t y = 0; y < height; y++)
    {
        assert(strlen(input[y]) == width);
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
                cartsMap1[x + y * width] = cartId;
                carts[cartId].d = up;
                cartId++;
                break;
            }

            case 'v':
            {
                map[x + y * width] = vertical;
                cartsMap1[x + y * width] = cartId;
                carts[cartId].d = down;
                cartId++;
                break;
            }

            case '<':
            {
                map[x + y * width] = horizontal;
                cartsMap1[x + y * width] = cartId;
                carts[cartId].d = left;
                cartId++;
                break;
            }

            case '>':
            {
                map[x + y * width] = horizontal;
                cartsMap1[x + y * width] = cartId;
                carts[cartId].d = right;
                cartId++;
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

    assert(cartId < MAX_CARTS);

    // Run until crash!
    uint32_t iteration = 0;
    uint32_t* cartsNow = NULL;
    uint32_t* cartsNext = NULL;
    uint32_t x2 = 0;
    uint32_t y2 = 0;
    while (true)
    {
        if (iteration % 2 == 0)
        {
            cartsNow = cartsMap1;
            cartsNext = cartsMap2;
        }
        else
        {
            cartsNow = cartsMap2;
            cartsNext = cartsMap1;
        }
        memset(cartsNext, 0, sizeof(uint32_t) * width * height);

        // DrawCars(cartsNow, carts, map, width, height);

        // Move the carts
        for (uint32_t y = 0; y < height; y++)
        {
            for (uint32_t x = 0; x < width; x++)
            {
                cartId = cartsNow[x + y * width];
                cartsNow[x + y * width] = 0;
                if (cartId == 0)
                {
                    continue;
                }

                // printf("\n(%d, %d)", x, y);

                switch (carts[cartId].d)
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
                    assert(false);
                }
                }

                if (cartsNext[x2 + y2 * width] != 0 || cartsNow[x2 + y2 * width] != 0)
                {
                    // Crash!
                    // cartsNext[x2 + y2 * width] = cartId;
                    // ProcessTurn(map, &carts[cartId], x2, y2, width);
                    // DrawCars(cartsNext, carts, map, width, height);
                    printf("Crash at %d,%d on iteration %d\n", x2, y2, iteration);
                    return;
                }
                cartsNext[x2 + y2 * width] = cartId;
                ProcessTurn(map, &carts[cartId], x2, y2, width);

                // switch (carts[cartId].d)
                // {
                // case up:
                // {
                //     printf(" ^");
                //     break;
                // }

                // case down:
                // {
                //     printf(" v");
                //     break;
                // }

                // case left:
                // {
                //     printf(" <");
                //     break;
                // }

                // case right:
                // {
                //     printf(" >");
                //     break;
                // }

                // default:
                // {
                //     assert(false);
                // }
                // }
            }
        }

        iteration++;
        // printf("\nnew round");
    }
}

int main(int argc, char* argv[])
{
    FindTheCrash(testData, ARRAY_SIZE(testData));
    FindTheCrash(input, ARRAY_SIZE(input));
    return 0;
}