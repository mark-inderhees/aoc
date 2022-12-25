/// A CPU that supports certain instructions, has cycle time count, and registers

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// Add an i32 to register X, takes 2 cycles
    Addx(i32),

    /// Do nothing, takes 1 cycle
    Noop,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    /// Time is how many cycles since the start of the program
    time: u32,

    /// CPU has a single signed register
    pub reg_x: i32,
}

pub struct Cpu {
    /// What was CPU state at each time cycle
    pub state_history: Vec<State>,

    state: State,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state_history: Vec::new(),

            // Init time and state to 1
            state: State { time: 1, reg_x: 1 },
        }
    }

    pub fn get_cycle_count(instruction: &Instruction) -> u32 {
        match instruction {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }

    fn step(&mut self) {
        self.state_history.push(self.state);
        self.state.time += 1;
    }

    pub fn get_reg_x(&self) -> i32 {
        self.state.reg_x
    }

    pub fn run_instruction(&mut self, instruction: &Instruction) {
        log::debug!(
            "Start cycle {:03}: begin executing {:?}",
            self.state.time,
            instruction
        );

        // Step processor state
        let count = Cpu::get_cycle_count(&instruction);
        for _ in 0..count {
            log::debug!(
                "CPU: {} {} {:?}",
                self.state_history.len(),
                self.state.reg_x,
                instruction
            );
            self.step();
        }

        // Perform operation
        match instruction {
            Instruction::Addx(i) => self.state.reg_x += i,
            Instruction::Noop => (),
        }

        log::debug!(
            "End of cycle{:03}: finish executing {:?} (Register X is now {})",
            self.state.time,
            instruction,
            self.state.reg_x
        );
    }
}
