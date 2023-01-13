/// A CPU that supports certain instructions, has cycle time count, and registers.
/// It can be expanded to support more instructions and registers.
/// And the cycle times for instructions could be configurable at creation.
pub struct Cpu {
    /// What was CPU state at each time cycle.
    pub state_history: Vec<State>,

    /// The current state.
    state: State,

    /// How long each instruction takes.
    instruction_cycle_count: Vec<u32>,

    /// The program to run.
    program: Vec<Instruction>,
}

/// Instructions that the CPU supports.
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Add an i32 to a register.
    Add(Register, i32),

    /// Do nothing.
    Noop,

    /// Half the value in a register.
    Half(Register),

    /// Triple the value in a register.
    Triple(Register),

    /// Increment the value of a register by 1.
    Increment(Register),

    /// Jump to an offset in the program, adds value to PC.
    Jump(isize),

    /// Jump to an offset if the register is even.
    JumpIfEven(Register, isize),

    /// Jump to an offset if the register is one.
    JumpIfOne(Register, isize),
}

impl Instruction {
    /// Convert instruction into an index to look into instruction_cycle_count list
    pub fn index(&self) -> usize {
        match self {
            Instruction::Add(_, _) => 0,
            Instruction::Noop => 1,
            Instruction::Half(_) => 2,
            Instruction::Triple(_) => 3,
            Instruction::Increment(_) => 4,
            Instruction::Jump(_) => 5,
            Instruction::JumpIfEven(_, _) => 6,
            Instruction::JumpIfOne(_, _) => 7,
        }
    }
}

/// The available registers.
#[derive(Debug, Clone)]
pub enum Register {
    X = 0,
    A,
    B,
}

/// State of the CPU at a given time.
#[derive(Debug, Clone)]
pub struct State {
    /// Time is how many cycles since the start of the program.
    time: u32,

    /// CPU registers.
    pub registers: Vec<i32>,

    /// The program counter.
    pc: usize,
}

impl Cpu {
    /// Create a new CPU. Time and regs are init to 1.
    pub fn new() -> Cpu {
        let cpu = Cpu {
            state_history: Vec::new(),

            // Init time and register states
            state: State {
                time: 1,
                registers: vec![
                    1, // x
                    0, // a
                    0, // b
                ],
                pc: 0,
            },

            // Each instruction must have a cycle count
            instruction_cycle_count: vec![
                2, // Add
                1, // Noop
                1, // Half
                1, // Triple
                1, // Increment
                1, // Jump
                1, // JumpIfEven
                1, // JumpIfOdd
            ],

            program: vec![],
        };
        cpu
    }

    /// How many cycles an instruction takes.
    pub fn cycle_count(&self, instruction: &Instruction) -> u32 {
        self.instruction_cycle_count[instruction.index()]
    }

    /// Increate time by one and save state to history.
    fn step(&mut self) {
        self.state_history.push(self.state.clone());
        self.state.time += 1;
    }

    /// Get the value of a register.
    pub fn reg(&self, register: Register) -> i32 {
        self.state.registers[register as usize]
    }

    /// Set the value of a register.
    pub fn set_reg(&mut self, register: Register, value: i32) {
        self.state.registers[register as usize] = value;
    }

    /// Add a new instruction to the program.
    pub fn add_instruction(&mut self, instruction: &Instruction) {
        self.program.push(instruction.clone());
    }

    /// Run a program.
    pub fn run(&mut self) {
        while self.state.pc < self.program.len() {
            let instruction = self.program[self.state.pc].clone();
            self.run_instruction(&instruction);
            self.state.pc += 1;
        }
    }

    /// Run an instruction.
    pub fn run_instruction(&mut self, instruction: &Instruction) {
        log::debug!(
            "\nStart cycle {:03}: begin executing {:?}",
            self.state.time,
            instruction
        );

        // Step processor state
        let count = self.cycle_count(&instruction);
        for _ in 0..count {
            log::debug!(
                "CPU: {} {:?} {:?}",
                self.state_history.len(),
                self.state.registers,
                instruction
            );
            self.step();
        }

        // Perform operation
        match instruction {
            Instruction::Add(r, i) => self.state.registers[r.clone() as usize] += i,
            Instruction::Noop => (),
            Instruction::Half(r) => self.state.registers[r.clone() as usize] /= 2,
            Instruction::Triple(r) => self.state.registers[r.clone() as usize] *= 3,
            Instruction::Increment(r) => self.state.registers[r.clone() as usize] += 1,
            Instruction::Jump(o) => self.offset_pc(o),
            Instruction::JumpIfEven(r, o) => {
                if self.state.registers[r.clone() as usize] % 2 == 0 {
                    self.offset_pc(o);
                }
            }
            Instruction::JumpIfOne(r, o) => {
                if self.state.registers[r.clone() as usize] == 1 {
                    self.offset_pc(o);
                }
            }
        }

        log::debug!(
            "End of cycle{:03}: finish executing {:?} (Registers are now {:?})",
            self.state.time,
            instruction,
            self.state.registers,
        );
    }

    /// Change the value of PC by a value
    fn offset_pc(&mut self, offset: &isize) {
        // Offset one less than the value as the PC will be auto incremented by 1
        self.state.pc = (self.state.pc as isize + offset - 1) as usize;
    }
}
