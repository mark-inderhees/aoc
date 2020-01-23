#include "..\common.h"
#include "16input.h"

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

bool IsOpCode(opcodes opcode, uint32_t inputA, uint32_t inputB, uint32_t outputC, uint32_t registersBefore[4], uint32_t registersAfter[4])
{
    uint32_t expectedOutput;
    switch (opcode)
    {
        case addr:
        {
            expectedOutput = registersBefore[inputA] + registersBefore[inputB];
            break;
        }

        case addi:
        {
            expectedOutput = registersBefore[inputA] + inputB;
            break;
        }

        case mulr:
        {
            expectedOutput = registersBefore[inputA] * registersBefore[inputB];
            break;
        }

        case muli:
        {
            expectedOutput = registersBefore[inputA] * inputB;
            break;
        }

        case banr:
        {
            expectedOutput = registersBefore[inputA] & registersBefore[inputB];
            break;
        }

        case bani:
        {
            expectedOutput = registersBefore[inputA] & inputB;
            break;
        }

        case borr:
        {
            expectedOutput = registersBefore[inputA] | registersBefore[inputB];
            break;
        }

        case bori:
        {
            expectedOutput = registersBefore[inputA] | inputB;
            break;
        }

        case setr:
        {
            expectedOutput = registersBefore[inputA];
            break;
        }

        case seti:
        {
            expectedOutput = inputA;
            break;
        }

        case gtir:
        {
            expectedOutput = inputA > registersBefore[inputB] ? 1 : 0;
            break;
        }

        case gtri:
        {
            expectedOutput = registersBefore[inputA] > inputB ? 1 : 0;
            break;
        }

        case gtrr:
        {
            expectedOutput = registersBefore[inputA] > registersBefore[inputB] ? 1 : 0;
            break;
        }

        case eqir:
        {
            expectedOutput = inputA == registersBefore[inputB] ? 1 : 0;
            break;
        }

        case eqri:
        {
            expectedOutput = registersBefore[inputA] == inputB ? 1 : 0;
            break;
        }

        case eqrr:
        {
            expectedOutput = registersBefore[inputA] == registersBefore[inputB] ? 1 : 0;
            break;
        }


        default:
        {
            assert(false);
            return false;
        }
    }

    if (expectedOutput == registersAfter[outputC])
    {
        // printf("Matches opcode %d\n", opcode);
        return true;
    }

    return false;
}

uint32_t CountMatches(uint32_t inputA, uint32_t inputB, uint32_t outputC, uint32_t registersBefore[4], uint32_t registersAfter[4])
{
    uint32_t count = 0;
    for (opcodes i = opcodeMin; i < opcodeCount; i++)
    {
        if (IsOpCode(i, inputA, inputB, outputC, registersBefore, registersAfter))
        {
            count++;
        }
    }

    return count;
}

uint32_t Problem1(char* input[], uint32_t length)
{
    // Count the number of input that have 3 more opcodes
    // Input looks like:
    // "Before: [3, 2, 1, 1]",
    // "9 2 1 2",
    // "After:  [3, 2, 2, 1]",

    uint32_t countsOfThree = 0;
    uint32_t opcode;
    uint32_t inputA;
    uint32_t inputB;
    uint32_t outputC;
    uint32_t registersBefore[4];
    uint32_t registersAfter[4];
    for (uint32_t i = 0; i < length; i++)
    {
        registersBefore[0] = input[i][9]  - '0';
        registersBefore[1] = input[i][12] - '0';
        registersBefore[2] = input[i][15] - '0';
        registersBefore[3] = input[i][18] - '0';
        // printf("%d: Before: [%d, %d, %d, %d]\n", i, registersBefore[0], registersBefore[1], registersBefore[2], registersBefore[3]);
        i++;

        uint32_t opcodeLen = strlen(input[i]);
        uint32_t addMe = (opcodeLen == 7) ? 0 : 1;
        if (addMe == 0)
        {
            opcode = input[i][0] - '0';
        }
        else
        {
            opcode = input[i][0] - '0';
            opcode *= 10;
            opcode += input[i][1] - '0';
        }
        inputA  = input[i][2 + addMe] - '0';
        inputB  = input[i][4 + addMe] - '0';
        outputC = input[i][6 + addMe] - '0';
        // printf("%d: %d %d %d %d\n", i, opcode, inputA, inputB, outputC);
        i++;

        registersAfter[0] = input[i][9]  - '0';
        registersAfter[1] = input[i][12] - '0';
        registersAfter[2] = input[i][15] - '0';
        registersAfter[3] = input[i][18] - '0';
        // printf("%d: After: [%d, %d, %d, %d]\n", i, registersAfter[0], registersAfter[1], registersAfter[2], registersAfter[3]);
        if (CountMatches(inputA, inputB, outputC, registersBefore, registersAfter) >= 3)
        {
            countsOfThree++;
        }
        i++;
    }

    return countsOfThree;
}

int main()
{
    // Problem 1
    uint32_t answer1;
    answer1 = Problem1(testData1, ARRAY_SIZE(testData1));
    assert(answer1 == 1);
    answer1 = Problem1(input1, ARRAY_SIZE(input1));
    printf("Problem 1: %d\n", answer1);
    return 0;
}