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
    for (uint32_t instructionPointer = 0; instructionPointer < length - 1; instructionPointer++)
    {
        instructionCount++;
        instructionCountSinceLastJump++;
        uint32_t tempIp = instructionPointer;
        registers[programCounterRegister] = instructionPointer;
        // printf("ip=%d [%lld, %lld, %lld, %lld, %lld, %lld] \t%s %lld %lld %lld",
        //     instructionPointer,
        //     registers[0], registers[1], registers[2], registers[3], registers[4], registers[5],
        //     OPCODE_NAMES[instructions[instructionPointer].opcode],
        //     instructions[instructionPointer].inputA,
        //     instructions[instructionPointer].inputB,
        //     instructions[instructionPointer].outputC);

        if (instructionPointer == 4)
        {
            printf("%lld == %lld ?\n", registers[2], registers[1]);
        }
        else if (instructionPointer == 9)
        {
            printf("%lld > %lld ?\n", registers[4], registers[1]);
        }

        result = ProcessOpcode(
            instructions[instructionPointer].opcode,
            instructions[instructionPointer].inputA,
            instructions[instructionPointer].inputB,
            instructions[instructionPointer].outputC,
            registers);
        registers[instructions[instructionPointer].outputC] = result;
        // printf(" [%lld, %lld, %lld, %lld, %lld, %lld]\n",
        //     registers[0], registers[1], registers[2], registers[3], registers[4], registers[5]);
        instructionPointer = registers[programCounterRegister];
        if (tempIp != instructionPointer)
        {
            jumpCount++;
            // printf("Jump from %d to %d \t[jc:%d ic:%d iclj:%d]\n", tempIp, instructionPointer + 1,
            //     jumpCount, instructionCount, instructionCountSinceLastJump);
            instructionCountSinceLastJump = 0;
        }
    }

    printf("[jc:%d ic:%d iclj:%d]\n",
        jumpCount, instructionCount, instructionCountSinceLastJump);

    return registers[0];
}

int main()
{
    // printf("Result: %lld\n", Problem1(testData, ARRAY_SIZE(testData)));
    printf("Result: %lld\n", Problem1(input, ARRAY_SIZE(input)));
    return 0;
}