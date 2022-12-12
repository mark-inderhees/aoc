#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pc: u32,
    pub reg_x: i32,
}

pub struct Cpu {
    pub state_history: Vec<State>,
    state: State,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state_history: Vec::new(),
            state: State { pc: 1, reg_x: 1 },
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
        self.state.pc += 1;
    }

    pub fn get_reg_x(&self) -> i32 {
        self.state.reg_x
    }

    pub fn run_instruction(&mut self, instruction: &Instruction) {
        log::debug!(
            "Start cycle {:03}: begin executing {:?}",
            self.state.pc,
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
            self.state.pc,
            instruction,
            self.state.reg_x
        );
    }
}
