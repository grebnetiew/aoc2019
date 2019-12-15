#[derive(Debug, Default)]
pub struct Debugger {
    memory: Vec<i64>,
    cursor: usize,
}

impl From<Vec<i64>> for Debugger {
    // Initialize a computer using the vector as initial memory
    fn from(memory: Vec<i64>) -> Self {
        Self::new(memory)
    }
}

impl Debugger {
    // Create a new Computer using the given memory (the program to run)
    // and an input vector.
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            ..Default::default()
        }
    }

    pub fn assembly(&mut self) -> String {
        let mut result = String::new();
        // Print code segment
        while self.cursor < self.memory.len() {
            let delta = self.op_length();
            result += &format!("{:4}: ", self.cursor);
            let optext = self.op_to_string();
            if optext.starts_with("data") {
                // Data segment
                break;
            }
            result += &optext;
            self.cursor += delta;
        }
        // Print data segment

        while self.cursor < self.memory.len() {
            result += &format!("{:7} ", self.memory[self.cursor]);
            self.cursor += 1;
            if self.cursor % 8 == 0 {
                result += &format!("\n{:4}: ", self.cursor);
            }
        }
        result + "\n"
    }

    fn op_length(&self) -> usize {
        match self.opcode() {
            1 | 2 | 7 | 8 => 4,
            3 | 4 | 9 => 2,
            5 | 6 => 3,
            _ => 1,
        }
    }

    fn op_to_string(&mut self) -> String {
        (match self.opcode() {
            1 => self.bin_op("+"),
            2 => self.bin_op("*"),
            3 => self.get_input(),
            4 => self.give_output(),
            5 => self.jmp_if("!="),
            6 => self.jmp_if("=="),
            7 => self.bin_op("<"),
            8 => self.bin_op("=="),
            9 => self.relative_base_increment(),
            99 => "halt".to_owned(),
            n => format!("data {}", n),
        }) + "\n"
    }

    // Information about the current instruction

    // Returns the opcode of the current instruction (removes the mask)
    fn opcode(&self) -> Opcode {
        self.memory[self.cursor] % 100
    }

    // Returns the mask of the current instruction, in parameter order
    fn mask(&self) -> Mask {
        Mask::new(self.memory[self.cursor])
    }

    fn param_to_string(&self, offset: usize) -> String {
        self.param_to_string_mode(offset, *self.mask().get(offset))
    }

    fn param_to_string_mode(&self, offset: usize, mode: Mode) -> String {
        match mode {
            Mode::Immediate => return format!("{}", self.memory[self.cursor + offset + 1]),
            Mode::Position => return format!("*{}", self.memory[self.cursor + offset + 1]),
            Mode::Relative => return format!("rb[{}]", self.memory[self.cursor + offset + 1]),
        }
    }

    fn param(&self, offset: usize) -> i64 {
        self.memory[self.cursor + offset + 1]
    }

    // Operators supported by the VM

    // Standard binary operator. The function supplied is the operation to
    // be performed.
    fn bin_op(&mut self, op: &str) -> String {
        if op == "+" && self.param(0) == 0 && *self.mask().get(0) == Mode::Immediate {
            return format!("{} := {}", self.param_to_string(2), self.param_to_string(1));
        }
        if op == "+" && self.param(1) == 0 && *self.mask().get(1) == Mode::Immediate {
            return format!("{} := {}", self.param_to_string(2), self.param_to_string(0));
        }
        if op == "*" && self.param(0) == 1 && *self.mask().get(0) == Mode::Immediate {
            return format!("{} := {}", self.param_to_string(2), self.param_to_string(1));
        }
        if op == "*" && self.param(1) == 1 && *self.mask().get(1) == Mode::Immediate {
            return format!("{} := {}", self.param_to_string(2), self.param_to_string(0));
        }
        if op == "=="
            && (self.memory[self.cursor + 4] % 100 == 5 || self.memory[self.cursor + 4] % 100 == 6)
        {
            // Possible inline of comparison-then-if
            let next_mask = Mask::new(self.memory[self.cursor + 4]);
            // If the destination of this == statement is also the test of the jump, replace
            if self.memory[self.cursor + 3] == self.memory[self.cursor + 5]
                && *self.mask().get(2) == *next_mask.get(0)
            {
                let result = format!(
                    "{} := {} {} {}\n{:4}: if {} {} {} {{ goto {} }}",
                    self.param_to_string(2),
                    self.param_to_string(0),
                    op,
                    self.param_to_string(1),
                    self.cursor + 4,
                    self.param_to_string(0),
                    if self.memory[self.cursor + 4] % 100 == 5 {
                        "=="
                    } else {
                        "!="
                    },
                    self.param_to_string(1),
                    self.param_to_string_mode(5, *next_mask.get(1)),
                );
                self.cursor += 3; // skip the jmp
                return result;
            }
        }
        format!(
            "{} := {} {} {}",
            self.param_to_string(2),
            self.param_to_string(0),
            op,
            self.param_to_string(1),
        )
    }

    // Standard conditional jump with one parameter. The function supplied
    // is used to decide whether to jump.
    fn jmp_if(&self, op: &str) -> String {
        if op == "!=" && self.param(0) == 1 && *self.mask().get(1) == Mode::Immediate {
            return format!("goto {}", self.param_to_string(1));
        }
        if op == "==" && self.param(0) == 0 && *self.mask().get(1) == Mode::Immediate {
            return format!("goto {}", self.param_to_string(1));
        }
        format!(
            "if {} {} 0 {{ goto {} }}",
            self.param_to_string(0),
            op,
            self.param_to_string(1)
        )
    }

    fn get_input(&self) -> String {
        format!("{} := input()", self.param_to_string(0))
    }

    fn give_output(&self) -> String {
        format!("output({})", self.param_to_string(0))
    }

    fn relative_base_increment(&self) -> String {
        format!("rb += {}", self.param_to_string(0))
    }
}

type Opcode = i64;
struct Mask(Vec<Mode>);

impl Mask {
    fn new(opcode: i64) -> Self {
        let mut mask = Vec::new();
        let mut digits = opcode / 100;
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

    fn get(&self, idx: usize) -> &Mode {
        self.0.get(idx).unwrap_or(&Mode::Position)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Immediate,
    Position,
    Relative,
}
