use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub reg_x: i32,
}

pub struct Cpu {
    program: VecDeque<Instruction>,
    pub state_history: Vec<State>,
    state: State,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program: VecDeque::new(),
            state_history: Vec::new(),
            state: State { reg_x: 1 },
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.program.push_back(instruction.clone());
    }

    pub fn run(&mut self) {
        for instruction in self.program.clone().iter(){
            self.step(&instruction);
        }
    }

    fn get_cycle_count(instruction: &Instruction) -> u32 {
        match instruction {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }

    fn step(&mut self, instruction: &Instruction) {
        let count = Cpu::get_cycle_count(&instruction);

        // Save state
        log::debug!("CPU: {} {} {:?}", self.state_history.len(), self.state.reg_x, instruction);
        self.state_history.push(self.state);

        // Add current state for multi cycling instuctions
        for _ in 0..count - 1 {
            log::debug!("CPU: {} {} {:?}", self.state_history.len(), self.state.reg_x, instruction);
            self.state_history.push(self.state)
        }

        // Perform operation
        match instruction {
            Instruction::Addx(i) => self.state.reg_x += i,
            Instruction::Noop => (),
        }


    }
}
