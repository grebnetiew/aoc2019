pub struct Computer {
    memory: Vec<isize>,
    procnt: isize,
    relbse: isize,
    halted: bool,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl Computer {
    pub fn new(program_data: Vec<isize>, input: Vec<isize>) -> Self {
        Self {
            memory: program_data,
            procnt: 0,
            relbse: 0,
            halted: false,
            input,
            output: Vec::new(),
        }
    }
    fn one_step(&mut self) {
        let (mask, opcode) = self.instruction();
        match opcode {
            1 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, &mask);
                self.write(replacement_pos, *mask.get(2), p[0] + p[1]);
                self.procnt += 4;
            }
            2 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, &mask);
                self.write(replacement_pos, *mask.get(2), p[0] * p[1]);
                self.procnt += 4;
            }
            3 => {
                let replacement_pos = self.memory[self.procnt as usize + 1]; // never immediate mode
                let value = self.input.pop().expect("Input was taken but none is left");
                self.write(replacement_pos, *mask.get(0), value);
                self.procnt += 2;
            }
            4 => {
                let p = self.params(1, &mask);
                self.output.push(p[0]);
                self.procnt += 2;
            }
            5 => {
                let p = self.params(2, &mask);
                if p[0] != 0 {
                    self.procnt = p[1];
                } else {
                    self.procnt += 3;
                }
            }
            6 => {
                let p = self.params(2, &mask);
                if p[0] == 0 {
                    self.procnt = p[1];
                } else {
                    self.procnt += 3;
                }
            }
            7 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, &mask);
                self.write(
                    replacement_pos,
                    *mask.get(2),
                    if p[0] < p[1] { 1 } else { 0 },
                );
                self.procnt += 4;
            }
            8 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, &mask);
                self.write(
                    replacement_pos,
                    *mask.get(2),
                    if p[0] == p[1] { 1 } else { 0 },
                );
                self.procnt += 4;
            }
            9 => {
                let p = self.params(1, &mask);
                self.relbse += p[0];
                self.procnt += 2;
            }
            99 => self.halted = true,
            n => panic!("Unknown opcode {}", n),
        }
    }

    pub fn run(&mut self) -> &Vec<isize> {
        while !self.halted {
            self.one_step();
        }
        &self.output
    }

    pub fn run_until_output(&mut self) -> Option<isize> {
        while (!self.halted) && self.output.is_empty() {
            self.one_step();
        }
        self.output.pop()
    }

    pub fn run_until_output_with<F>(&mut self, f: F) -> Option<isize>
    where
        F: Fn() -> isize,
    {
        self.input.clear();
        while (!self.halted) && self.output.is_empty() {
            if self.instruction().1 == 3 {
                self.more_input(f());
            }
            self.one_step();
        }
        self.output.pop()
    }

    fn instruction(&self) -> (Mask, Opcode) {
        let opcode = self.memory[self.procnt as usize] % 100;
        let mut mask = Vec::new();
        let mut digits = self.memory[self.procnt as usize] / 100;
        while digits > 0 {
            mask.push(match digits % 10 {
                0 => Mode::Position,
                1 => Mode::Immediate,
                _ => Mode::Relative,
            });
            digits /= 10;
        }
        (Mask(mask), opcode)
    }

    fn read(&self, operand: isize, mode: Mode) -> isize {
        match &mode {
            Mode::Immediate => operand,
            Mode::Position => *self.memory.get(operand as usize).unwrap_or(&0),
            Mode::Relative => *self
                .memory
                .get((self.relbse + operand) as usize)
                .unwrap_or(&0),
        }
    }

    fn params(&self, amount: usize, mask: &Mask) -> Vec<isize> {
        (0..amount)
            .map(|i| self.read(self.memory[self.procnt as usize + i + 1], *mask.get(i)))
            .collect()
    }

    fn write(&mut self, mut addr: isize, mode: Mode, value: isize) {
        match &mode {
            Mode::Immediate => panic!(
                "Attempted to write {} to an immediate mode address {}",
                value, addr
            ),
            Mode::Position => {}
            Mode::Relative => addr += self.relbse,
        }
        if self.memory.len() <= addr as usize {
            self.memory.resize_with(addr as usize + 1, Default::default);
        }
        self.memory[addr as usize] = value;
    }

    pub fn mem_first(&self) -> isize {
        self.memory[0]
    }

    pub fn more_input(&mut self, i: isize) {
        self.input.insert(0, i)
    }
}

impl From<Vec<isize>> for Computer {
    fn from(v: Vec<isize>) -> Self {
        Computer::new(v, Vec::new())
    }
}

type Opcode = isize;
struct Mask(Vec<Mode>);

impl Mask {
    fn get(&self, idx: usize) -> &Mode {
        self.0.get(idx).unwrap_or(&Mode::Position)
    }
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Immediate,
    Position,
    Relative,
}
