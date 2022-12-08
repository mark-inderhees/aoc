#include "..\common.h"
#include "19input.h"

typedef enum _opcodes {
    opcodeMin,
    addr = opcodeMin,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
    opcodeCount
} opcodes;

const char* OPCODE_NAMES[] = {
    "addr",
    "addi",
    "mulr",
    "muli",
    "banr",
    "bani",
    "borr",
    "bori",
    "setr",
    "seti",
    "gtir",
    "gtri",
    "gtrr",
    "eqir",
    "eqri",
    "eqrr",
};

typedef struct _instruction {
    opcodes opcode;
    uint64_t inputA;
    uint64_t inputB;
    uint64_t outputC;
} instruction;

uint64_t ProcessOpcode(opcodes opcode, uint64_t inputA, uint64_t inputB, uint64_t outputC, uint64_t registersBefore[6])
{
    (void)(outputC);

    uint64_t output;
    switch (opcode)
    {
        case addr:
        {
            output = registersBefore[inputA] + registersBefore[inputB];
            // printf("r%d = r%d + r%d = %d + %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], registersBefore[inputB], output);
            break;
        }

        case addi:
        {
            output = registersBefore[inputA] + inputB;
            // printf("r%d = r%d + %d = %d + %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], inputB, output);
            break;
        }

        case mulr:
        {
            output = registersBefore[inputA] * registersBefore[inputB];
            // printf("r%d = r%d * r%d = %d * %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], registersBefore[inputB], output);
            break;
        }

        case muli:
        {
            output = registersBefore[inputA] * inputB;
            // printf("r%d = r%d * %d = %d * %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], inputB, output);
            break;
        }

        case banr:
        {
            output = registersBefore[inputA] & registersBefore[inputB];
            // printf("r%d = r%d & r%d = %d & %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], registersBefore[inputB], output);
            break;
        }

        case bani:
        {
            output = registersBefore[inputA] & inputB;
            // printf("r%d = r%d & r%d = %d & %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], inputB, output);
            break;
        }

        case borr:
        {
            output = registersBefore[inputA] | registersBefore[inputB];
            // printf("r%d = r%d | r%d = %d | %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], registersBefore[inputB], output);
            break;
        }

        case bori:
        {
            output = registersBefore[inputA] | inputB;
            // printf("r%d = r%d | r%d = %d | %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], inputB, output);
            break;
        }

        case setr:
        {
            output = registersBefore[inputA];
            // printf("r%d = r%d = %d\n", outputC, inputA, registersBefore[inputA]);
            break;
        }

        case seti:
        {
            output = inputA;
            // printf("r%d = %d\n", outputC, inputA);
            break;
        }

        case gtir:
        {
            output = inputA > registersBefore[inputB] ? 1 : 0;
            // printf("r%d = %d > r%d = %d > %d = %d\n", outputC, inputA, inputB, inputA, registersBefore[inputB], output);
            break;
        }

        case gtri:
        {
            output = registersBefore[inputA] > inputB ? 1 : 0;
            // printf("r%d = r%d > %d = %d > %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], inputB, output);
            break;
        }

        case gtrr:
        {
            output = registersBefore[inputA] > registersBefore[inputB] ? 1 : 0;
            // printf("r%d = r%d > r%d = %d > %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], registersBefore[inputB], output);
            break;
        }

        case eqir:
        {
            output = inputA == registersBefore[inputB] ? 1 : 0;
            // printf("r%d = %d == r%d = %d == %d = %d\n", outputC, inputA, inputB, inputA, registersBefore[inputB], output);
            break;
        }

        case eqri:
        {
            output = registersBefore[inputA] == inputB ? 1 : 0;
            // printf("r%d = r%d == %d = %d == %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], inputB, output);
            break;
        }

        case eqrr:
        {
            output = registersBefore[inputA] == registersBefore[inputB] ? 1 : 0;
            // printf("r%d = r%d == r%d = %d == %d = %d\n", outputC, inputA, inputB, registersBefore[inputA], registersBefore[inputB], output);
            break;
        }


        default:
        {
            assert(false);
            return 0;
        }
    }

    return output;
}

uint64_t Problem1(char* input[], uint32_t length)
{
    // Parse input
    // Get program counter
    uint64_t programCounterRegister = input[0][4] - '0';

    // Get instructions
    instruction* instructions = malloc(sizeof(instruction) * (length - 1));
    uint32_t instructionI = 0;
    for (uint32_t i = 1; i < length; i++, instructionI++)
    {
        // Prase opcode
        uint32_t stringLength = strlen(input[i]);
        char* instructionString = malloc(stringLength);
        memcpy(instructionString, input[i], stringLength);
        instructionString[4] = '\0';
        for (uint32_t j = 0; j <= opcodeCount; j++)
        {
            assert(j != opcodeCount);
            if (0 == strcmp(instructionString, OPCODE_NAMES[j]))
            {
                instructions[instructionI].opcode = j;
                break;
            }
        }

        // Prase parameters
        uint32_t charI = 5;
        instructions[instructionI].inputA = input[i][charI] - '0';
        charI = 7;
        instructions[instructionI].inputB = input[i][charI] - '0';
        charI++;
        if (input[i][charI] != ' ')
        {
            instructions[instructionI].inputB *= 10;
            instructions[instructionI].inputB += input[i][charI] - '0';
            charI++;
        }
        charI++;
        instructions[instructionI].outputC = input[i][charI] - '0';
    }

    printf("#ip %lld\n", programCounterRegister);
    for (uint32_t i = 0; i < length - 1; i++)
    {
        printf("%s %lld %lld %lld\n",
            OPCODE_NAMES[instructions[i].opcode],
            instructions[i].inputA,
            instructions[i].inputB,
            instructions[i].outputC);
    }

    // Now process the program!
    uint64_t registers[6] = {0};
    registers[0] = 0;
    uint64_t result = 0;
    uint32_t jumpCount = 0;
    uint32_t instructionCount = 0;
    uint32_t instructionCountSinceLastJump = 0;
    bool print = false;
    for (uint32_t instructionPointer = 0; instructionPointer < length - 1; instructionPointer++)
    {
        instructionCount++;
        instructionCountSinceLastJump++;
        uint32_t tempIp = instructionPointer;
        registers[programCounterRegister] = instructionPointer;
        if (print)
        {
            printf("ip=%d [%lld, %lld, %lld, %lld, %lld, %lld] \t%s %lld %lld %lld",
                instructionPointer,
                registers[0], registers[1], registers[2], registers[3], registers[4], registers[5],
                OPCODE_NAMES[instructions[instructionPointer].opcode],
                instructions[instructionPointer].inputA,
                instructions[instructionPointer].inputB,
                instructions[instructionPointer].outputC);
        }

        if (instructionPointer == 4)
        {
            // printf("%lld == %lld ?\n", registers[2], registers[1]);
            if (registers[2] == registers[1])
            {
                // print = true;
            }
        }
        else if (instructionPointer == 9)
        {
            // printf("%lld > %lld ?\n", registers[4], registers[1]);
            if (registers[4] > registers[1])
            {
                // print = true;
            }
        }

        result = ProcessOpcode(
            instructions[instructionPointer].opcode,
            instructions[instructionPointer].inputA,
            instructions[instructionPointer].inputB,
            instructions[instructionPointer].outputC,
            registers);
        registers[instructions[instructionPointer].outputC] = result;
        if (print)
        {
            printf(" [%lld, %lld, %lld, %lld, %lld, %lld]\n",
                registers[0], registers[1], registers[2], registers[3], registers[4], registers[5]);
        }
        instructionPointer = registers[programCounterRegister];
        if (tempIp != instructionPointer)
        {
            jumpCount++;
            // if (print || ((tempIp != 6) && (tempIp != 11)))
            // {
            //     printf("Jump from %d to %d \t[jc:%d ic:%d iclj:%d]\n", tempIp, instructionPointer + 1,
            //         jumpCount, instructionCount, instructionCountSinceLastJump);
            // }
            instructionCountSinceLastJump = 0;
        }
    }

    printf("[jc:%d ic:%d iclj:%d]\n",
        jumpCount, instructionCount, instructionCountSinceLastJump);

    return registers[0];
}

uint32_t TheCode(uint32_t r0)
{
    // Jump to label_init
    // addi 3 16 3  pc = pc + 16                      ===== PC is R3

    // Here is label_init
    // addi 1 2 1   r1 = r1 + 2
    // mulr 1 1 1   r1 = r1 * r1
    // mulr 3 1 1   r1 = pc * r1
    // muli 1 11 1  r1 = r1 * 11
    // addi 2 4 2   r2 = r2 + 4
    // mulr 2 3 2   r2 = r2 * pc
    // addi 2 19 2  r2 = r2 + 19
    // addr 1 2 1   r1 = r1 + r2

    // If input was zero, go to label_start
    // addr 3 0 3   pc = pc + r0
    // seti 0 7 3   pc = 0

    // Else continue here
    // setr 3 2 2   r2 = pc + r2
    // mulr 2 3 2   r2 = r2 * pc
    // addr 3 2 2   r2 = pc + r2
    // mulr 3 2 2   r2 = pc * r2
    // muli 2 14 2  r2 = r2 * 14
    // mulr 2 3 2   r2 = r2 * pc
    // addr 1 2 1   r1 = r1 + r2
    // seti 0 1 0   r0 = 0

    uint32_t r1 = 0;
    uint32_t r2 = 0;
    uint32_t r4 = 0;
    uint32_t r5 = 0;
    if (r0 == 0)
    {
        r1 = 943;
        r2 = 107;
    }
    else
    {
        r0 = 0;
        r1 = 10551343;
        r2 = 10550400;
    }

    // Now go to label_start
    // seti 0 5 3   pc = 0

    // Here is label_start
    // seti 1 8 5   r5 = 1
    r5 = 1;

    // Here is label_loop1
    // while (true)
    // r1 is 943 or 10551343
    for (r5 = 1; r5 <= r1; r5++)
    {
        // seti 1 0 4   r4 = 1                    !!!!!   r4 = 1
        // r4 = 1;

        // Here is label_loop2
        // while (true)
        // r1 is 943 or 10551343
        for (r4 = 1; r4 <= r1; r4++)
        {
            // mulr 5 4 2   r2 = r5 * r4                        !!!!!   r2 = r5 * r4
            r2 = r5 * r4;

            // If r2 == r1, add r5 to r0
            // eqrr 2 1 2   r2 = (r2 == r1)                                r2 = (r2 == r1)
            // addr 2 3 3   pc = r2 + pc                                r3 = r2 + r3
            // addi 3 1 3   pc = pc + 1                        !!!!!   r3 = r3 + 1
            if (r2 == r1) // r1 is 943 or 10551343
            {
                r0 += r5;
            }

            // Here's that add
            // addr 5 0 0   r0 = r5 + r0                        <----   r0 = r0 + r5

            // Else continue here
            // addi 4 1 4   r4 = r4 + 1                        !!!!!   r4 = r4 + 1
            // r4 = r4 + 1;

            // If r4 > r1, then exit loop, else goto label_loop2
            // gtrr 4 1 2   r2 = (r4 > r1)                                r2 = (r4 > r1)
            // addr 3 2 3   pc = pc + r2                                r3 = r3 + r2
            // seti 2 3 3   pc = 2                        !!!!!   r3 = 2
            // if (r4 > r1) // r1 is 943 or 10551343
            // {
            //     break;
            // }
        }

        // Exited inner loop
        // addi 5 1 5   r5 = r5 + 1                    !!!!!   r5 = r5 + 1
        // r5 = r5 + 1;

        // If r5 > r1, exit outer loop, else goto label_loop1
        // gtrr 5 1 2   r2 = (r5 > r1)                    !!!!!   r2 = r5 > r1
        // addr 2 3 3   pc = pc + r2                    !!!!!   r3 = r2 + r3
        // seti 1 4 3   pc = 1
        // if (r5 > r1)
        // {
        //     break;
        // }
    }

    // Exited outer loop, end program
    // mulr 3 3 3   pc = pc * pc                    !!!!!   r3 = r3 * r3 == 256 (end program)
    return r0;
}

uint32_t TheCodeSimplified(uint32_t r0)
{
    uint32_t r1 = 0;
    uint32_t r2 = 0;
    if (r0 == 0)
    {
        r1 = 943;
        r2 = 107;
    }
    else
    {
        r0 = 0;
        r1 = 10551343;
        r2 = 10550400;
    }

    // r1 is 943 or 10551343
    for (uint32_t r5 = 1; r5 <= r1; r5++)
    {
        // r1 is 943 or 10551343
        for (uint32_t r4 = 1; r4 <= r1; r4++)
        {
            r2 = r5 * r4;

            if (r2 == r1) // r1 is 943 or 10551343
            {
                r0 += r5;
                printf("%d * %d == %d. r5:%d r0:%d\n", r5, r4, r2, r5, r0);
            }
        }
    }

    return r0;
}

uint32_t SumFactorsOfNumber(uint32_t number)
{
    uint32_t sum = 0;
    for (uint32_t i = 1; i <= number; i++)
    {
        if (number % i == 0)
        {
            sum += i;
        }
    }
    return sum;
}

int main()
{
    // printf("Result: %lld\n", Problem1(testData, ARRAY_SIZE(testData)));
    // printf("Result: %lld\n", Problem1(input, ARRAY_SIZE(input)));
    printf("The code result: %d\n", TheCode(0));
    printf("The code result: %d\n", TheCodeSimplified(0));
    printf("Sum of factors: %d\n", SumFactorsOfNumber(943));
    printf("Sum of factors: %d\n", SumFactorsOfNumber(10551343));
    // printf("The code result: %d\n", TheCodeSimplified(1));
    return 0;
}