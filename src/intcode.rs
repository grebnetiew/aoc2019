pub struct Computer {
    memory: Vec<isize>,
    procnt: isize,
    halted: bool,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl Computer {
    pub fn new(program_data: Vec<isize>, input: Vec<isize>) -> Self {
        Self {
            memory: program_data,
            procnt: 0,
            halted: false,
            input: input,
            output: Vec::new(),
        }
    }
    fn one_step(&mut self) {
        let (mask, opcode) = self.instruction();
        match opcode {
            1 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = p[0] + p[1];
                self.procnt += 4;
            }
            2 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = p[0] * p[1];
                self.procnt += 4;
            }
            3 => {
                let replacement_pos = self.memory[self.procnt as usize + 1]; // never immediate mode
                self.memory[replacement_pos as usize] =
                    self.input.pop().expect("Input was taken but none is left");
                self.procnt += 2;
            }
            4 => {
                let p = self.params(1, mask);
                self.output.push(p[0]);
                self.procnt += 2;
            }
            5 => {
                let p = self.params(2, mask);
                if p[0] != 0 {
                    self.procnt = p[1];
                } else {
                    self.procnt += 3;
                }
            }
            6 => {
                let p = self.params(2, mask);
                if p[0] == 0 {
                    self.procnt = p[1];
                } else {
                    self.procnt += 3;
                }
            }
            7 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = if p[0] < p[1] { 1 } else { 0 };
                self.procnt += 4;
            }
            8 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = if p[0] == p[1] { 1 } else { 0 };
                self.procnt += 4;
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
        while (!self.halted) && self.output.len() == 0 {
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
                _ => Mode::Immediate,
            });
            digits /= 10;
        }
        (Mask(mask), opcode)
    }

    fn read(&self, operand: isize, mode: &Mode) -> isize {
        match &mode {
            Mode::Immediate => operand,
            Mode::Position => self.memory[operand as usize],
        }
    }

    fn params(&self, amount: usize, mask: Mask) -> Vec<isize> {
        (0..amount)
            .map(|i| self.read(self.memory[self.procnt as usize + i + 1], mask.get(i)))
            .collect()
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
}
