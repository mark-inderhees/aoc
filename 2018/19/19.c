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
    uint32_t inputA;
    uint32_t inputB;
    uint32_t outputC;
} instruction;

uint32_t ProcessOpcode(opcodes opcode, uint32_t inputA, uint32_t inputB, uint32_t outputC, uint32_t registersBefore[4])
{
    (void)(outputC);

    uint32_t output;
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

uint32_t Problem1(char* input[], uint32_t length)
{
    // Parse input
    // Get program counter
    uint32_t programCounterRegister = input[0][4] - '0';

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

    printf("#ip %d\n", programCounterRegister);
    for (uint32_t i = 0; i < length - 1; i++)
    {
        printf("%s %d %d %d\n",
            OPCODE_NAMES[instructions[i].opcode],
            instructions[i].inputA,
            instructions[i].inputB,
            instructions[i].outputC);
    }

    return 0;
}

int main()
{
    printf("Result: %d\n", Problem1(testData, ARRAY_SIZE(testData)));
    return 0;
}