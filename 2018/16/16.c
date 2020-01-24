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

uint8_t _opcodeMatches[opcodeCount][opcodeCount] = {0};
opcodes _opcodeLookup[opcodeCount] = {0};
uint32_t _opcodeInstances[opcodeCount] = {0};
bool _opcodeInstancesProcessed[opcodeCount] = {0};

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

bool IsOpCode(opcodes opcode, uint32_t inputA, uint32_t inputB, uint32_t outputC, uint32_t registersBefore[4], uint32_t registersAfter[4])
{
    uint32_t expectedOutput = ProcessOpcode(opcode, inputA, inputB, outputC, registersBefore);
    if (expectedOutput == registersAfter[outputC])
    {
        // printf("Matches opcode %d\n", opcode);
        return true;
    }

    return false;
}

uint32_t CountMatches(uint32_t inputOpcode, uint32_t inputA, uint32_t inputB, uint32_t outputC, uint32_t registersBefore[4], uint32_t registersAfter[4])
{
    uint32_t count = 0;
    for (opcodes i = opcodeMin; i < opcodeCount; i++)
    {
        if (IsOpCode(i, inputA, inputB, outputC, registersBefore, registersAfter))
        {
            count++;
        }
        else
        {
            _opcodeMatches[inputOpcode][i] = 0;
        }
    }

    return count;
}

// The opcode to keep must only exist onces in _opcodeMatches
void ClearAllOtherOpcodes(opcodes opcodeToKeep)
{
    // Find the input opcode that has this opcode.
    bool foundOpcode = false;
    for (uint32_t inputOpcodeI = 0; inputOpcodeI < opcodeCount; inputOpcodeI++)
    {
        if (_opcodeMatches[inputOpcodeI][opcodeToKeep] == 1)
        {
            printf("For input %02d, keeping just %02d %s\n", inputOpcodeI, opcodeToKeep, OPCODE_NAMES[opcodeToKeep]);
            assert(!foundOpcode);
            foundOpcode = true;

            // Remove all other opcodes from this input opcode.
            for (uint32_t opcodeI = 0; opcodeI < opcodeCount; opcodeI++)
            {
                if (opcodeI != opcodeToKeep)
                {
                    _opcodeMatches[inputOpcodeI][opcodeI] = 0;
                }
            }
        }
    }

    assert(foundOpcode);
}

void CalculateOpcodeInstances()
{
    memset(_opcodeInstances, 0, sizeof(_opcodeInstances));
    for (uint32_t inputOpcodeI = 0; inputOpcodeI < opcodeCount; inputOpcodeI++)
    {
        for (uint32_t opcodeI = 0; opcodeI < opcodeCount; opcodeI++)
        {
            if (_opcodeMatches[inputOpcodeI][opcodeI] == 1)
            {
                _opcodeInstances[opcodeI]++;
            }
        }
    }
}

uint32_t Problem2(char* input[], uint32_t length, uint32_t program[], uint32_t programLength)
{
    // Count the number of input that have 3 more opcodes
    // Input looks like:
    // "Before: [3, 2, 1, 1]",
    // "9 2 1 2",
    // "After:  [3, 2, 2, 1]",

    // Clear out OpcodeMatches (everything starts at true)
    memset(_opcodeMatches, 1, sizeof(_opcodeMatches));

    uint32_t countsOfThree = 0;
    uint32_t inputOpcode;
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
            inputOpcode = input[i][0] - '0';
        }
        else
        {
            inputOpcode = input[i][0] - '0';
            inputOpcode *= 10;
            inputOpcode += input[i][1] - '0';
        }
        inputA  = input[i][2 + addMe] - '0';
        inputB  = input[i][4 + addMe] - '0';
        outputC = input[i][6 + addMe] - '0';
        // printf("%d: %d %d %d %d\n", i, inputOpcode, inputA, inputB, outputC);
        i++;

        registersAfter[0] = input[i][9]  - '0';
        registersAfter[1] = input[i][12] - '0';
        registersAfter[2] = input[i][15] - '0';
        registersAfter[3] = input[i][18] - '0';
        // printf("%d: After: [%d, %d, %d, %d]\n", i, registersAfter[0], registersAfter[1], registersAfter[2], registersAfter[3]);
        if (CountMatches(inputOpcode, inputA, inputB, outputC, registersBefore, registersAfter) >= 3)
        {
            countsOfThree++;
        }
        i++;
    }

    // return countsOfThree;

    // Start of problem 2 logic
    // Need to find input opcodes that exactly match one opcode.
    // Remove all other opcodes from that input opcode.
    // Then repeat from the beginning.
    CalculateOpcodeInstances();
    uint32_t opcodeI = 0;
    while (opcodeI < opcodeCount)
    {
        if (_opcodeInstances[opcodeI] == 1 && !_opcodeInstancesProcessed[opcodeI])
        {
            _opcodeInstancesProcessed[opcodeI] = true;
            ClearAllOtherOpcodes(opcodeI);
            CalculateOpcodeInstances();
            opcodeI = 0;
            continue;
        }
        opcodeI++;
    }

    for (uint32_t inputOpcodeI = 0; inputOpcodeI < opcodeCount; inputOpcodeI++)
    {
        uint32_t someCount = 0;
        for (uint32_t opcodeI = 0; opcodeI < opcodeCount; opcodeI++)
        {
            if (_opcodeMatches[inputOpcodeI][opcodeI] == 1)
            {
                _opcodeLookup[inputOpcodeI] = opcodeI;
                someCount++;
                printf("Input opcode %d = %s\n", inputOpcodeI, OPCODE_NAMES[opcodeI]);
            }
        }
        assert(someCount == 1);
    }

    // Now process the program!
    uint32_t registers[4] = {0};
    uint32_t result = 0;
    opcodes opcode;
    // printf("r0:%d r1:%d r2:%d r3:%d\n", registers[0], registers[1], registers[2], registers[3]);
    for (uint32_t i = 0; i < programLength; i++)
    {
        inputOpcode = program[i++];
        opcode = _opcodeLookup[inputOpcode];
        inputA = program[i++];
        inputB = program[i++];
        outputC = program[i];
        result = ProcessOpcode(opcode, inputA, inputB, outputC, registers);
        registers[outputC] = result;
        // printf("r0:%d r1:%d r2:%d r3:%d\n", registers[0], registers[1], registers[2], registers[3]);
    }

    return registers[0];
}

int main()
{
    // Problem 1
    // uint32_t answer1;
    // answer1 = Problem1(testData1, ARRAY_SIZE(testData1));
    // assert(answer1 == 1);
    // answer1 = Problem1(input1, ARRAY_SIZE(input1));
    // printf("Problem 1: %d\n", answer1);

    // Problem 2
    uint32_t answer2;
    answer2 = Problem2(input1, ARRAY_SIZE(input1), input2, ARRAY_SIZE(input2));
    printf("Problem 2: %d\n", answer2);
    return 0;
}