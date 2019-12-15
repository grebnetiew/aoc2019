#[derive(Debug, Default)]
pub struct Computer {
    memory: Vec<i64>,
    procnt: i64,
    relbse: i64,
    halted: bool,
    input: Vec<i64>,
    output: Vec<i64>,
}

impl From<Vec<i64>> for Computer {
    // Initialize a computer using the vector as initial memory
    fn from(memory: Vec<i64>) -> Self {
        Self::new(memory, Default::default())
    }
}

impl Computer {
    // Create a new Computer using the given memory (the program to run)
    // and an input vector.
    pub fn new(memory: Vec<i64>, input: Vec<i64>) -> Self {
        Self {
            memory,
            input,
            ..Default::default()
        }
    }

    // Various modes of running

    // Run until halted, no chance to supply more input.
    // Output is provided as a Vec at the end.
    pub fn run(&mut self) -> &Vec<i64> {
        while !self.halted {
            self.one_step();
        }
        &self.output
    }

    // Run until there is one output, and return it.
    // Returns None if the program halts.
    pub fn run_until_output(&mut self) -> Option<i64> {
        while (!self.halted) && self.output.is_empty() {
            self.one_step();
        }
        self.output.pop()
    }

    // Run until there is one output, and return it.
    // Supply a function that is called whenever input is needed.
    // This clears the internal input buffer.
    pub fn run_until_output_with<F>(&mut self, f: F) -> Option<i64>
    where
        F: Fn() -> i64,
    {
        self.input.clear();
        while (!self.halted) && self.output.is_empty() {
            if self.opcode() == 3 {
                self.more_input(f());
            }
            self.one_step();
        }
        self.output.pop()
    }

    // Gives the first value in memory. Needed for an early puzzle.
    pub fn mem_first(&self) -> i64 {
        self.memory[0]
    }

    // Supplies more input to be added to the internal buffer.
    pub fn more_input(&mut self, i: i64) {
        self.input.insert(0, i)
    }

    // Process one step starting from current program counter
    fn one_step(&mut self) {
        match self.opcode() {
            1 => self.bin_op(|a, b| a + b),
            2 => self.bin_op(|a, b| a * b),
            3 => self.get_input(),
            4 => self.give_output(),
            5 => self.jmp_if(|a| a != 0),
            6 => self.jmp_if(|a| a == 0),
            7 => self.bin_op(|a, b| if a < b { 1 } else { 0 }),
            8 => self.bin_op(|a, b| if a == b { 1 } else { 0 }),
            9 => self.set_relbase(),
            99 => self.halted = true,
            n => panic!("Unknown opcode {}", n),
        }
    }

    // Information about the current instruction

    // Returns the opcode of the current instruction (removes the mask)
    fn opcode(&self) -> Opcode {
        self.memory[self.procnt as usize] % 100
    }

    // Returns the mask of the current instruction, in parameter order
    fn mask(&self) -> Mask {
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
        Mask(mask)
    }

    // Low-level reading and writing functionality

    // Read one value in the selected mode. Operand is the address or
    // value to be read.
    fn read(&self, operand: i64, mode: Mode) -> i64 {
        match &mode {
            Mode::Immediate => operand,
            Mode::Position => *self.memory.get(operand as usize).unwrap_or(&0),
            Mode::Relative => *self
                .memory
                .get((self.relbse + operand) as usize)
                .unwrap_or(&0),
        }
    }

    // Extract the parameters of a function with <amount> parameters
    fn params(&self, amount: usize) -> Vec<i64> {
        let mask = self.mask();
        (0..amount)
            .map(|i| self.read(self.memory[self.procnt as usize + i + 1], *mask.get(i)))
            .collect()
    }

    // Write one value in the specified mode to the address given by the
    // parameter found <offset> from the current program counter. Immediate
    // mode can not be used here, and will cause a panic if supplied.
    fn write(&mut self, offset: usize, value: i64) {
        // Determine the write address. These are counted from the start in
        // Position mode, and from the relative base in Relative mode.
        let mut replacement_pos = match self.mask().get(offset - 1) {
            Mode::Immediate => panic!(
                "Attempted to write {} in immediate mode at PC = {}",
                value, self.procnt
            ),
            Mode::Position => 0,
            Mode::Relative => self.relbse,
        };
        // Add the value in the 'destination' memory slot
        replacement_pos += self.memory[self.procnt as usize + offset];

        // Ensure we have enough memory
        if self.memory.len() <= replacement_pos as usize {
            self.memory
                .resize_with(replacement_pos as usize + 1, Default::default);
        }
        self.memory[replacement_pos as usize] = value;
    }

    // Operators supported by the VM

    // Standard binary operator. The function supplied is the operation to
    // be performed.
    fn bin_op<F>(&mut self, f: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let p = self.params(2);
        self.write(3, f(p[0], p[1]));
        self.procnt += 4;
    }

    // Standard conditional jump with one parameter. The function supplied
    // is used to decide whether to jump.
    fn jmp_if<F>(&mut self, f: F)
    where
        F: Fn(i64) -> bool,
    {
        let p = self.params(2);
        if f(p[0]) {
            self.procnt = p[1];
        } else {
            self.procnt += 3;
        }
    }

    // Take input from the buffer and put it in the location specified by
    // the only parameter
    fn get_input(&mut self) {
        let value = self.input.pop().expect("Input was taken but none is left");
        self.write(1, value);
        self.procnt += 2;
    }

    // Add the value of the only parameter to the output buffer
    fn give_output(&mut self) {
        let p = self.params(1);
        self.output.push(p[0]);
        self.procnt += 2;
    }

    // Adjust the relative base value by the only parameter
    fn set_relbase(&mut self) {
        let p = self.params(1);
        self.relbse += p[0];
        self.procnt += 2;
    }
}

type Opcode = i64;
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
