// addi 3 16 3  pc = pc + 16                      ===== PC is R3
// seti 1 8 5   r5 = 1
// seti 1 0 4   r4 = 1                    !!!!!   r4 = 1
// mulr 5 4 2   r2 = r5 * r4                        !!!!!   r2 = r5 * r4
// eqrr 2 1 2   r2 = (r2 == r1)                                r2 = (r2 == r1)
// addr 2 3 3   pc = r2 + pc                                r3 = r2 + r3
// addi 3 1 3   pc = pc + 1                        !!!!!   r3 = r3 + 1
// addr 5 0 0   r0 = r5 + r0                        <----   r0 = r0 + r5
// addi 4 1 4   r4 = r4 + 1                        !!!!!   r4 = r4 + 1
// gtrr 4 1 2   r2 = (r4 > r1)                                r2 = (r4 > r1)
// addr 3 2 3   pc = pc + r2                                r3 = r3 + r2
// seti 2 3 3   pc = 2                        !!!!!   r3 = 2
// addi 5 1 5   r5 = r5 + 1                    !!!!!   r5 = r5 + 1
// gtrr 5 1 2   r2 = (r5 > r1)                    !!!!!   r2 = r5 > r1
// addr 2 3 3   pc = pc + r2                    !!!!!   r3 = r2 + r3
// seti 1 4 3   pc = 1
// mulr 3 3 3   pc = pc * pc                    !!!!!   r3 = r3 * r3 == 256 (end program)
// addi 1 2 1   r1 = r1 + 2
// mulr 1 1 1   r1 = r1 * r1
// mulr 3 1 1   r1 = pc * r1
// muli 1 11 1  r1 = r1 * 11
// addi 2 4 2   r2 = r2 + 4
// mulr 2 3 2   r2 = r2 * pc
// addi 2 19 2  r2 = r2 + 19
// addr 1 2 1   r1 = r1 + r2
// addr 3 0 3   pc = pc + r0
// seti 0 7 3   pc = 0
// setr 3 2 2   r2 = pc + r2
// mulr 2 3 2   r2 = r2 * pc
// addr 3 2 2   r2 = pc + r2
// mulr 3 2 2   r2 = pc * r2
// muli 2 14 2  r2 = r2 * 14
// mulr 2 3 2   r2 = r2 * pc
// addr 1 2 1   r1 = r1 + r2
// seti 0 1 0   r0 = 0
// seti 0 5 3   pc = 0

char* testData[] = {
    "#ip 0",
    "seti 5 0 1",
    "seti 6 0 2",
    "addi 0 1 0",
    "addr 1 2 3",
    "setr 1 0 0",
    "seti 8 0 4",
    "seti 9 0 5",
};

char* input[] = {
    "#ip 3",
    "addi 3 16 3",
    "seti 1 8 5",
    "seti 1 0 4",
    "mulr 5 4 2",
    "eqrr 2 1 2",
    "addr 2 3 3",
    "addi 3 1 3",
    "addr 5 0 0",
    "addi 4 1 4",
    "gtrr 4 1 2",
    "addr 3 2 3",
    "seti 2 3 3",
    "addi 5 1 5",
    "gtrr 5 1 2",
    "addr 2 3 3",
    "seti 1 4 3",
    "mulr 3 3 3",
    "addi 1 2 1",
    "mulr 1 1 1",
    "mulr 3 1 1",
    "muli 1 11 1",
    "addi 2 4 2",
    "mulr 2 3 2",
    "addi 2 19 2",
    "addr 1 2 1",
    "addr 3 0 3",
    "seti 0 7 3",
    "setr 3 2 2",
    "mulr 2 3 2",
    "addr 3 2 2",
    "mulr 3 2 2",
    "muli 2 14 2",
    "mulr 2 3 2",
    "addr 1 2 1",
    "seti 0 1 0",
    "seti 0 5 3",
};