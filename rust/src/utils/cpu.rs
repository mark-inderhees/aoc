use std::collections::VecDeque;

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
    pub program: VecDeque<Instruction>,
    pub state_history: Vec<State>,
    pub state: State,
    pub crt: Vec<char>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program: VecDeque::new(),
            state_history: Vec::new(),
            state: State { pc: 1, reg_x: 1 },
            crt: vec![' '; 240],
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.program.push_back(instruction.clone());
    }

    pub fn run(&mut self) {
        for instruction in self.program.clone().iter() {
            self.run_instruction(&instruction);
        }
    }

    fn get_cycle_count(instruction: &Instruction) -> u32 {
        match instruction {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }

    pub fn run_instruction(&mut self, instruction: &Instruction) {
        let mut pixel = match self.state.pc {
            pc if (self.state.reg_x - 1..=self.state.reg_x + 1)
                .contains(&((pc as i32 - 1) % 40)) =>
            {
                '#'
            }
            _ => '.',
        };
        let mut sprite = vec!['.'; 40];
        if self.state.reg_x > 0 && self.state.reg_x < 39 {
            let xu32 = self.state.reg_x as usize;
            sprite[xu32 - 1] = '#';
            sprite[xu32] = '#';
            sprite[xu32 + 1] = '#';
        }
        log::debug!(
            "Sprite position: {}\n",
            sprite.into_iter().collect::<String>()
        );
        log::debug!(
            "Start cycle {:03}: begin executing {:?}",
            self.state.pc,
            instruction
        );

        let count = Cpu::get_cycle_count(&instruction);

        // Save state
        log::debug!(
            "CPU: {} {} {:?}",
            self.state_history.len(),
            self.state.reg_x,
            instruction
        );
        log::debug!(
            "During cycle{:03}: CRT draws pixel in position {} {} {}",
            self.state.pc,
            self.state.pc - 1,
            pixel,
            self.state.reg_x,
        );
        self.crt[self.state.pc as usize - 1] = pixel;
        log::debug!(
            "Current CRT row: {:}",
            self.crt[0..40].into_iter().collect::<String>()
        );
        self.state_history.push(self.state);
        self.state.pc += 1;

        pixel = match self.state.pc {
            pc if (self.state.reg_x - 1..=self.state.reg_x + 1)
                .contains(&((pc as i32 - 1) % 40)) =>
            {
                '#'
            }
            _ => '.',
        };

        // Add current state for multi cycling instuctions
        for _ in 0..count - 1 {
            log::debug!(
                "CPU: {} {} {:?}",
                self.state_history.len(),
                self.state.reg_x,
                instruction
            );
            log::debug!(
                "During cycle{:03}: CRT draws pixel in position {} {} {}",
                self.state.pc,
                self.state.pc - 1,
                pixel,
                self.state.reg_x,
            );
            self.crt[self.state.pc as usize - 1] = pixel;
            log::debug!(
                "Current CRT row: {:}",
                self.crt[0..40].into_iter().collect::<String>()
            );
            self.state_history.push(self.state);
            self.state.pc += 1;
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
